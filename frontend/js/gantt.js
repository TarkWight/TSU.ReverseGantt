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

    getGanttDepColors() {
        const rs = getComputedStyle(document.documentElement);
        const pick = (name, fallback) => {
            const v = (rs.getPropertyValue(name) || '').trim();
            return v || fallback;
        };
        return {
            FS: pick('--dep-fs', '#6c757d'),
            SS: pick('--dep-ss', '#0d6efd'),
            FF: pick('--dep-ff', '#6f42c1'),
            SF: pick('--dep-sf', '#20c997')
        };
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

            const dependencyLists = await Promise.all(
                items.map(i => api.getDependencies(i.task.id).catch(() => []))
            );
            const depMap = new Map();
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

            const CELL_PX = 140;
            const MAX_GRID_PX = 2600;

            let timeDivisionHours;
            if (rangeHours <= 12) timeDivisionHours = 1;
            else if (rangeHours <= 48) timeDivisionHours = 6;
            else if (rangeHours <= 168) timeDivisionHours = 12;
            else if (rangeHours <= 24 * 30) timeDivisionHours = 24;
            else timeDivisionHours = 24 * 7;

            const steps = [1, 2, 3, 6, 12, 24, 48, 72, 168, 336, 720];
            const nextStep = (h) => steps.find(s => s > h) || h * 2;

            const columnsCount = (h) => Math.ceil(rangeHours / h);
            while (columnsCount(timeDivisionHours) * CELL_PX > MAX_GRID_PX) {
                timeDivisionHours = nextStep(timeDivisionHours);
                if (timeDivisionHours > 24 * 365) break;
            }

            const pxPerHour = CELL_PX / timeDivisionHours;


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

            const rowHeight = 34;
            const rowGap = 14;
            const taskRowHeight = rowHeight + rowGap;
            const totalRows = items.length;
            const gridHeight = totalRows * taskRowHeight + 20;
            const headerHeight = 60;

            const now = new Date();
            const nowPx = timeToPx(now);

            let html = '<div class="gantt-wrapper">';

            this.ganttTaskListWidth = this.ganttTaskListWidth || 300;
            const taskListWidth = this.ganttTaskListWidth;

            html += `<div id="gantt-task-list" class="gantt-task-list" style="width:${taskListWidth}px;">`;

            html += `<div class="gantt-task-list-header" style="height:${headerHeight}px;">Tasks</div>`;
            html += `<div class="gantt-task-list-body" style="height:${gridHeight}px;">`;

            items.forEach((item, rowIndex) => {
                const top = rowIndex * taskRowHeight;
                html += `
                <div class="gantt-task-item" style="top:${top}px; height:${rowHeight}px; padding-bottom:${rowGap}px;"
                     onclick="app.showTaskDetails('${item.task.id}')">
                  <div class="gantt-task-item-title">${this.escapeHtml(item.task.name)}</div>
                </div>
            `;
            });

            html += `</div>`;
            html += `<div id="gantt-resizer" class="gantt-resizer"></div>`;
            html += `</div>`;

            html += `<div id="gantt-grid-wrapper" class="gantt-grid-wrapper">`;

            html += `<div class="gantt-header" style="height:${headerHeight}px; width:${totalGridWidth}px;">`;
            html += `<div class="gantt-header-row">`;


            timeGrid.forEach((time, index) => {
                if (index === timeGrid.length - 1) return;

                const leftPx = timeToPx(time);
                const rightPx = timeToPx(timeGrid[index + 1]);
                const width = Math.max(1, rightPx - leftPx);

                const timeLabel = (timeDivisionHours >= 24)
                    ? this.formatLocal(time, { year: 'numeric', month: '2-digit', day: '2-digit' })
                    : this.formatLocal(time, { day: '2-digit', month: '2-digit', hour: '2-digit', minute: '2-digit', hour12: false });

                html += `<div class="gantt-time-cell" style="left:${leftPx}px; width:${width}px;">${this.escapeHtml(timeLabel)}</div>`;
            });

            if (nowPx >= 0 && nowPx <= totalGridWidth) {
                const nowLabel = `Now ${this.formatLocal(now, { hour: '2-digit', minute: '2-digit', hour12: false })}`;
                html += `<div class="gantt-vline gantt-now-line-header" style="left:${nowPx}px;"></div>`;
                html += `<div class="gantt-now-label" style="left:${Math.max(0, nowPx - 35)}px;">${this.escapeHtml(nowLabel)}</div>`;
            }

            html += '</div>';
            html += '</div>';

            html += `<div class="gantt-grid-body" style="height:${gridHeight}px; width:${totalGridWidth}px;">`;

            if (projectStartDate && projectStartDate.getTime() >= minTime.getTime() && projectStartDate.getTime() <= gridEnd.getTime()) {
                const startPx = timeToPx(projectStartDate);
                html += `<div class="gantt-vline gantt-start-line" style="left:${startPx}px;"></div>`;
            }

            if (projectStartDate && projectStartDate.getTime() >= minTime.getTime()) {
                const overflowWidth = timeToPx(projectStartDate);
                if (overflowWidth > 0) {
                    html += `<div class="gantt-overflow-area" style="left:0; width:${overflowWidth}px; height:${gridHeight}px;"></div>`;
                }
            }

            if (projectDueDate && projectDueDate.getTime() <= gridEnd.getTime()) {
                const deadlinePx = timeToPx(projectDueDate);
                html += `<div class="gantt-vline gantt-deadline-line" style="left:${deadlinePx}px;"></div>`;
            }

            if (nowPx >= 0 && nowPx <= totalGridWidth) {
                html += `<div class="gantt-vline gantt-deadline-line" style="left:${deadlinePx}px;"></div>`;
            }

            timeGrid.forEach((time, index) => {
                if (index === timeGrid.length - 1) return;
                const left = timeToPx(time);
                html += `<div class="gantt-time-division" style="left:${left}px; height:${gridHeight}px;"></div>`;
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

                const statusRaw = (item.task.status || 'planned').toLowerCase();
                const status = statusRaw || 'planned';

                const progress = item.task.progress || 0;
                const statusLabel = this.humanizeEnum(item.task.status);

                let badgeText = null;
                if (status === 'needsreview') badgeText = 'Review';
                if (status === 'blocked') badgeText = 'Blocked';

                const statusClass = `gantt-task--${status}`;
                const criticalClass = item.isCritical ? 'gantt-task--critical' : '';
                const overflowClass = isOverflow ? 'gantt-task--overflow' : '';
                const stripedClass = (status === 'rejected') ? 'gantt-task--striped' : '';
                const hasProgressClass =
                    ((status === 'inprogress' || status === 'blocked') && progress > 0)
                        ? 'gantt-task--has-progress'
                        : '';

                taskBarsHtml += `
                    <div class="gantt-task-bar-wrapper"
                         style="position:absolute; left:${left}px; top:${top}px; z-index:3; display:flex; align-items:center; gap:4px;">
                      <div class="gantt-task-bar ${statusClass} ${criticalClass} ${overflowClass} ${stripedClass} ${hasProgressClass}"
                           style="width:${width}px; height:${rowHeight}px; --progress:${progress}%;"
                           onclick="app.showTaskDetails('${item.task.id}')"
                           title="${this.escapeHtml(item.task.name)} - ${this.escapeHtml(item.ownerName)}${isOverflow ? ' (Overflow)' : ''}">
                        <div class="gantt-task-progress"></div>
                        ${badgeText ? `<div class="gantt-task-badge">${this.escapeHtml(badgeText)}</div>` : ''}
                      </div>
                
                      <div class="gantt-task-meta gantt-task-meta--status">${this.escapeHtml(statusLabel)}</div>
                      <div class="gantt-task-meta gantt-task-meta--progress">${progress}%</div>
                    </div>
                  `;
            });


            if (allDeps.length > 0) {
                const depColors = this.getGanttDepColors();
                let depsSvg = `
                    <svg class="gantt-deps-overlay"
                         width="${totalGridWidth}"
                         height="${gridHeight}"
                         viewBox="0 0 ${totalGridWidth} ${gridHeight}">
                      <defs>
                        <marker id="gantt-arrowhead-fs" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto" markerUnits="strokeWidth">
                          <path d="M 0 0 L 10 5 L 0 10 z"
                      fill="${depColors.FS}"
                      stroke="${depColors.FS}"
                      stroke-width="1"></path>
                
                        </marker>
                        <marker id="gantt-arrowhead-ss" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto" markerUnits="strokeWidth">
                          <path d="M 0 0 L 10 5 L 0 10 z" fill="${depColors.SS}" stroke="none"></path>
                        </marker>
                        <marker id="gantt-arrowhead-ff" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto" markerUnits="strokeWidth">
                          <path d="M 0 0 L 10 5 L 0 10 z" fill="${depColors.FF}" stroke="none"></path>
                        </marker>
                        <marker id="gantt-arrowhead-sf" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto" markerUnits="strokeWidth">
                          <path d="M 0 0 L 10 5 L 0 10 z" fill="${depColors.SF}" stroke="none"></path>
                        </marker>
                      </defs>
                  `;

                const getFromId = (dep) => dep.fromTaskId || dep.from_task_id;
                const getToId = (dep) => dep.toTaskId || dep.to_task_id;

                allDeps.forEach(dep => {
                    const fromId = getFromId(dep);
                    const toId = getToId(dep);
                    if (!fromId || !toId) return;

                    const from = barPos.get(fromId);
                    const to = barPos.get(toId);
                    if (!from || !to) return;

                    const depType = (dep.depType || dep.dep_type || 'FS').toUpperCase();

                    const depClassMap = {
                        FS: 'gantt-dep--fs',
                        SS: 'gantt-dep--ss',
                        FF: 'gantt-dep--ff',
                        SF: 'gantt-dep--sf'
                    };
                    const depClass = depClassMap[depType] || depClassMap.FS;

                    const x1 = from.left;
                    const y1 = from.midY;

                    const x2 = to.left;
                    const y2 = to.midY;

                    const L = 18;
                    const R = 10;
                    const cornerX = Math.max(6, Math.min(x1, x2) - L);

                    const d = [
                        `M ${x1} ${y1}`,
                        `L ${cornerX} ${y1}`,
                        `L ${cornerX} ${y2}`,
                        `L ${x2 - R} ${y2}`,
                        `L ${x2} ${y2}`
                    ].join(' ');

                    const markerId = `gantt-arrowhead-${depType.toLowerCase()}`;

                    depsSvg += `
                      <path class="gantt-dep-path ${depClass}"
                            d="${d}"
                            stroke="currentColor"
                            marker-end="url(#${markerId})"></path>
                    `;
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
        const depColors = this.getGanttDepColors();
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
                  <div class="gantt-task-bar gantt-legend-sample gantt-task--planned"></div>
                  <span><strong>Planned</strong></span>
                </div>

                <div class="d-flex align-items-center mb-2">
                  <div class="gantt-task-bar gantt-legend-sample gantt-task--inprogress gantt-legend-sample--progress"></div>
                  <span><strong>In Progress</strong></span>
                </div>

                <div class="d-flex align-items-center mb-2">
                  <div class="gantt-task-bar gantt-legend-sample gantt-task--needsreview">
                    <div class="gantt-legend-badge">Review</div>
                  </div>
                  <span><strong>Needs Review</strong></span>
                </div>

                <div class="d-flex align-items-center mb-2">
                  <div class="gantt-task-bar gantt-legend-sample gantt-task--accepted"></div>
                  <span><strong>Accepted</strong></span>
                </div>
              </div>

              <div class="col-md-6">
                <div class="d-flex align-items-center mb-2">
                  <div class="gantt-task-bar gantt-legend-sample gantt-task--rejected gantt-task--striped"></div>
                  <span><strong>Rejected</strong></span>
                </div>

                <div class="d-flex align-items-center mb-2">
                  <div class="gantt-task-bar gantt-legend-sample gantt-task--blocked">
                    <div class="gantt-legend-badge">Blocked</div>
                  </div>
                  <span><strong>Blocked</strong></span>
                </div>

                <div class="d-flex align-items-center mb-2">
                  <div class="gantt-task-bar gantt-legend-sample gantt-task--done"></div>
                  <span><strong>Done</strong></span>
                </div>
              </div>
            </div>

            <h6 class="mb-3">Special Indicators</h6>

            <div class="row g-3 mb-4">
              <div class="col-md-6">
                <div class="d-flex align-items-center mb-2">
                  <div class="gantt-task-bar gantt-legend-sample gantt-task--planned gantt-legend-sample--critical gantt-task--critical"></div>
                  <span><strong>Critical Path</strong></span>
                </div>

                <div class="d-flex align-items-center mb-2">
                  <div class="gantt-task-bar gantt-legend-sample gantt-task--planned gantt-task--overflow"></div>
                  <span><strong>Overflow</strong></span>
                </div>
              </div>

              <div class="col-md-6">
                <div class="d-flex align-items-center mb-2">
                  <div class="gantt-legend-line gantt-legend-line--now"></div>
                  <span><strong>Current Time</strong></span>
                </div>

                <div class="d-flex align-items-center mb-2">
                  <div class="gantt-legend-line gantt-legend-line--start"></div>
                  <span><strong>Project Start</strong></span>
                </div>

                <div class="d-flex align-items-center mb-2">
                  <div class="gantt-legend-line gantt-legend-line--deadline"></div>
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
      <svg class="gantt-legend-dep" viewBox="0 0 44 16">
        <defs>
          <marker id="legend-arrowhead-fs"
                  markerWidth="10"
                  markerHeight="10"
                  refX="8"
                  refY="5"
                  orient="auto"
                  markerUnits="strokeWidth">
            <path d="M 0 0 L 10 5 L 0 10 z"
                  fill="${depColors.FS}"
                  stroke="${depColors.FS}"
                  stroke-width="1"></path>
          </marker>
        </defs>
        <path d="M 0 8 L 38 8"
              stroke="${depColors.FS}"
              stroke-width="2"
              fill="none"
              marker-end="url(#legend-arrowhead-fs)"></path>
      </svg>
      <span><strong>FS</strong> (Finish -> Start)</span>
    </div>

    <div class="d-flex align-items-center mb-2">
      <svg class="gantt-legend-dep" viewBox="0 0 44 16">
        <defs>
          <marker id="legend-arrowhead-ss"
                  markerWidth="10"
                  markerHeight="10"
                  refX="8"
                  refY="5"
                  orient="auto"
                  markerUnits="strokeWidth">
            <path d="M 0 0 L 10 5 L 0 10 z"
                  fill="${depColors.SS}"
                  stroke="${depColors.SS}"
                  stroke-width="1"></path>
          </marker>
        </defs>
        <path d="M 0 8 L 38 8"
              stroke="${depColors.SS}"
              stroke-width="2"
              fill="none"
              marker-end="url(#legend-arrowhead-ss)"></path>
      </svg>
      <span><strong>SS</strong> (Start -> Start)</span>
    </div>

  </div>

  <div class="col-md-6">

    <div class="d-flex align-items-center mb-2">
      <svg class="gantt-legend-dep" viewBox="0 0 44 16">
        <defs>
          <marker id="legend-arrowhead-ff"
                  markerWidth="10"
                  markerHeight="10"
                  refX="8"
                  refY="5"
                  orient="auto"
                  markerUnits="strokeWidth">
            <path d="M 0 0 L 10 5 L 0 10 z"
                  fill="${depColors.FF}"
                  stroke="${depColors.FF}"
                  stroke-width="1"></path>
          </marker>
        </defs>
        <path d="M 0 8 L 38 8"
              stroke="${depColors.FF}"
              stroke-width="2"
              fill="none"
              marker-end="url(#legend-arrowhead-ff)"></path>
      </svg>
      <span><strong>FF</strong> (Finish -> Finish)</span>
    </div>

    <div class="d-flex align-items-center mb-2">
      <svg class="gantt-legend-dep" viewBox="0 0 44 16">
        <defs>
          <marker id="legend-arrowhead-sf"
                  markerWidth="10"
                  markerHeight="10"
                  refX="8"
                  refY="5"
                  orient="auto"
                  markerUnits="strokeWidth">
            <path d="M 0 0 L 10 5 L 0 10 z"
                  fill="${depColors.SF}"
                  stroke="${depColors.SF}"
                  stroke-width="1"></path>
          </marker>
        </defs>
        <path d="M 0 8 L 38 8"
              stroke="${depColors.SF}"
              stroke-width="2"
              fill="none"
              marker-end="url(#legend-arrowhead-sf)"></path>
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
        if (existingModal) existingModal.remove();

        document.body.insertAdjacentHTML('beforeend', legendHtml);

        const modalEl = document.getElementById('gantt-legend-modal');
        const modal = new bootstrap.Modal(modalEl);
        modal.show();

        modalEl.addEventListener('hidden.bs.modal', function () {
            this.remove();
        });
    }
});
