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

    async renderGanttChart(tasks) {
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
                        const dateStr = project.startDate.split('T')[0]; // "YYYY-MM-DD"
                        const [year, month, day] = dateStr.split('-').map(Number);
                        projectStartDate = new Date(Date.UTC(year, month - 1, day, 0, 0, 0, 0));
                    }
                    if (project.dueDate) {
                        const dateStr = project.dueDate.split('T')[0]; // "YYYY-MM-DD"
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
            
                return { task, ownerName, ownerId };
            });
            
            const tasksWithData = await Promise.all(taskDataPromises);

            const hasSchedule = tasksWithData.some(({ task }) => {
                return task.schedule && task.schedule.ls && task.schedule.lf;
            });

            if (!hasSchedule) {
                container.innerHTML = '<p class="text-muted">Schedule not calculated. Click \'Calculate Reverse Schedule\'.</p>';
                return;
            }

            const items = tasksWithData
                .map(({ task, ownerName, ownerId }) => {
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
            let pxPerHour; // Пикселей на час
            
            if (rangeHours <= 12) {
                timeDivisionHours = 1;
                pxPerHour = 60; // 1 hour = 60px
            } else if (rangeHours <= 48) { // 2 days
                timeDivisionHours = 6;
                pxPerHour = 40; // 1 hour = 40px
            } else if (rangeHours <= 168) { // 7 days
                timeDivisionHours = 12;
                pxPerHour = 30; // 1 hour = 30px
            } else {
                timeDivisionHours = 24;
                pxPerHour = 20; // 1 hour = 20px
            }
            
            const pxPerMs = pxPerHour / (1000 * 60 * 60); // Пикселей на миллисекунду
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

            const divisionWidthPx = timeDivisionHours * pxPerHour;
            const totalGridWidth = timeToPx(gridEnd);

            const rowHeight = 30;
            const rowGap = 4;
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
                const timeLabel = this.formatTimeLabel(time, timeDivisionHours);
                html += `<div style="position: absolute; left: ${leftPx}px; top: 0; width: ${width}px; height: 100%; padding: 8px; text-align: center; border-right: 1px solid #dee2e6; font-size: 0.85rem; background: #f8f9fa; display: flex; align-items: center; justify-content: center;">${this.escapeHtml(timeLabel)}</div>`;
            });
            
            if (nowPx >= 0 && nowPx <= totalGridWidth) {
                const nowLabel = `Now ${this.formatLocal(now, { hour: '2-digit', minute: '2-digit', hour12: false })}`;
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
                const startPx = timeToPx(projectStartDate);
                const overflowWidth = startPx;
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

            items.forEach((item, rowIndex) => {
                const leftRaw = timeToPx(item.ls);
                const rightRaw = timeToPx(item.lf);
                const top = rowIndex * taskRowHeight;
                
                const startPx = projectStartDate ? timeToPx(projectStartDate) : null;
                const isOverflow = projectStartDate && item.ls.getTime() < projectStartDate.getTime();
                
                const clampedRight = Math.min(rightRaw, totalGridWidth);
                const clampedLeft = Math.max(0, leftRaw); // Обрезаем слева до 0, но визуально показываем overflow
                const left = clampedLeft;
                const width = Math.max(2, clampedRight - clampedLeft);
                
                const status = (item.task.status || '').toLowerCase();
                const progress = item.task.progress || 0;
                const statusLabel = this.humanizeEnum(item.task.status);
                
                let borderColor = '#6c757d'; // Серый контур по умолчанию (Planned)
                let bgColor = '#e9ecef'; // Светло-серый фон по умолчанию (Planned)
                let fillStyle = 'solid';
                let showProgress = false;
                let badgeText = null;
                
                if (status === 'planned') {
                    borderColor = '#6c757d'; // Серый контур
                    bgColor = '#e9ecef'; // Светло-серый фон
                } else if (status === 'inprogress') {
                    borderColor = '#6c757d'; // Серый контур (как Planned)
                    bgColor = '#e9ecef'; // Светло-серый фон (как Planned)
                    showProgress = true; // Показываем прогресс
                } else if (status === 'needsreview') {
                    borderColor = '#fd7e14'; // Оранжевый контур
                    bgColor = '#fefefe'; // Светлый (почти белый) фон
                    badgeText = 'Review';
                } else if (status === 'accepted') {
                    borderColor = '#198754'; // Зелёный контур
                    bgColor = '#d1e7dd'; // Очень светло-зелёный фон
                } else if (status === 'rejected') {
                    borderColor = '#dc3545'; // Красно-оранжевый контур
                    bgColor = '#e9ecef'; // Серый фон
                    fillStyle = 'striped'; // Штриховка
                } else if (status === 'blocked') {
                    borderColor = '#000000'; // Чёрный контур
                    bgColor = '#e9ecef'; // Серый фон
                    badgeText = 'Blocked';
                    showProgress = true; // Показываем прогресс, если есть
                } else if (status === 'done') {
                    borderColor = '#adb5bd'; // Светло-серый контур
                    bgColor = '#ffffff'; // Белый фон
                }
                
                if (isOverflow) {
                    borderColor = '#dc3545';
                }
                
                const borderWidth = item.isCritical ? '3px' : '1px';
                
                const right = left + width;
                html += `<div class="gantt-task-bar-wrapper" style="position: absolute; left: ${left}px; top: ${top}px; z-index: 3; display: flex; align-items: center; gap: 4px;">`;
                html += `<div class="gantt-task-bar" style="width: ${width}px; height: ${rowHeight}px; background: ${bgColor}; border: ${borderWidth} solid ${borderColor}; border-radius: 3px; cursor: pointer; position: relative;" onclick="app.showTaskDetails('${item.task.id}')" title="${this.escapeHtml(item.task.name)} - ${this.escapeHtml(item.ownerName)}${isOverflow ? ' (Overflow)' : ''}">`;
                if (showProgress && progress > 0) {
                    html += `<div style="position: absolute; left: 0; top: 0; width: ${(progress / 100) * width}px; height: 100%; background: #198754; border-radius: 3px; z-index: 1;"></div>`;
                }
                if (fillStyle === 'striped') {
                    html += `<div style="position: absolute; left: 0; top: 0; width: 100%; height: 100%; background: repeating-linear-gradient(45deg, transparent, transparent 4px, rgba(0,0,0,0.1) 4px, rgba(0,0,0,0.1) 8px); border-radius: 3px; z-index: 2;"></div>`;
                }
                if (badgeText) {
                    html += `<div style="position: absolute; right: 2px; top: 1px; font-size: 0.6rem; color: #495057; background: rgba(255,255,255,0.8); padding: 1px 3px; border-radius: 2px; z-index: 3;">${this.escapeHtml(badgeText)}</div>`;
                }
                html += `</div>`;
                html += `<div style="font-size: 0.7rem; color: #495057; white-space: nowrap; padding: 0 2px;">${this.escapeHtml(statusLabel)}</div>`;
                html += `<div style="font-size: 0.7rem; color: #6c757d; white-space: nowrap; padding: 0 2px;">${progress}%</div>`;
                html += `</div>`;
            });

            html += '</div>'; // Конец тела грида
            html += '</div>'; // Конец обёртки грида
            html += '</div>'; // Конец всей обёртки

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

    formatTimeLabel(date, divisionHours) {
        if (divisionHours === 1) {
            return this.formatLocal(date, { hour: '2-digit', minute: '2-digit', hour12: false });
        } else if (divisionHours === 6 || divisionHours === 12) {
            return this.formatLocal(date, { day: '2-digit', month: 'short' }) + ' ' +
                   this.formatLocal(date, { hour: '2-digit', minute: '2-digit', hour12: false });
        } else {
            return this.formatLocal(date, { day: '2-digit', month: 'short', year: 'numeric' });
        }
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
                
                const newWidth = taskList.offsetWidth;
                this.ganttTaskListWidth = newWidth;
                
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
                                        <span><strong>Planned</strong> - Gray border, light gray background</span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 1px solid #6c757d; background: #e9ecef; border-radius: 3px; margin-right: 10px; position: relative;">
                                            <div style="position: absolute; left: 0; top: 0; width: 50%; height: 100%; background: #198754; border-radius: 3px 0 0 3px;"></div>
                                        </div>
                                        <span><strong>In Progress</strong> - Gray border, light gray background + green progress bar</span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 1px solid #fd7e14; background: #fefefe; border-radius: 3px; margin-right: 10px; position: relative;">
                                            <div style="position: absolute; right: 2px; top: 1px; font-size: 0.5rem; color: #495057; background: rgba(255,255,255,0.8); padding: 1px 2px; border-radius: 2px;">Review</div>
                                        </div>
                                        <span><strong>Needs Review</strong> - Orange border, light background + "Review" badge</span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 1px solid #198754; background: #d1e7dd; border-radius: 3px; margin-right: 10px;"></div>
                                        <span><strong>Accepted</strong> - Green border, very light green background</span>
                                    </div>
                                </div>
                                <div class="col-md-6">
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 1px solid #dc3545; background: #e9ecef; border-radius: 3px; margin-right: 10px; position: relative;">
                                            <div style="position: absolute; left: 0; top: 0; width: 100%; height: 100%; background: repeating-linear-gradient(45deg, transparent, transparent 3px, rgba(0,0,0,0.1) 3px, rgba(0,0,0,0.1) 6px); border-radius: 3px;"></div>
                                        </div>
                                        <span><strong>Rejected</strong> - Red-orange border, gray background + striped pattern</span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 1px solid #000000; background: #e9ecef; border-radius: 3px; margin-right: 10px; position: relative;">
                                            <div style="position: absolute; right: 2px; top: 1px; font-size: 0.5rem; color: #495057; background: rgba(255,255,255,0.8); padding: 1px 2px; border-radius: 2px;">Blocked</div>
                                        </div>
                                        <span><strong>Blocked</strong> - Black border, gray background + "Blocked" badge</span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 1px solid #adb5bd; background: #ffffff; border-radius: 3px; margin-right: 10px;"></div>
                                        <span><strong>Done</strong> - Light gray border, white background</span>
                                    </div>
                                </div>
                            </div>
                            
                            <h6 class="mb-3">Special Indicators</h6>
                            <div class="row g-3 mb-4">
                                <div class="col-md-6">
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 3px solid #6c757d; background: #e9ecef; border-radius: 3px; margin-right: 10px;"></div>
                                        <span><strong>Critical Path</strong> - Thicker border (3px)</span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 40px; height: 20px; border: 1px solid #dc3545; background: #e9ecef; border-radius: 3px; margin-right: 10px;"></div>
                                        <span><strong>Overflow</strong> - Red border (task starts before project start date)</span>
                                    </div>
                                </div>
                                <div class="col-md-6">
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 3px; height: 30px; background: #ffc107; box-shadow: 0 0 4px rgba(255, 193, 7, 0.8); margin-right: 10px;"></div>
                                        <span><strong>Current Time</strong> - Yellow vertical line with "Now" label</span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 2px; height: 30px; background: #198754; margin-right: 10px;"></div>
                                        <span><strong>Project Start</strong> - Green vertical line</span>
                                    </div>
                                    <div class="d-flex align-items-center mb-2">
                                        <div style="width: 2px; height: 30px; background: #dc3545; border-left: 2px dashed #dc3545; margin-right: 10px;"></div>
                                        <span><strong>Project Deadline</strong> - Red dashed vertical line</span>
                                    </div>
                                </div>
                            </div>
                            
                            <h6 class="mb-3">Progress Bar</h6>
                            <div class="mb-2">
                                <p class="mb-1">Progress is shown as a green bar (left to right) for <strong>In Progress</strong> and <strong>Blocked</strong> tasks.</p>
                                <p class="mb-0 text-muted small">The progress bar appears even on overflow (red border) sections, showing actual completion percentage.</p>
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
