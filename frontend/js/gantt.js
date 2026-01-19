Object.assign(app, {
    async renderGanttTab() {
        const tabContent = document.getElementById('project-tab-content');
        const tasks = this.currentProjectTasks || [];
        
        let html = '<div class="d-flex justify-content-between align-items-center mb-3">';
        html += '<h5>Gantt Chart</h5>';
        html += '<button class="btn btn-sm btn-outline-secondary" onclick="app.showGanttLegend()" title="Show legend">';
        html += '<i class="bi bi-info-circle"></i> Legend';
        html += '</button>';
        html += '</div>';
        html += '<div id="gantt-container" class="gantt-container"></div>';
        
        tabContent.innerHTML = html;
        await this.renderGanttChart(tasks);
    },

    renderGanttChart: async function (tasks) {
        try {
            const container = document.getElementById('gantt-container');
            if (!container) return;
            if (!tasks || tasks.length === 0) {
                container.innerHTML = '<p class="text-muted">No tasks to display</p>';
                return;
            }

            let projectStartDate = null;
            let projectDueDate = null;
            if (this.currentProjectId) {
                try {
                    const project = await api.getProject(this.currentProjectId);
                    if (project.startDate) {
                        const dateStr = project.startDate.split('T')[0];
                        const [year, month, day] = dateStr.split('-').map(Number);
                        projectStartDate = new Date(Date.UTC(year, month - 1, day, 0, 0, 0, 0));
                    }
                    if (project.dueDate) {
                        const dateStr = project.dueDate.split('T')[0];
                        const [year, month, day] = dateStr.split('-').map(Number);
                        projectDueDate = new Date(Date.UTC(year, month - 1, day, 23, 59, 59, 999));
                    }
                } catch (error) {
                    console.error('Failed to load project:', error);
                }
            }

            const taskDataPromises = tasks.map(async (task) => {
                let ownerName = 'Unassigned';
                let ownerId = null;

                try {
                    const assignments = await api.getTaskAssignments(task.id);
                    const ownerAssignment = assignments.find(a => a.role === 'owner');
                    if (ownerAssignment) {
                        ownerId = ownerAssignment.userId;
                        const user = await api.getUser(ownerAssignment.userId);
                        ownerName = user.name;
                    }
                } catch (error) {
                    console.error(`Failed to load owner for task ${task.id}:`, error);
                }

                return {task, ownerName, ownerId};
            });

            const tasksWithData = await Promise.all(taskDataPromises);

            const hasSchedule = tasksWithData.some(({task}) => {
                return task.schedule && task.schedule.ls && task.schedule.lf;
            });

            if (!hasSchedule) {
                container.innerHTML = '<p class="text-muted">Schedule not calculated. Click \'Calculate Reverse Schedule\'.</p>';
                return;
            }

            const items = tasksWithData
                .map(({task, ownerName, ownerId}) => {
                    const ls = task.schedule && task.schedule.ls;
                    const lf = task.schedule && task.schedule.lf;
                    if (!ls || !lf) return null;

                    const lsDate = new Date(ls);
                    const lfDate = new Date(lf);
                    if (isNaN(lsDate.getTime()) || isNaN(lfDate.getTime())) return null;

                    return {
                        task,
                        ownerName,
                        ownerId,
                        ls: lsDate,
                        lf: lfDate,
                        isCritical: task.schedule && task.schedule.isCritical,
                        slack: task.schedule && task.schedule.slack
                    };
                })
                .filter(Boolean);

            if (items.length === 0) {
                container.innerHTML = '<p class="text-muted">Schedule not calculated. Click \'Calculate Reverse Schedule\'.</p>';
                return;
            }

            const dependencyLists = await Promise.all(
                items.map(i => api.getDependencies(i.task.id).catch(() => []))
            );
            const depMap = new Map(); // depId -> dep
            dependencyLists.flat().forEach(dep => {
                const depId = dep.id || `${dep.fromTaskId || dep.from_task_id}->${dep.toTaskId || dep.to_task_id}`;
                if (!depMap.has(depId)) depMap.set(depId, dep);
            });
            const allDeps = Array.from(depMap.values());

            const minTaskTime = new Date(Math.min(...items.map(i => i.ls.getTime())));
            const minTime = projectStartDate && projectStartDate.getTime() < minTaskTime.getTime()
                ? projectStartDate
                : minTaskTime;

            const maxTime = new Date(Math.max(...items.map(i => i.lf.getTime())));
            const gridEnd = projectDueDate && projectDueDate.getTime() > maxTime.getTime()
                ? projectDueDate
                : maxTime;

            const rangeMs = gridEnd.getTime() - minTime.getTime();
            const rangeHours = rangeMs / (1000 * 60 * 60);

            let timeDivisionHours;
            let pxPerHour;

            if (rangeHours <= 12) {
                timeDivisionHours = 1;
                pxPerHour = 60;
            } else if (rangeHours <= 48) {
                timeDivisionHours = 6;
                pxPerHour = 40;
            } else if (rangeHours <= 168) {
                timeDivisionHours = 12;
                pxPerHour = 30;
            } else {
                timeDivisionHours = 24;
                pxPerHour = 20;
            }

            const pxPerMs = pxPerHour / (1000 * 60 * 60);
            const timeDivisionMs = timeDivisionHours * 60 * 60 * 1000;

            const gridStart = new Date(minTime);
            const startHour = gridStart.getHours();
            const roundedHour = Math.floor(startHour / timeDivisionHours) * timeDivisionHours;
            gridStart.setHours(roundedHour, 0, 0, 0);

            const timeToPx = (time) => {
                return (time.getTime() - gridStart.getTime()) * pxPerMs;
            };

            const timeGrid = [];
            let currentTime = new Date(gridStart);

            while (currentTime <= gridEnd) {
                timeGrid.push(new Date(currentTime));
                currentTime = new Date(currentTime.getTime() + timeDivisionMs);
            }
            if (timeGrid.length === 0 || timeGrid[timeGrid.length - 1].getTime() < gridEnd.getTime()) {
                timeGrid.push(new Date(gridEnd));
            }

            const totalGridWidth = timeToPx(gridEnd);

            let rowHeight = 34;
            const rowGap = 14;
            const taskRowHeight = rowHeight + rowGap;
            const totalRows = items.length;
            const gridHeight = totalRows * taskRowHeight + 20;
            const headerHeight = 60;

            const now = new Date();
            const nowPx = timeToPx(now);

            let html = '<div class="gantt-wrapper" style="display: flex; border: 1px solid #dee2e6;">';

            this.ganttTaskListWidth = this.ganttTaskListWidth || 300;
            const taskListWidth = this.ganttTaskListWidth;
            html += `<div id="gantt-task-list" class="gantt-task-list" style="width: ${taskListWidth}px; background: #f8f9fa; flex-shrink: 0; display: flex; flex-direction: column; position: relative;">`;
            html += `<div style="padding: 8px; font-weight: 600; border-bottom: 1px solid #dee2e6; background: white; flex-shrink: 0; height: ${headerHeight}px; display: flex; align-items: center;">Tasks</div>`;
            html += `<div style="position: relative; height: ${gridHeight}px; overflow-y: auto;">`;

            items.forEach((item, rowIndex) => {
                html += `<div class="gantt-task-item" style="position: absolute; top: ${rowIndex * taskRowHeight}px; left: 0; right: 0; height: ${rowHeight}px; padding: 4px 8px; border-bottom: ${rowGap}px solid transparent; cursor: pointer; display: flex; align-items: center;" onclick="app.showTaskDetails('${item.task.id}')">`;
                html += `<div style="font-weight: 500; font-size: 0.85rem; line-height: 1.2; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">${this.escapeHtml(item.task.name)}</div>`;
                html += `</div>`;
            });

            html += '</div>';
            html += `<div id="gantt-resizer" class="gantt-resizer" style="position: absolute; right: 0; top: 0; bottom: 0; width: 4px; background: #dee2e6; cursor: col-resize; z-index: 20; border-right: 1px solid #adb5bd;"></div>`;
            html += '</div>';

            html += `<div id="gantt-grid-wrapper" class="gantt-grid-wrapper" style="flex: 1; overflow-x: auto; position: relative; min-width: 0;">`;

            html += `<div class="gantt-header" style="position: relative; z-index: 10; background: white; border-bottom: 2px solid #dee2e6; height: ${headerHeight}px; width: ${totalGridWidth}px;">`;
            html += `<div class="gantt-header-row" style="position: relative; height: 100%;">`;

            timeGrid.forEach((time, index) => {
                if (index === timeGrid.length - 1) return;
                const leftPx = timeToPx(time);
                const rightPx = timeToPx(timeGrid[index + 1]);
                const width = Math.max(1, rightPx - leftPx);
                html += `<div style="position: absolute;
                left: ${
                        leftPx
                }px;
                top: 0;
                width: ${width}px;
                height: 100%;
                padding: 8px;
                text-align: center;
                border-right: 1px solid #dee2e6;
                font-size: 0.85rem;
                background: #f8f9fa;
                display: flex;
                align-items: center;
                justify-content: center;
                ">$ {
                    this.escapeHtml(timeLabel)
                }</div>`;
            });

            if (nowPx >= 0 && nowPx <= totalGridWidth) {
                const nowLabel = `Now ${this.formatLocal(now, {hour: '2-digit', minute: '2-digit', hour12: false})}`;
                html += `<div class="gantt-now-line-header" style="position: absolute; left: ${nowPx}px; top: 0; bottom: 0; width: 3px; background: #ffc107; z-index: 12; pointer-events: none; box-shadow: 0 0 4px rgba(255, 193, 7, 0.8);"></div>`;
                html += `<div class="gantt-now-label" style="position: absolute; left: ${Math.max(0, nowPx - 35)}px; top: 2px; background: #ffc107; color: #000; padding: 2px 6px; border-radius: 3px; font-size: 0.7rem; font-weight: 700; z-index: 13; pointer-events: none; white-space: nowrap;">${this.escapeHtml(nowLabel)}</div>`;
            }

            html += '</div>';
            html += '</div>';

            html += `<div class="gantt-grid-body" style="position: relative; height: ${gridHeight}px; width: ${totalGridWidth}px;">`;

            if (projectStartDate && projectStartDate.getTime() >= minTime.getTime() && projectStartDate.getTime() <= gridEnd.getTime()) {
                const startPx = timeToPx(projectStartDate);
                html += `<div class="gantt-start-line" style="position: absolute; left: ${startPx}px; top: 0; bottom: 0; width: 2px; background: #198754; z-index: 4; pointer-events: none; border-left: 2px solid #198754;"></div>`;
            }

            if (projectStartDate && projectStartDate.getTime() >= minTime.getTime()) {
                const overflowWidth = timeToPx(projectStartDate);
                if (overflowWidth > 0) {
                    html += `<div class="gantt-overflow-area" style="position: absolute; left: 0; top: 0; width: ${overflowWidth}px; height: ${gridHeight}px; background: rgba(220, 53, 69, 0.1); z-index: 1; pointer-events: none;"></div>`;
                }
            }

            if (projectDueDate && projectDueDate.getTime() <= gridEnd.getTime()) {
                const deadlinePx = timeToPx(projectDueDate);
                html += `<div class="gantt-deadline-line" style="position: absolute; left: ${deadlinePx}px; top: 0; bottom: 0; width: 2px; background: #dc3545; z-index: 4; pointer-events: none; border-left: 2px dashed #dc3545;"></div>`;
            }

            if (nowPx >= 0 && nowPx <= totalGridWidth) {
                html += `<div class="gantt-now-line" style="position: absolute; left: ${nowPx}px; top: 0; bottom: 0; width: 3px; background: #ffc107; z-index: 6; pointer-events: none; box-shadow: 0 0 4px rgba(255, 193, 7, 0.8);"></div>`;
            }

            timeGrid.forEach((time, index) => {
                if (index === timeGrid.length - 1) return;
                const left = timeToPx(time);
                html += `<div class="gantt-time-division" style="position: absolute; left: ${left}px; top: 0; width: 1px; height: ${gridHeight}px; border-right: 1px solid #dee2e6; pointer-events: none;"></div>`;
            });

            const barPos = new Map();
            let taskBarsHtml = '';

            items.forEach((item, rowIndex) => {
                const leftRaw = timeToPx(item.ls);
                const rightRaw = timeToPx(item.lf);
                const top = rowIndex * taskRowHeight;

                const isOverflow = projectStartDate && item.ls.getTime() < projectStartDate.getTime();

                const clampedRight = Math.min(rightRaw, totalGridWidth);
                const clampedLeft = Math.max(0, leftRaw);
                const left = clampedLeft;
                const width = Math.max(2, clampedRight - clampedLeft);
                const right = left + width;

                barPos.set(item.task.id, {
                    left,
                    right,
                    top,
                    bottom: top + rowHeight,
                    midY: top + (rowHeight / 2),
                    rowIndex
                });

                const status = (item.task.status || '').toLowerCase();
                const progress = item.task.progress || 0;
                const statusLabel = this.humanizeEnum(item.task.status);

                let borderColor = '#6c757d';
                let bgColor = '#e9ecef';
                let fillStyle = 'solid';
                let showProgress = false;
                let badgeText = null;

                if (status === 'planned') {
                    borderColor = '#6c757d';
                    bgColor = '#e9ecef';
                } else if (status === 'inprogress') {
                    borderColor = '#6c757d';
                    bgColor = '#e9ecef';
                    showProgress = true;
                } else if (status === 'needsreview') {
                    borderColor = '#fd7e14';
                    bgColor = '#fefefe';
                    badgeText = 'Review';
                } else if (status === 'accepted') {
                    borderColor = '#198754';
                    bgColor = '#d1e7dd';
                } else if (status === 'rejected') {
                    borderColor = '#dc3545';
                    bgColor = '#e9ecef';
                    fillStyle = 'striped';
                } else if (status === 'blocked') {
                    borderColor = '#000000';
                    bgColor = '#e9ecef';
                    badgeText = 'Blocked';
                    showProgress = true;
                } else if (status === 'done') {
                    borderColor = '#adb5bd';
                    bgColor = '#ffffff';
                }

                if (isOverflow) {
                    borderColor = '#dc3545';
                }

                const borderWidth = item.isCritical ? '3px' : '1px';

                taskBarsHtml += `<div class="gantt-task-bar-wrapper" style="position: absolute; left: ${left}px; top: ${top}px; z-index: 3; display: flex; align-items: center; gap: 4px;">`;
                taskBarsHtml += `<div class="gantt-task-bar" style="width: ${width}px; height: ${rowHeight}px; background: ${bgColor}; border: ${borderWidth} solid ${borderColor}; border-radius: 3px; cursor: pointer; position: relative;" onclick="app.showTaskDetails('${item.task.id}')" title="${this.escapeHtml(item.task.name)} - ${this.escapeHtml(item.ownerName)}${isOverflow ? ' (Overflow)' : ''}">`;
                if (showProgress && progress > 0) {
                    taskBarsHtml += `<div style="position: absolute; left: 0; top: 0; width: ${(progress / 100) * width}px; height: 100%; background: #198754; border-radius: 3px; z-index: 1;"></div>`;
                }
                if (fillStyle === 'striped') {
                    taskBarsHtml += `<div style="position: absolute; left: 0; top: 0; width: 100%; height: 100%; background: repeating-linear-gradient(45deg, transparent, transparent 4px, rgba(0,0,0,0.1) 4px, rgba(0,0,0,0.1) 8px); border-radius: 3px; z-index: 2;"></div>`;
                }
                if (badgeText) {
                    taskBarsHtml += `<div style="position: absolute; right: 2px; top: 1px; font-size: 0.6rem; color: #495057; background: rgba(255,255,255,0.8); padding: 1px 3px; border-radius: 2px; z-index: 3;">${this.escapeHtml(badgeText)}</div>`;
                }
                taskBarsHtml += `</div>`;
                taskBarsHtml += `<div style="font-size: 0.7rem; color: #495057; white-space: nowrap; padding: 0 2px;">${this.escapeHtml(statusLabel)}</div>`;
                taskBarsHtml += `<div style="font-size: 0.7rem; color: #6c757d; white-space: nowrap; padding: 0 2px;">${progress}%</div>`;
                taskBarsHtml += `</div>`;
            });

            if (allDeps.length > 0) {
                let depsSvg = `<svg class="gantt-deps-overlay" width="${totalGridWidth}" height="${gridHeight}" viewBox="0 0 ${totalGridWidth} ${gridHeight}" style="position: absolute; left: 0; top: 0; z-index: 2; pointer-events: none; overflow: visible;">`;
                depsSvg += `
                    <defs>
                        <marker id="gantt-arrowhead-fs" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto" markerUnits="strokeWidth">
                            <path d="M 0 0 L 10 5 L 0 10 z" fill="#6c757d"></path>
                        </marker>
                        <marker id="gantt-arrowhead-ss" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto" markerUnits="strokeWidth">
                            <path d="M 0 0 L 10 5 L 0 10 z" fill="#0d6efd"></path>
                        </marker>
                        <marker id="gantt-arrowhead-ff" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto" markerUnits="strokeWidth">
                            <path d="M 0 0 L 10 5 L 0 10 z" fill="#6f42c1"></path>
                        </marker>
                        <marker id="gantt-arrowhead-sf" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto" markerUnits="strokeWidth">
                            <path d="M 0 0 L 10 5 L 0 10 z" fill="#20c997"></path>
                        </marker>
                    </defs>
                `;

                const getFromId = (dep) => dep.fromTaskId || dep.from_task_id;
                const getToId = (dep) => dep.toTaskId || dep.to_task_id;

                const laneByKey = new Map();
                const laneCount = 8;
                const laneStepX = 10;
                let laneCursor = 0;

                allDeps.forEach(dep => {
                    const fromId = getFromId(dep);
                    const toId = getToId(dep);
                    if (!fromId || !toId) return;

                    const from = barPos.get(fromId);
                    const to = barPos.get(toId);
                    if (!from || !to) return;

                    const depType = (dep.depType || dep.dep_type || 'FS').toUpperCase();

                    const depStyleMap = {
                        FS: { color: '#6c757d', marker: 'gantt-arrowhead-fs' },
                        SS: { color: '#0d6efd', marker: 'gantt-arrowhead-ss' },
                        FF: { color: '#6f42c1', marker: 'gantt-arrowhead-ff' },
                        SF: { color: '#20c997', marker: 'gantt-arrowhead-sf' }
                    };
                    const depStyle = depStyleMap[depType] || depStyleMap.FS;

                    const x1 = from.left;
                    const y1 = from.midY;

                    const x2 = to.left;
                    const y2 = to.midY;

                    const L = 18;
                    const R = 10;
                    const cornerX = Math.max(6, Math.min(x1, x2) - L);

                    const s = y2 > y1;

                    const d = [
                        `M ${x1} ${y1}`,
                        `L ${cornerX} ${y1}`,
                        `L ${cornerX} ${y2}`,
                        `L ${x2 - R} ${y2}`,
                        `L ${x2} ${y2}`
                    ].join(' ');

                    depsSvg += `<path d="${d}" fill="none" stroke="${depStyle.color}" stroke-width="1.5"
    stroke-linejoin="round" stroke-linecap="round"
    marker-end="url(#${depStyle.marker})"></path>`;
                });


                depsSvg += `</svg>`;
                html += depsSvg;
            }

            html += taskBarsHtml;

            html += '</div>';
            html += '</div>';
            html += '</div>';

            container.innerHTML = html;

            this.initGanttResizer();
        } catch (error) {
            console.error('Error rendering Gantt chart:', error);
            const container = document.getElementById('gantt-container');
            if (container) {
                container.innerHTML = '<p class="text-danger">Error rendering Gantt chart. Check console for details.</p>';
            }
        }
    },

    formatLocal(date, opts) {
        return new Intl.DateTimeFormat('en-GB', { ...opts }).format(date);
    },

    initGanttResizer() {
        const resizer = document.getElementById('gantt-resizer');
        const taskList = document.getElementById('gantt-task-list');
        const gridWrapper = document.getElementById('gantt-grid-wrapper');
        
        if (!resizer || !taskList || !gridWrapper) return;
        
        let isResizing = false;
        let startX = 0;
        let startWidth = 0;
        
        resizer.addEventListener('mousedown', (e) => {
            isResizing = true;
            startX = e.clientX;
            startWidth = taskList.offsetWidth;
            document.body.style.cursor = 'col-resize';
            document.body.style.userSelect = 'none';
            e.preventDefault();
        });
        
        document.addEventListener('mousemove', (e) => {
            if (!isResizing) return;
            
            const diff = e.clientX - startX;
            const newWidth = Math.max(150, Math.min(500, startWidth + diff));
            
            taskList.style.width = newWidth + 'px';
        });
        
        document.addEventListener('mouseup', async () => {
            if (isResizing) {
                isResizing = false;
                document.body.style.cursor = '';
                document.body.style.userSelect = '';

                this.ganttTaskListWidth = taskList.offsetWidth;
                
                const tasks = this.currentProjectTasks || [];
                await this.renderGanttChart(tasks);
            }
        });
    },

    showGanttLegend() {
        const legendHtml = `
            <div class="modal fade" id="gantt-legend-modal" tabindex="-1">
                <div class="modal-dialog modal-lg">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title">Gantt Chart Legend</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
                        </div>
                        <div class="modal-body">
                            <h6 class="mb-3">Task Status Colors</h6>
                            <div class="row g-3 mb-4">
                                <div class="col-md-6">
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 1px solid #6c757d; background: #e9ecef; border-radius: 3px; margin-right: 10px;"></div>
                                        <span><strong>Planned</strong></span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 1px solid #6c757d; background: #e9ecef; border-radius: 3px; margin-right: 10px; position: relative;">
                                            <div style="position: absolute; left: 0; top: 0; width: 50%; height: 100%; background: #198754; border-radius: 3px 0 0 3px;"></div>
                                        </div>
                                        <span><strong>In Progress</strong></span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 1px solid #fd7e14; background: #fefefe; border-radius: 3px; margin-right: 10px; position: relative;">
                                            <div style="position: absolute; right: 2px; top: 1px; font-size: 0.5rem; color: #495057; background: rgba(255,255,255,0.8); padding: 1px 2px; border-radius: 2px;">Review</div>
                                        </div>
                                        <span><strong>Needs Review</strong></span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 1px solid #198754; background: #d1e7dd; border-radius: 3px; margin-right: 10px;"></div>
                                        <span><strong>Accepted</strong></span>
                                    </div>
                                </div>
                                <div class="col-md-6">
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 1px solid #dc3545; background: #e9ecef; border-radius: 3px; margin-right: 10px; position: relative;">
                                            <div style="position: absolute; left: 0; top: 0; width: 100%; height: 100%; background: repeating-linear-gradient(45deg, transparent, transparent 3px, rgba(0,0,0,0.1) 3px, rgba(0,0,0,0.1) 6px); border-radius: 3px;"></div>
                                        </div>
                                        <span><strong>Rejected</strong></span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 1px solid #000000; background: #e9ecef; border-radius: 3px; margin-right: 10px; position: relative;">
                                            <div style="position: absolute; right: 2px; top: 1px; font-size: 0.5rem; color: #495057; background: rgba(255,255,255,0.8); padding: 1px 2px; border-radius: 2px;">Blocked</div>
                                        </div>
                                        <span><strong>Blocked</strong></span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 1px solid #adb5bd; background: #ffffff; border-radius: 3px; margin-right: 10px;"></div>
                                        <span><strong>Done</strong></span>
                                    </div>
                                </div>
                            </div>
                            
                            <h6 class="mb-3">Special Indicators</h6>
                            <div class="row g-3 mb-4">
                                <div class="col-md-6">
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 3px solid #6c757d; background: #e9ecef; border-radius: 3px; margin-right: 10px;"></div>
                                        <span><strong>Critical Path</strong></span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 1px solid #dc3545; background: #e9ecef; border-radius: 3px; margin-right: 10px;"></div>
                                        <span><strong>Overflow</strong></span>
                                    </div>
                                </div>
                                <div class="col-md-6">
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 3px; height: 30px; background: #ffc107; box-shadow: 0 0 4px rgba(255, 193, 7, 0.8); margin-right: 10px;"></div>
                                        <span><strong>Current Time</strong></span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 2px; height: 30px; background: #198754; margin-right: 10px;"></div>
                                        <span><strong>Project Start</strong></span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 2px; height: 30px; background: #dc3545; border-left: 2px dashed #dc3545; margin-right: 10px;"></div>
                                        <span><strong>Project Deadline</strong></span>
                                    </div>
                                </div>
                            </div>
                            
                            <h6 class="mb-3">Progress Bar</h6>
                            <div class="mb-2">
                                <p class="mb-1">Progress is shown as a green bar (left to right) for <strong>In Progress</strong> and <strong>Blocked</strong> tasks.</p>
                                <p class="mb-0 text-muted small">The progress bar appears even on overflow (red border) sections, showing actual completion percentage.</p>
                            </div>

                            <hr class="my-4"/>

                            <h6 class="mb-3">Dependencies</h6>
                            <div class="row g-3">
                                <div class="col-md-6">
                                    <div class="d-flex align-items-center mb-2">
                                        <svg width="44" height="16" style="margin-right: 10px; overflow: visible;">
                                            <defs>
                                                <marker id="legend-arrow-fs" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto" markerUnits="strokeWidth">
                                                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#6c757d"></path>
                                                </marker>
                                            </defs>
                                            <path d="M 0 8 L 38 8" stroke="#6c757d" stroke-width="2" marker-end="url(#legend-arrow-fs)"></path>
                                        </svg>
                                        <span><strong>FS</strong> (Finish -> Start)</span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <svg width="44" height="16" style="margin-right: 10px; overflow: visible;">
                                            <defs>
                                                <marker id="legend-arrow-ss" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto" markerUnits="strokeWidth">
                                                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#0d6efd"></path>
                                                </marker>
                                            </defs>
                                            <path d="M 0 8 L 38 8" stroke="#0d6efd" stroke-width="2" marker-end="url(#legend-arrow-ss)"></path>
                                        </svg>
                                        <span><strong>SS</strong> (Start -> Start)</span>
                                    </div>
                                </div>
                                <div class="col-md-6">
                                    <div class="d-flex align-items-center mb-2">
                                        <svg width="44" height="16" style="margin-right: 10px; overflow: visible;">
                                            <defs>
                                                <marker id="legend-arrow-ff" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto" markerUnits="strokeWidth">
                                                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#6f42c1"></path>
                                                </marker>
                                            </defs>
                                            <path d="M 0 8 L 38 8" stroke="#6f42c1" stroke-width="2" marker-end="url(#legend-arrow-ff)"></path>
                                        </svg>
                                        <span><strong>FF</strong> (Finish -> Finish)</span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <svg width="44" height="16" style="margin-right: 10px; overflow: visible;">
                                            <defs>
                                                <marker id="legend-arrow-sf" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto" markerUnits="strokeWidth">
                                                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#20c997"></path>
                                                </marker>
                                            </defs>
                                            <path d="M 0 8 L 38 8" stroke="#20c997" stroke-width="2" marker-end="url(#legend-arrow-sf)"></path>
                                        </svg>
                                        <span><strong>SF</strong> (Start -> Finish)</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Close</button>
                        </div>
                    </div>
                </div>
            </div>
        `;
        
        const existingModal = document.getElementById('gantt-legend-modal');
        if (existingModal) {
            existingModal.remove();
        }
        
        document.body.insertAdjacentHTML('beforeend', legendHtml);
        
        const modal = new bootstrap.Modal(document.getElementById('gantt-legend-modal'));
        modal.show();
        
        document.getElementById('gantt-legend-modal').addEventListener('hidden.bs.modal', function() {
            this.remove();
        });
    }
});
