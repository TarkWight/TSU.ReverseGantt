Object.assign(app, {
    async showTeachers() {
        if (!auth.isAuthenticated() || !auth.isTeacher()) {
            this.showProjects();
            return;
        }

        this.hideAllPages();
        document.getElementById('teachers-page').style.display = 'block';

        try {
            const projects = await api.getProjects();
            const select = document.getElementById('reverse-schedule-project');
            select.innerHTML = '<option value="">Select project...</option>';
            projects.forEach(project => {
                const option = document.createElement('option');
                option.value = project.id;
                option.textContent = project.name;
                select.appendChild(option);
            });
        } catch (error) {
            console.error('Failed to load projects:', error);
        }

        await this.loadAllUsers();
    },

    async loadAllUsers() {
        try {
            const users = await api.getAllUsers();
            const students = users.filter(u => u.globalRole === 'student');
            
            const container = document.getElementById('users-list');
            if (students.length === 0) {
                container.innerHTML = '<p class="text-muted">No students found</p>';
                return;
            }

            let html = '<div class="list-group">';
            students.forEach(user => {
                html += `
                    <div class="list-group-item d-flex justify-content-between align-items-center">
                        <div>
                            <strong>${this.escapeHtml(user.name)}</strong>
                            <div class="text-muted small">${this.escapeHtml(user.email)}</div>
                        </div>
                        <button class="btn btn-sm btn-warning" onclick="app.promoteToTeacher('${user.id}')">
                            <i class="bi bi-star"></i> Promote to Teacher
                        </button>
                    </div>`;
            });
            html += '</div>';
            container.innerHTML = html;
        } catch (error) {
            console.error('Failed to load users:', error);
            document.getElementById('users-list').innerHTML = '<p class="text-danger">Failed to load users</p>';
        }
    },

    async promoteToTeacher(userId) {
        if (!confirm('Are you sure you want to promote this student to teacher?')) {
            return;
        }

        try {
            await api.promoteToTeacher(userId);
            this.showToast('User promoted to teacher successfully', 'success');
            await this.loadAllUsers();
        } catch (error) {
            this.showToast('Failed to promote user', 'error', error);
            console.error('Error promoting user:', error);
        }
    },

    async runReverseSchedule() {
        const projectId = document.getElementById('reverse-schedule-project').value;
        if (!projectId) {
            this.showToast('Please select a project', 'error');
            return;
        }

        const resultDiv = document.getElementById('reverse-schedule-result');
        resultDiv.style.display = 'block';
        resultDiv.innerHTML = '<div class="spinner-border spinner-border-sm" role="status"><span class="visually-hidden">Loading...</span></div> Calculating schedule...';

        try {
            const response = await api.reverseSchedule(projectId);
            resultDiv.innerHTML = `
                <div class="alert alert-success">
                    <h6><i class="bi bi-check-circle"></i> Schedule calculated successfully!</h6>
                    <p class="mb-0">Updated ${response.tasks.length} task(s) with reverse schedule.</p>
                    <button class="btn btn-sm btn-primary mt-2" onclick="app.showProjectDetails('${projectId}')">View Project</button>
                </div>`;
            this.showToast('Reverse schedule calculated successfully', 'success');
        } catch (error) {
            resultDiv.innerHTML = `
                <div class="alert alert-danger">
                    <h6><i class="bi bi-exclamation-triangle"></i> Failed to calculate schedule</h6>
                    <p class="mb-0">${this.escapeHtml(this.formatError(error))}</p>
                </div>`;
            this.showToast('Failed to calculate schedule', 'error', error);
            console.error('Error calculating reverse schedule:', error);
        }
    },

    async runReverseScheduleForCurrentProject() {
        if (!this.currentProjectId) {
            this.showToast('No project selected', 'error');
            return;
        }

        if (!confirm('Calculate reverse schedule for this project? This will update all task schedules.')) {
            return;
        }

        try {
            this.showToast('Calculating schedule...', 'info');
            const response = await api.reverseSchedule(this.currentProjectId);
            this.showToast(`Schedule calculated successfully! Updated ${response.tasks.length} task(s).`, 'success');
            
            await this.loadProjectTasks();
            await this.showProjectTab('gantt');
        } catch (error) {
            this.showToast('Failed to calculate schedule', 'error', error);
            console.error('Error calculating reverse schedule:', error);
        }
    }
});

