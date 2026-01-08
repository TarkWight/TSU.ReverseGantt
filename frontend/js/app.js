const app = {
    currentProjectId: null,
    currentTaskId: null,
    currentUser: null,
    globalRole: null,
    userName: null,
    allProjects: [],
    currentProjectMembers: null,
    currentProjectTasks: null,

    init() {
        this.initTemplates();
        this.initEventListeners();
        auth.init();
    },

    initTemplates() {
        document.getElementById('main-nav').innerHTML = templates.navbar;
        document.querySelector('.toast-container').innerHTML = templates.toast;
        document.getElementById('login-page').innerHTML = templates.loginPage;
        document.getElementById('register-page').innerHTML = templates.registerPage;
        document.getElementById('projects-page').innerHTML = templates.projectsPage;
        document.getElementById('project-details-page').innerHTML = templates.projectDetailsPage;
        document.getElementById('teachers-page').innerHTML = templates.teachersPage;
        document.getElementById('task-details-page').innerHTML = templates.taskDetailsPage;

        const modalsContainer = document.getElementById('modals-container');
        modalsContainer.innerHTML = 
            templates.modals.project + 
            templates.modals.member + 
            templates.modals.task + 
            templates.modals.dependency + 
            templates.modals.settings;
    },

    initEventListeners() {
        document.getElementById('login-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            const email = document.getElementById('login-email').value;
            const password = document.getElementById('login-password').value;
            await auth.login(email, password);
        });

        document.getElementById('register-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            const email = document.getElementById('register-email').value;
            const name = document.getElementById('register-name').value;
            const password = document.getElementById('register-password').value;
            const passwordConfirm = document.getElementById('register-password-confirm').value;
            await auth.register(email, name, password, passwordConfirm);
        });
    },

    setUserInfo(userId, globalRole, userName) {
        this.currentUser = userId;
        this.globalRole = globalRole;
        this.userName = userName;
        
        const userNameEl = document.getElementById('user-name');
        const roleBadge = globalRole === 'teacher' 
            ? '<span class="badge bg-warning text-dark me-2">Teacher</span>' 
            : '<span class="badge bg-info text-dark me-2">Student</span>';
        userNameEl.innerHTML = roleBadge + (userName || userId);
        
        document.getElementById('main-nav').style.display = 'block';
        
        if (globalRole === 'teacher') {
            document.getElementById('nav-projects-text').textContent = 'All Projects';
            document.getElementById('projects-filter').style.display = 'block';
            document.getElementById('nav-teachers').style.display = 'block';
        } else {
            document.getElementById('nav-projects-text').textContent = 'My Projects';
            document.getElementById('projects-filter').style.display = 'none';
            document.getElementById('nav-teachers').style.display = 'none';
        }
        
        notifications.startPolling();
    },

    hideAllPages() {
        document.getElementById('login-page').style.display = 'none';
        document.getElementById('register-page').style.display = 'none';
        document.getElementById('projects-page').style.display = 'none';
        document.getElementById('project-details-page').style.display = 'none';
        document.getElementById('task-details-page').style.display = 'none';
        document.getElementById('teachers-page').style.display = 'none';
    },

    showLogin() {
        this.hideAllPages();
        document.getElementById('login-page').style.display = 'block';
        document.getElementById('main-nav').style.display = 'none';
        document.getElementById('login-error').style.display = 'none';
        notifications.stopPolling();
    },

    showRegister() {
        this.hideAllPages();
        document.getElementById('register-page').style.display = 'block';
        document.getElementById('main-nav').style.display = 'none';
        document.getElementById('register-error').style.display = 'none';
    },

    async showProjects() {
        if (!auth.isAuthenticated()) {
            this.showLogin();
            return;
        }

        this.hideAllPages();
        document.getElementById('projects-page').style.display = 'block';
        
        try {
            const projects = await api.getProjects();
            this.allProjects = projects;
            await this.renderProjects(projects);
            
            document.getElementById('projects-title').textContent = 
                auth.isTeacher() ? 'All Projects' : 'My Projects';
        } catch (error) {
            this.showToast('Failed to load projects', 'error', error);
            console.error('Error loading projects:', error);
        }
    },

    async filterMyProjects(showOnlyMine) {
        if (showOnlyMine && auth.isTeacher()) {
            try {
                const memberships = await api.getUserMemberships();
                const myProjectIds = new Set(memberships.map(m => m.projectId));
                const filtered = this.allProjects.filter(p => myProjectIds.has(p.id));
                this.renderProjects(filtered);
            } catch (error) {
                console.error('Failed to load memberships:', error);
                this.renderProjects(this.allProjects);
            }
        } else {
            this.renderProjects(this.allProjects);
        }
    },

    async renderProjects(projects) {
        const container = document.getElementById('projects-list');
        container.innerHTML = '';

        if (projects.length === 0) {
            container.innerHTML = '<div class="col-12"><p class="text-muted">No projects found</p></div>';
            return;
        }

        const leaderPromises = projects.map(async (project) => {
            try {
                const memberships = await api.getProjectMemberships(project.id);
                const leader = memberships.find(m => m.isLeader);
                if (leader) {
                    const user = await api.getUser(leader.userId);
                    return { projectId: project.id, leaderName: user.name };
                }
            } catch (error) {
                console.error(`Failed to load leader for project ${project.id}:`, error);
            }
            return { projectId: project.id, leaderName: null };
        });

        const leaders = await Promise.all(leaderPromises);
        const leaderMap = new Map(leaders.map(l => [l.projectId, l.leaderName]));

        projects.forEach(project => {
            const leaderName = leaderMap.get(project.id);
            const card = document.createElement('div');
            card.className = 'col-md-4 mb-3';
            card.innerHTML = `
                <div class="card project-card" onclick="app.showProjectDetails('${project.id}')">
                    <div class="card-body">
                        <h5 class="card-title">${this.escapeHtml(project.name)}</h5>
                        <p class="card-text text-muted">${this.escapeHtml(project.description || 'No description')}</p>
                        <div class="text-muted small">
                            ${leaderName ? `<div><i class="bi bi-person-badge"></i> Leader: ${this.escapeHtml(leaderName)}</div>` : ''}
                            <div><i class="bi bi-calendar"></i> Due: ${this.formatDate(project.dueDate)}</div>
                            <div><i class="bi bi-clock"></i> Created: ${this.formatDate(project.createdAt)}</div>
                        </div>
                    </div>
                </div>`;
            container.appendChild(card);
        });
    },

    async showProjectDetails(projectId) {
        this.currentProjectId = projectId;
        this.hideAllPages();
        document.getElementById('project-details-page').style.display = 'block';

        try {
            const project = await api.getProject(projectId);
            this.renderProjectDetails(project);
            
            await this.loadProjectMembers();
            await this.loadProjectTasks();
            await this.showProjectTab('overview');
        } catch (error) {
            this.showToast('Failed to load project', 'error', error);
            console.error('Error loading project:', error);
            this.showProjects();
        }
    },

    renderProjectDetails(project) {
        document.getElementById('project-name').textContent = project.name;
        document.getElementById('project-description').textContent = project.description || 'No description';
        document.getElementById('project-start-date').textContent = project.startDate ? this.formatDate(project.startDate) : '-';
        document.getElementById('project-due-date').textContent = this.formatDate(project.dueDate);
        document.getElementById('project-created-at').textContent = this.formatDate(project.createdAt);

        const actionsDiv = document.getElementById('project-actions');
            actionsDiv.innerHTML = `
                <button class="btn btn-outline-primary" onclick="app.showEditProject('${project.id}')">
                    <i class="bi bi-pencil"></i> Edit
                </button>
                <button class="btn btn-outline-danger" onclick="app.deleteProject('${project.id}')">
                    <i class="bi bi-trash"></i> Delete
            </button>`;
    },

    async showProjectTab(tab) {
        const tabContent = document.getElementById('project-tab-content');
        
        document.querySelectorAll('#project-tabs .nav-link').forEach(link => {
            link.classList.remove('active');
        });
        
        const tabLinks = document.querySelectorAll('#project-tabs .nav-link');
        tabLinks.forEach(link => {
            const onclickAttr = link.getAttribute('onclick');
            if (onclickAttr && onclickAttr.includes(`'${tab}'`)) {
                link.classList.add('active');
            }
        });
        
        if (event?.target) {
            event.target.classList.add('active');
        }

        if (tab === 'overview') {
            await this.renderOverviewTab();
        } else if (tab === 'members') {
            await this.renderMembersTab();
        } else if (tab === 'tasks') {
            await this.renderTasksListTab();
        } else if (tab === 'gantt') {
            await this.renderGanttTab();
        }
        
        const reverseScheduleContainer = document.getElementById('reverse-schedule-button-container');
        if (reverseScheduleContainer) {
            reverseScheduleContainer.style.display = tab === 'gantt' ? 'block' : 'none';
        }
    },

    async loadProjectMembers() {
        try {
            const memberships = await api.getProjectMemberships(this.currentProjectId);
            this.currentProjectMembers = memberships;
        } catch (error) {
            console.error('Failed to load members:', error);
        }
    },

    async loadProjectTasks() {
        try {
            const tasks = await api.getTasksByProject(this.currentProjectId);
            this.currentProjectTasks = tasks;
        } catch (error) {
            console.error('Failed to load tasks:', error);
        }
    },

    async renderOverviewTab() {
        const tabContent = document.getElementById('project-tab-content');
        tabContent.innerHTML = '<div class="text-center py-4"><div class="spinner-border" role="status"></div><p class="mt-2">Loading stats...</p></div>';
        
        try {
            const stats = await api.getProjectStats(this.currentProjectId);
            
            // Time status
            let timeStatusHtml = '';
            if (stats.isOverdue) {
                const days = Math.abs(stats.daysRemaining);
                timeStatusHtml = `<span class="badge bg-danger fs-6"><i class="bi bi-exclamation-triangle"></i> Overdue by ${days} day${days !== 1 ? 's' : ''}</span>`;
            } else if (stats.daysRemaining <= 7) {
                timeStatusHtml = `<span class="badge bg-warning text-dark fs-6"><i class="bi bi-clock"></i> ${stats.daysRemaining} day${stats.daysRemaining !== 1 ? 's' : ''} remaining</span>`;
            } else {
                timeStatusHtml = `<span class="badge bg-success fs-6"><i class="bi bi-check-circle"></i> ${stats.daysRemaining} days remaining</span>`;
            }
            
            // Slack status
            let slackHtml = '';
            if (stats.slackDays !== null) {
                if (stats.slackDays < 0) {
                    slackHtml = `<span class="text-danger"><i class="bi bi-exclamation-circle"></i> Behind schedule by ${Math.abs(stats.slackDays)} day${Math.abs(stats.slackDays) !== 1 ? 's' : ''}</span>`;
                } else if (stats.slackDays === 0) {
                    slackHtml = `<span class="text-warning"><i class="bi bi-dash-circle"></i> No slack - on critical path</span>`;
                } else {
                    slackHtml = `<span class="text-success"><i class="bi bi-plus-circle"></i> ${stats.slackDays} day${stats.slackDays !== 1 ? 's' : ''} of slack</span>`;
                }
            }
            
            // Progress bar color
            let progressColor = 'bg-primary';
            if (stats.completionPercent >= 100) progressColor = 'bg-success';
            else if (stats.isOverdue) progressColor = 'bg-danger';
            else if (stats.completionPercent >= 75) progressColor = 'bg-info';
            
            let html = `
            <div class="row g-4">
                <!-- Progress Card -->
                <div class="col-md-6">
                    <div class="card h-100">
                        <div class="card-header bg-primary text-white">
                            <i class="bi bi-graph-up"></i> Progress
                        </div>
                        <div class="card-body">
                            <div class="d-flex justify-content-between mb-2">
                                <span>Completion</span>
                                <strong>${stats.completionPercent.toFixed(1)}%</strong>
                            </div>
                            <div class="progress mb-3" style="height: 25px;">
                                <div class="progress-bar ${progressColor}" role="progressbar" 
                                     style="width: ${stats.completionPercent}%">
                                    ${stats.completionPercent.toFixed(0)}%
                                </div>
                            </div>
                            
                            <div class="row text-center">
                                <div class="col">
                                    <div class="fs-3 text-muted">${stats.totalTasks}</div>
                                    <small class="text-muted">Total</small>
                                </div>
                                <div class="col">
                                    <div class="fs-3 text-success">${stats.completedTasks}</div>
                                    <small class="text-muted">Done</small>
                                </div>
                                <div class="col">
                                    <div class="fs-3 text-primary">${stats.inProgressTasks}</div>
                                    <small class="text-muted">In Progress</small>
                                </div>
                                <div class="col">
                                    <div class="fs-3 text-warning">${stats.needsReviewTasks}</div>
                                    <small class="text-muted">Review</small>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                
                <!-- Time Card -->
                <div class="col-md-6">
                    <div class="card h-100">
                        <div class="card-header ${stats.isOverdue ? 'bg-danger' : 'bg-info'} text-white">
                            <i class="bi bi-calendar-event"></i> Timeline
                        </div>
                        <div class="card-body">
                            <div class="mb-3">
                                ${timeStatusHtml}
                            </div>
                            <p><strong>Due Date:</strong> ${stats.dueDate}</p>
                            ${slackHtml ? `<p>${slackHtml}</p>` : ''}
                            ${stats.criticalTasksCount > 0 ? 
                                `<p><span class="badge bg-danger"><i class="bi bi-exclamation-triangle"></i> ${stats.criticalTasksCount} critical task${stats.criticalTasksCount !== 1 ? 's' : ''}</span></p>` 
                                : '<p><span class="badge bg-success"><i class="bi bi-check"></i> No critical tasks</span></p>'}
                        </div>
                    </div>
                </div>
                
                <!-- Team Card -->
                <div class="col-12">
                    <div class="card">
                        <div class="card-header bg-secondary text-white">
                            <i class="bi bi-people"></i> Team (${stats.membersCount} member${stats.membersCount !== 1 ? 's' : ''})
                        </div>
                        <div class="card-body">
                            ${stats.leader ? `
                            <div class="mb-3">
                                <strong><i class="bi bi-star-fill text-warning"></i> Leader:</strong>
                                ${this.escapeHtml(stats.leader.name)} 
                                <small class="text-muted">(${this.escapeHtml(stats.leader.email)})</small>
                            </div>` : ''}
                            
                            <div class="row">
                                ${stats.members.map(m => `
                                    <div class="col-md-4 col-sm-6 mb-2">
                                        <div class="d-flex align-items-center p-2 border rounded">
                                            <i class="bi bi-person-circle fs-4 me-2 text-muted"></i>
                                            <div>
                                                <div>${this.escapeHtml(m.name)} ${m.isLeader ? '<i class="bi bi-star-fill text-warning"></i>' : ''}</div>
                                                <small class="text-muted">${this.escapeHtml(m.email)}</small>
                                                ${m.tags && m.tags.length > 0 ? 
                                                    `<div>${m.tags.map(t => `<span class="badge bg-light text-dark me-1">${this.escapeHtml(t)}</span>`).join('')}</div>` 
                                                    : ''}
                                            </div>
                                        </div>
                                    </div>
                                `).join('')}
                            </div>
                        </div>
                    </div>
                </div>
            </div>`;
            
            tabContent.innerHTML = html;
        } catch (error) {
            console.error('Failed to load project stats:', error);
            tabContent.innerHTML = `<div class="alert alert-danger">Failed to load statistics: ${this.formatError(error)}</div>`;
        }
    },

    async renderMembersTab() {
        const tabContent = document.getElementById('project-tab-content');
        const members = this.currentProjectMembers || [];
        
        let html = '<div class="d-flex justify-content-between align-items-center mb-3">';
        html += '<h5>Project Members</h5>';
        
        if (auth.isTeacher() || this.isProjectLeader()) {
            html += '<button class="btn btn-primary btn-sm" onclick="app.showAddMember()">';
            html += '<i class="bi bi-person-plus"></i> Add Member</button>';
        }
        html += '</div>';
        
        const userPromises = members.map(async (membership) => {
            try {
                const user = await api.getUser(membership.userId);
                return { membership, userName: user.name };
            } catch (error) {
                console.error(`Failed to load user ${membership.userId}:`, error);
                return { membership, userName: membership.userId };
            }
        });
        
        const membersWithNames = await Promise.all(userPromises);
        
        html += '<div class="list-group">';
        membersWithNames.forEach(({ membership, userName }) => {
            const isLeader = membership.isLeader;
            html += `
                <div class="list-group-item d-flex justify-content-between align-items-center">
                    <div>
                        <strong>${this.escapeHtml(userName)}</strong>
                        ${isLeader ? '<span class="badge bg-warning text-dark ms-2">Leader</span>' : ''}
                        ${membership.tags && membership.tags.length > 0 ? 
                            membership.tags.map(tag => `<span class="badge bg-secondary ms-1">${this.escapeHtml(tag)}</span>`).join('') 
                            : ''}
                    </div>
                    <div>
                        ${(auth.isTeacher() || (this.isProjectLeader() && !isLeader)) ? 
                            `<button class="btn btn-sm btn-outline-danger" onclick="app.removeMember('${membership.id}')">
                                <i class="bi bi-trash"></i>
                            </button>` : ''}
                        ${(auth.isTeacher() || this.isProjectLeader()) && !isLeader ? 
                            `<button class="btn btn-sm btn-outline-warning ms-1" onclick="app.makeLeader('${membership.userId}')">
                                <i class="bi bi-star"></i> Make Leader
                            </button>` : ''}
                    </div>
                </div>`;
        });
        html += '</div>';
        
        tabContent.innerHTML = html;
    },

    getTaskTypeIcon(taskType) {
        const type = (taskType || '').toLowerCase();
        if (type === 'feature') {
            return '⭐ Feature';
        }
        return '🧩 Task';
    },

    getTaskStatusDisplay(task) {
        const status = (task.status || '').toLowerCase();
        const statusLabel = this.humanizeEnum(task.status);
        const lf = task.schedule?.lf ? new Date(task.schedule.lf) : null;
        const isLate = lf ? new Date() > lf : false;
        
        let statusColor = '#6c757d'; // gray - default
        if (status === 'accepted') {
            statusColor = '#155724'; // dark green
        } else if (status === 'done' || status === 'needsreview') {
            statusColor = '#198754'; // green
        } else if (status === 'rejected' || status === 'blocked' || isLate) {
            statusColor = '#dc3545'; // red
        } else if (status === 'inprogress') {
            statusColor = '#0d6efd'; // blue
        }
        
        return {
            label: statusLabel,
            color: statusColor
        };
    },

    getPriorityDisplay(priority) {
        const prio = (priority || '').toLowerCase();
        let color = '#6c757d'; // gray - default
        let icon = '';
        
        if (prio === 'critical') {
            color = '#dc3545'; // red
            icon = '<i class="bi bi-exclamation-triangle-fill"></i> ';
        } else if (prio === 'high') {
            color = '#fd7e14'; // orange
            icon = '<i class="bi bi-arrow-up-circle-fill"></i> ';
        } else if (prio === 'normal') {
            color = '#0d6efd'; // blue
        } else if (prio === 'low') {
            color = '#6c757d'; // gray
            icon = '<i class="bi bi-arrow-down-circle-fill"></i> ';
        }
        
        return {
            label: this.humanizeEnum(priority),
            color: color,
            icon: icon
        };
    },

    taskFilters: [],

    getTaskFilterDefinitions() {
        return [
            { key: 'type', label: 'Тип', options: [
                { value: 'all', label: 'Все' },
                { value: 'task', label: 'Task' },
                { value: 'feature', label: 'Feature' }
            ]},
            { key: 'status', label: 'Статус', options: [
                { value: 'all', label: 'Все' },
                { value: 'planned', label: 'Planned' },
                { value: 'inprogress', label: 'InProgress' },
                { value: 'needsreview', label: 'NeedsReview' },
                { value: 'accepted', label: 'Accepted' },
                { value: 'rejected', label: 'Rejected' },
                { value: 'blocked', label: 'Blocked' },
                { value: 'done', label: 'Done' }
            ]},
            { key: 'priority', label: 'Приоритет', options: [
                { value: 'all', label: 'Все' },
                { value: 'low', label: 'Low' },
                { value: 'normal', label: 'Normal' },
                { value: 'high', label: 'High' },
                { value: 'critical', label: 'Critical' }
            ]},
            { key: 'owner', label: 'Назначена', options: [] },
            { key: 'buffer', label: 'Буфер', options: [
                { value: 'all', label: 'Все' },
                { value: 'with', label: 'С буфером' },
                { value: 'without', label: 'Без буфера' }
            ]}
        ];
    },

    getAvailableFilterKeys() {
        const used = new Set(this.taskFilters.map(f => f.key));
        return this.getTaskFilterDefinitions().filter(def => !used.has(def.key));
    },

    addTaskFilterFromSelect(selectEl) {
        const key = selectEl.value;
        if (!key) return;
        const exists = this.taskFilters.some(f => f.key === key);
        if (!exists) {
            this.taskFilters.push({ key, value: 'all', active: true });
            this.renderTasksListTab();
        }
        selectEl.value = '';
    },

    setTaskFilterValue(key, value) {
        const filter = this.taskFilters.find(f => f.key === key);
        if (filter) {
            filter.value = value;
        }
    },

    toggleTaskFilterActive(key, active) {
        const filter = this.taskFilters.find(f => f.key === key);
        if (filter) {
            filter.active = active;
        }
    },

    applyTaskFiltersUI() {
        // Удаляем неактивные фильтры и перерисовываем
        this.taskFilters = this.taskFilters.filter(f => f.active);
        this.renderTasksListTab();
    },

    clearTaskFilters() {
        this.taskFilters = [];
        this.renderTasksListTab();
    },

    applyFiltersToTasks(tasksWithOwners) {
        if (!this.taskFilters.length) return tasksWithOwners;

        const activeFilters = this.taskFilters.filter(f => f.active && f.value && f.value !== 'all');
        if (!activeFilters.length) return tasksWithOwners;

        return tasksWithOwners.filter(({ task, ownerId }) => {
            return activeFilters.every(f => {
                const val = (f.value || '').toLowerCase();
                switch (f.key) {
                    case 'type':
                        return (task.taskType || '').toLowerCase() === val;
                    case 'status':
                        return (task.status || '').toLowerCase() === val;
                    case 'priority':
                        return (task.priority || '').toLowerCase() === val;
                    case 'owner':
                        return ownerId ? ownerId.toString() === val : false;
                    case 'buffer':
                        const hasBuffer = task.buffer && task.buffer > 0;
                        return val === 'with' ? hasBuffer : !hasBuffer;
                    default:
                        return true;
                }
            });
        });
    },

    buildFilterControlsHtml(ownerOptions) {
        const availableFilters = this.getAvailableFilterKeys();
        const definitions = this.getTaskFilterDefinitions();

        const filterChips = this.taskFilters.map(f => {
            const def = definitions.find(d => d.key === f.key);
            if (!def) return '';

            let options = def.options;
            if (f.key === 'owner') {
                const ownerOpts = ownerOptions.length
                    ? ownerOptions
                    : [{ value: 'all', label: 'Все' }];
                options = [{ value: 'all', label: 'Все' }, ...ownerOpts];
            }

            const selectOptions = options.map(o => {
                const selected = (f.value || 'all') === o.value ? 'selected' : '';
                return `<option value="${o.value}" ${selected}>${this.escapeHtml(o.label)}</option>`;
            }).join('');

            const checkboxChecked = f.active ? 'checked' : '';

            return `
                <div class="filter-chip d-flex align-items-center gap-2 flex-wrap">
                    <input type="checkbox" class="form-check-input" ${checkboxChecked}
                        onchange="app.toggleTaskFilterActive('${f.key}', this.checked)">
                    <span class="fw-semibold">${this.escapeHtml(def.label)}</span>
                    <select class="form-select form-select-sm filter-select"
                        onchange="app.setTaskFilterValue('${f.key}', this.value)">
                        ${selectOptions}
                    </select>
                </div>
            `;
        }).join('');

        const addSelectOptions = ['<option value="">Выберите...</option>']
            .concat(availableFilters.map(f => `<option value="${f.key}">${this.escapeHtml(f.label)}</option>`))
            .join('');

        return `
            <div class="card mb-3">
                <div class="card-body">
                    <div class="d-flex flex-wrap align-items-center gap-2 mb-3">
                        <select class="form-select form-select-sm" style="width: 220px;"
                            onchange="app.addTaskFilterFromSelect(this)">
                            ${addSelectOptions}
                        </select>
                        <span class="text-muted">добавить фильтр</span>
                        <div class="ms-auto d-flex gap-2">
                            <button class="btn btn-primary btn-sm" onclick="app.applyTaskFiltersUI()">Применить</button>
                            <button class="btn btn-outline-secondary btn-sm" onclick="app.clearTaskFilters()">Очистить</button>
                        </div>
                    </div>
                    <div class="d-flex flex-wrap gap-2">
                        ${filterChips || '<span class="text-muted">Фильтры не выбраны</span>'}
                    </div>
                </div>
            </div>
        `;
    },

    async renderTasksListTab() {
        const tabContent = document.getElementById('project-tab-content');
        const tasks = this.currentProjectTasks || [];
        
        let html = '<div class="d-flex justify-content-between align-items-center mb-3">';
        html += '<h5>Tasks</h5>';
        
        if (auth.isAuthenticated()) {
            html += '<button class="btn btn-primary btn-sm" onclick="app.showCreateTask()">';
            html += '<i class="bi bi-plus-circle"></i> Add Task</button>';
        }
        html += '<div class="btn-group ms-2">';
        html += '<button type="button" class="btn btn-success btn-sm dropdown-toggle" data-bs-toggle="dropdown" aria-expanded="false">';
        html += '<i class="bi bi-download"></i> Export</button>';
        html += '<ul class="dropdown-menu">';
        html += '<li><a class="dropdown-item" href="#" onclick="app.exportTasks(\'html\'); return false;"><i class="bi bi-filetype-html"></i> Export to HTML</a></li>';
        html += '<li><a class="dropdown-item" href="#" onclick="app.exportTasks(\'pdf\'); return false;"><i class="bi bi-filetype-pdf"></i> Export to PDF</a></li>';
        html += '<li><a class="dropdown-item" href="#" onclick="app.exportTasks(\'csv\'); return false;"><i class="bi bi-filetype-csv"></i> Export to CSV</a></li>';
        html += '</ul></div></div>';
        
        if (tasks.length === 0) {
            html += '<p class="text-muted">No tasks yet</p>';
        } else {
            const ownerMap = new Map();
            const taskPromises = tasks.map(async (task) => {
                let ownerName = 'Unassigned';
                let ownerId = null;
                try {
                    const assignments = await api.getTaskAssignments(task.id);
                    const ownerAssignment = assignments.find(a => a.role === 'owner');
                    if (ownerAssignment) {
                        ownerId = ownerAssignment.userId;
                        const user = await api.getUser(ownerAssignment.userId);
                        ownerName = user.name;
                        ownerMap.set(ownerId.toString(), user.name);
                    }
                } catch (error) {
                    console.error(`Failed to load owner for task ${task.id}:`, error);
                }
                
                return { task, ownerName, ownerId };
            });
            
            const tasksWithOwners = await Promise.all(taskPromises);
            const ownerOptions = Array.from(ownerMap.entries()).map(([id, name]) => ({
                value: id,
                label: name
            }));

            html += this.buildFilterControlsHtml(ownerOptions);
            const filteredTasks = this.applyFiltersToTasks(tasksWithOwners);
            
            html += '<div class="table-responsive"><table class="table table-hover tasks-table">';
            html += '<thead><tr>';
            html += '<th style="width: 80px;">Type</th>';
            html += '<th style="width: 120px;">Status</th>';
            html += '<th style="width: 100px;">Priority</th>';
            html += '<th>Title</th>';
            html += '<th style="width: 150px;">Assignee</th>';
            html += '<th style="width: 120px;">Progress</th>';
            html += '<th style="width: 200px;">Info</th>';
            html += '</tr></thead><tbody>';
            
            filteredTasks.forEach(({ task, ownerName }) => {
                const taskId = typeof task.id === 'string' ? task.id : task.id.toString();
                const typeIcon = this.getTaskTypeIcon(task.taskType);
                const statusDisplay = this.getTaskStatusDisplay(task);
                const priorityDisplay = this.getPriorityDisplay(task.priority);
                const isCritical = task.schedule && task.schedule.isCritical;
                const lf = task.schedule?.lf ? new Date(task.schedule.lf) : null;
                const slack = task.schedule?.slack;
                const progress = task.progress || 0;
                const isOverdue = lf ? new Date() > lf : false;
                
                const progressBgColor = isOverdue ? '#dc3545' : '#e9ecef';
                const textColor = progress > 50 ? '#fff' : (isOverdue ? '#fff' : '#495057');
                const progressBar = `
                    <div class="task-progress-container" style="width: 100px; height: 20px; background-color: ${progressBgColor}; border-radius: 4px; position: relative; overflow: hidden;">
                        <div class="task-progress-bar" style="width: ${progress}%; height: 100%; background-color: #198754; transition: width 0.3s ease;"></div>
                        <span class="task-progress-text" style="position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); font-size: 0.75rem; font-weight: 600; color: ${textColor}; pointer-events: none; z-index: 1; text-shadow: 0 1px 2px rgba(0,0,0,0.1);">${progress}%</span>
                    </div>
                `;
                
                let infoBadges = '';
                if (isCritical) {
                    infoBadges += '<span class="badge bg-danger" style="font-size: 0.7em; margin-right: 4px;">Critical</span>';
                }
                if (lf) {
                    const dueDate = this.formatDate(lf.toISOString());
                    infoBadges += `<span class="badge bg-secondary" style="font-size: 0.7em; margin-right: 4px;">Due: ${dueDate}</span>`;
                }
                if (slack !== null && slack !== undefined) {
                    const slackHours = Math.round(slack / 3600);
                    const slackClass = slackHours < 0 ? 'bg-danger' : slackHours === 0 ? 'bg-warning' : 'bg-info';
                    infoBadges += `<span class="badge ${slackClass}" style="font-size: 0.7em; margin-right: 4px;">Slack: ${slackHours}h</span>`;
                }
                if (task.buffer && task.buffer > 0) {
                    const bufferHours = Math.round(task.buffer / 3600);
                    infoBadges += `<span class="badge bg-primary" style="font-size: 0.7em;">Buffer: ${bufferHours}h</span>`;
                }
                
                html += `<tr class="task-row" onclick="app.showTaskDetails('${taskId}')" style="cursor: pointer;">`;
                html += `<td>${typeIcon}</td>`;
                html += `<td><div class="d-flex align-items-center"><span class="status-indicator" style="width: 4px; height: 16px; background-color: ${statusDisplay.color}; margin-right: 6px; border-radius: 2px;"></span><span>${this.escapeHtml(statusDisplay.label)}</span></div></td>`;
                html += `<td style="color: ${priorityDisplay.color};">${priorityDisplay.icon}${this.escapeHtml(priorityDisplay.label)}</td>`;
                html += `<td class="task-title">${this.escapeHtml(task.name)}</td>`;
                html += `<td>${this.escapeHtml(ownerName)}</td>`;
                html += `<td>${progressBar}</td>`;
                html += `<td>${infoBadges}</td>`;
                html += '</tr>';
            });
            
            html += '</tbody></table></div>';

            const currentUserId = localStorage.getItem('userId');
            const needsReview = tasksWithOwners.filter(t => 
                t.task.status === 'needsReview' || t.task.status === 'NeedsReview'
            );
            const mySentToReview = needsReview.filter(t => t.ownerId === currentUserId);

            if (auth.isTeacher() || this.isProjectLeader()) {
                html += '<div class="mt-4"><h6><i class="bi bi-clipboard-check"></i> Review Queue</h6>';
                if (needsReview.length === 0) {
                    html += '<p class="text-muted">No tasks awaiting review.</p>';
                } else {
                    html += '<ul class="list-group">';
                    needsReview.forEach(t => {
                        const taskId = typeof t.task.id === 'string' ? t.task.id : t.task.id.toString();
                        html += `<li class="list-group-item d-flex justify-content-between align-items-center" style="cursor: pointer;" onclick="app.showTaskDetails('${taskId}')">
                            <div><strong>${this.escapeHtml(t.task.name)}</strong><br><small class="text-muted">Owner: ${this.escapeHtml(t.ownerName)}</small></div>
                            <span class="badge bg-warning text-dark">Needs Review</span>
                        </li>`;
                    });
                    html += '</ul>';
        }
        html += '</div>';
            }

            if (!auth.isTeacher()) {
                html += '<div class="mt-4"><h6><i class="bi bi-send"></i> My tasks sent to review</h6>';
                if (mySentToReview.length === 0) {
                    html += '<p class="text-muted">No tasks in review.</p>';
                } else {
                    html += '<ul class="list-group">';
                    mySentToReview.forEach(t => {
                        const taskId = typeof t.task.id === 'string' ? t.task.id : t.task.id.toString();
                        html += `<li class="list-group-item d-flex justify-content-between align-items-center" style="cursor: pointer;" onclick="app.showTaskDetails('${taskId}')">
                            <div><strong>${this.escapeHtml(t.task.name)}</strong></div>
                            <span class="badge bg-info text-dark">Awaiting Review</span>
                        </li>`;
                    });
                    html += '</ul>';
                }
                html += '</div>';
            }
        }
        
        tabContent.innerHTML = html;
    },

    async showTaskDetails(taskId) {
        const taskIdStr = typeof taskId === 'string' ? taskId : String(taskId);
        this.currentTaskId = taskIdStr;
        this.hideAllPages();
        document.getElementById('task-details-page').style.display = 'block';

        try {
            const task = await api.getTask(taskIdStr);
            this.renderTaskDetails(task);
        } catch (error) {
            console.error('Error loading task:', error);
            this.showToast('Failed to load task', 'error', error);
            if (this.currentProjectId) {
                await this.showProjectDetails(this.currentProjectId);
            } else {
                this.showProjects();
            }
        }
    },

    async renderTaskDetails(task) {
        document.getElementById('task-name').textContent = task.name;
        
        let ownerName = 'Unassigned';
        let isOwner = false;
        try {
            const assignments = await api.getTaskAssignments(task.id);
            const ownerAssignment = assignments.find(a => a.role === 'owner');
            if (ownerAssignment) {
                const user = await api.getUser(ownerAssignment.userId);
                ownerName = user.name;
                const currentUserId = localStorage.getItem('userId');
                isOwner = ownerAssignment.userId === currentUserId;
            }
        } catch (error) {
            console.error(`Failed to load owner for task ${task.id}:`, error);
        }
        
        const content = document.getElementById('task-details-content');
        const statusLabel = this.humanizeEnum(task.status);
        const priorityLabel = this.humanizeEnum(task.priority);
        const taskTypeLabel = this.humanizeEnum(task.taskType);
        const hardnessLabel = this.humanizeEnum(task.hardness);
        
        let html = `
            <div class="mb-3"><label class="form-label"><strong>Description</strong></label><p>${this.escapeHtml(task.description || 'No description')}</p></div>
            <div class="row mb-3">
                <div class="col-md-3"><label class="form-label"><strong>Status</strong></label><p><span class="status-badge status-${(task.status || '').toLowerCase().replace('_', '-')}">${this.escapeHtml(statusLabel)}</span></p></div>
                <div class="col-md-3"><label class="form-label"><strong>Priority</strong></label><p>${this.escapeHtml(priorityLabel)}</p></div>
                <div class="col-md-3"><label class="form-label"><strong>Task Type</strong></label><p>${this.escapeHtml(taskTypeLabel)}</p></div>
                <div class="col-md-3"><label class="form-label"><strong>Owner</strong></label><p>${this.escapeHtml(ownerName)}</p></div>
            </div>
            <div class="row mb-3">
                <div class="col-md-3"><label class="form-label"><strong>Estimated Duration</strong></label><p>${task.estimatedDuration ? Math.round(task.estimatedDuration / 3600) + ' hours' : '-'}</p></div>
                <div class="col-md-3"><label class="form-label"><strong>Hardness</strong></label><p>${this.escapeHtml(hardnessLabel)}</p></div>
                <div class="col-md-3"><label class="form-label"><strong>Buffer</strong></label><p>${task.buffer ? Math.round(task.buffer / 3600) + ' hours' : '-'}</p></div>
            </div>
            ${task.schedule && (task.schedule.ls || task.schedule.lf) ? `
            <div class="row mb-3">
                ${task.schedule.ls ? `<div class="col-md-3"><label class="form-label"><strong>Latest Start</strong></label><p>${this.formatDate(task.schedule.ls)}</p></div>` : ''}
                ${task.schedule.lf ? `<div class="col-md-3"><label class="form-label"><strong>Latest Finish</strong></label><p>${this.formatDate(task.schedule.lf)}</p></div>` : ''}
                ${task.schedule.slack !== undefined && task.schedule.slack !== null ? `<div class="col-md-3"><label class="form-label"><strong>Slack</strong></label><p>${Math.round(task.schedule.slack / 3600)} hours</p></div>` : ''}
                ${task.schedule.isCritical ? `<div class="col-md-3"><label class="form-label"><strong>Critical Path</strong></label><p><span class="badge bg-danger">Yes</span></p></div>` : ''}
                </div>` : ''}
            ${task.progress !== undefined ? `<div class="row mb-3"><div class="col-md-3"><label class="form-label"><strong>Progress</strong></label><p>${task.progress}%</p></div></div>` : ''}
        `;
        
        html += '<div class="mb-3"><h5>Dependencies</h5><div id="dependencies-list"></div></div>';
        html += '<div class="mb-3"><h5>Artifacts</h5><div id="artifacts-list"></div><div id="artifacts-form-container"></div></div>';
        html += '<div class="mb-3"><h5>Review</h5><div id="review-section"></div></div>';
        
        content.innerHTML = html;
        
        const taskIdStr = typeof task.id === 'string' ? task.id : String(task.id);
        await this.loadTaskDependencies(taskIdStr);
        await this.loadTaskArtifacts(taskIdStr, { canEdit: auth.isTeacher() || isOwner || this.isProjectLeader() });
        await this.loadTaskReview(taskIdStr, task, isOwner);
        
        const actionsDiv = document.getElementById('task-actions');
        const isTaskOwner = await this.isTaskOwner(taskIdStr);
        const isLeader = this.isProjectLeader();
        
        let actionButtons = '';
        if (auth.isTeacher() || isTaskOwner || isLeader) {
            actionButtons += `<button class="btn btn-outline-primary" onclick="app.showEditTask('${taskIdStr}')"><i class="bi bi-pencil"></i> Edit</button>`;
        }
        
        const taskStatus = task.status?.toLowerCase() || '';
        if (isTaskOwner && taskStatus !== 'needsreview' && taskStatus !== 'done' && taskStatus !== 'rejected') {
            actionButtons += `<button class="btn btn-outline-warning ms-2" onclick="app.sendTaskToReview('${taskIdStr}')"><i class="bi bi-send"></i> Send to Review</button>`;
        }
        
        if (auth.isTeacher() || isLeader) {
            actionButtons += `<button class="btn btn-outline-danger ms-2" onclick="app.deleteTask('${taskIdStr}')"><i class="bi bi-trash"></i> Delete</button>`;
        }
        
        if (actionButtons) {
            actionsDiv.innerHTML = actionButtons;
        }
    },

    async loadTaskDependencies(taskId) {
        try {
            const dependencies = await api.getDependencies(taskId);
            const listDiv = document.getElementById('dependencies-list');
            
            const isTaskOwner = await this.isTaskOwner(taskId);
            const isLeader = this.isProjectLeader();
            const canManage = auth.isTeacher() || isTaskOwner || isLeader;
            
            if (dependencies.length === 0) {
                listDiv.innerHTML = '<p class="text-muted">No dependencies</p>';
            } else {
                const currentTask = await api.getTask(taskId);
                const currentTaskName = currentTask.name;
                
                const taskNamePromises = dependencies.map(async (dep) => {
                    const isOutgoing = dep.fromTaskId === taskId || dep.from_task_id === taskId;
                    const otherTaskId = isOutgoing ? dep.toTaskId : dep.fromTaskId;
                    
                    try {
                        const otherTask = await api.getTask(otherTaskId);
                        return { dep, otherTaskName: otherTask.name, isOutgoing };
                    } catch (error) {
                        return { dep, otherTaskName: `Task ${otherTaskId}`, isOutgoing };
                    }
                });
                
                const dependenciesWithNames = await Promise.all(taskNamePromises);
                
                let html = '';
                dependenciesWithNames.forEach(({ dep, otherTaskName, isOutgoing }) => {
                    const depTypeLabel = dep.depType || dep.dep_type || 'FS';
                    const fromName = isOutgoing ? currentTaskName : otherTaskName;
                    const toName = isOutgoing ? otherTaskName : currentTaskName;
                    
                    html += `
                        <div class="dependency-item d-flex justify-content-between align-items-center mb-2">
                            <span><strong>${this.escapeHtml(fromName)}</strong> → <strong>${this.escapeHtml(toName)}</strong> <span class="badge bg-info text-dark ms-1">${this.escapeHtml(depTypeLabel)}</span></span>
                            ${canManage ? `<button class="btn btn-sm btn-outline-danger" onclick="app.removeDependency('${taskId}', '${dep.id}')"><i class="bi bi-trash"></i></button>` : ''}
                        </div>`;
                });
                listDiv.innerHTML = html;
            }
            
            if (canManage) {
                listDiv.innerHTML += `<button class="btn btn-sm btn-primary mt-2" onclick="app.showAddDependency('${taskId}')"><i class="bi bi-plus"></i> Add Dependency</button>`;
            }
        } catch (error) {
            console.error('Failed to load dependencies:', error);
        }
    },

    isProjectLeader() {
        if (!this.currentProjectId || !this.currentProjectMembers) return false;
        const currentUserId = localStorage.getItem('userId');
        const membership = this.currentProjectMembers.find(m => m.userId === currentUserId);
        return membership ? membership.isLeader : false;
    },

    async isTaskOwner(taskId) {
        try {
            const currentUserId = localStorage.getItem('userId');
            const assignments = await api.getTaskAssignments(taskId);
            return assignments.some(a => a.userId === currentUserId && a.role === 'owner');
        } catch (error) {
            console.error('Failed to check task ownership:', error);
        return false;
        }
    }
};

document.addEventListener('DOMContentLoaded', () => {
    app.init();
});
