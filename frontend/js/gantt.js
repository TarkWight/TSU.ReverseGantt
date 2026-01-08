Object.assign(app, {
    async renderGanttTab() {
        const tabContent = document.getElementById('project-tab-content');
        const tasks = this.currentProjectTasks || [];
        
        let html = '<div class="d-flex justify-content-between align-items-center mb-3">';
        html += '<h5>Gantt Chart</h5>';
        html += '</div>';
        html += '<div id="gantt-container" class="gantt-container"></div>';
        
        tabContent.innerHTML = html;
        await this.renderGanttChart(tasks);
    },

    async renderGanttChart(tasks) {
        const container = document.getElementById('gantt-container');
        if (!container) return;
        if (!tasks || tasks.length === 0) {
            container.innerHTML = '<p class="text-muted">No tasks to display</p>';
            return;
        }

        const taskDataPromises = tasks.map(async (task) => {
            let ownerName = 'Unassigned';
            let dependencies = [];
            
            try {
                const assignments = await api.getTaskAssignments(task.id);
                const ownerAssignment = assignments.find(a => a.role === 'owner');
                if (ownerAssignment) {
                    const user = await api.getUser(ownerAssignment.userId);
                    ownerName = user.name;
                }
            } catch (error) {
                console.error(`Failed to load owner for task ${task.id}:`, error);
            }
            
            try {
                dependencies = await api.getDependencies(task.id);
            } catch (error) {
                console.error(`Failed to load dependencies for task ${task.id}:`, error);
            }
            
            return { task, ownerName, dependencies };
        });
        
        const tasksWithData = await Promise.all(taskDataPromises);

        const items = tasksWithData.map(({ task, ownerName, dependencies }) => {
            const startStr = task.schedule && task.schedule.ls;
            const finishStr = task.schedule && task.schedule.lf;
            if (!startStr || !finishStr) return null;
            const start = new Date(startStr);
            const finish = new Date(finishStr);
            if (isNaN(start.getTime()) || isNaN(finish.getTime())) return null;
            return { task, ownerName, dependencies, start, finish };
        }).filter(Boolean).sort((a, b) => a.start - b.start || a.finish - b.finish);

        if (items.length === 0) {
            container.innerHTML = '<p class="text-muted">No schedule data available</p>';
            return;
        }

        const minStart = new Date(Math.min(...items.map(i => i.start.getTime())));
        const maxFinish = new Date(Math.max(...items.map(i => i.finish.getTime())));
        const rangeMs = Math.max(1, maxFinish.getTime() - minStart.getTime());

        const daysRange = rangeMs / (1000 * 60 * 60 * 24);
        let divisionHours = 1;
        if (daysRange > 7) {
            divisionHours = 24;
        } else if (daysRange > 2) {
            divisionHours = 12;
        } else if (daysRange > 0.5) {
            divisionHours = 6;
        }

        const timeMarkers = [];
        const divisionMs = divisionHours * 60 * 60 * 1000;
        let currentTime = new Date(minStart);
        currentTime.setMinutes(0, 0, 0);
        
        const hoursToSubtract = currentTime.getHours() % divisionHours;
        currentTime.setHours(currentTime.getHours() - hoursToSubtract);
        
        while (currentTime <= maxFinish) {
            const position = ((currentTime.getTime() - minStart.getTime()) / rangeMs) * 100;
            if (position >= 0 && position <= 100) {
                timeMarkers.push({ time: new Date(currentTime), position });
            }
            currentTime = new Date(currentTime.getTime() + divisionMs);
        }

        const depTypeColors = {
            'FS': '#0d6efd',
            'FF': '#198754',
            'SS': '#ffc107',
            'SF': '#dc3545'
        };

        const dependencyArrows = [];
        items.forEach((item, itemIndex) => {
            const { task, dependencies } = item;
            if (!dependencies || dependencies.length === 0) return;
            
            dependencies.forEach(dep => {
                const isOutgoing = dep.fromTaskId === task.id || dep.from_task_id === task.id;
                const otherTaskId = isOutgoing ? (dep.toTaskId || dep.to_task_id) : (dep.fromTaskId || dep.from_task_id);
                
                const otherItemIndex = items.findIndex(i => 
                    i.task.id === otherTaskId || i.task.id.toString() === otherTaskId.toString()
                );
                
                if (otherItemIndex !== -1 && otherItemIndex !== itemIndex) {
                    const otherItem = items[otherItemIndex];
                    const fromItem = isOutgoing ? item : otherItem;
                    const toItem = isOutgoing ? otherItem : item;
                    
                    const depType = dep.depType || dep.dep_type || 'FS';
                    const color = depTypeColors[depType] || '#6c757d';
                    
                    const fromX = ((fromItem.finish.getTime() - minStart.getTime()) / rangeMs) * 100;
                    const toX = ((toItem.start.getTime() - minStart.getTime()) / rangeMs) * 100;
                    const fromY = isOutgoing ? itemIndex : otherItemIndex;
                    const toY = isOutgoing ? otherItemIndex : itemIndex;
                    
                    dependencyArrows.push({ fromX, toX, fromY, toY, depType, color });
                }
            });
        });

        let html = `
            <div class="row">
                <div class="col-md-9">
                    <div class="d-flex justify-content-between align-items-center mb-2">
                        <h6 class="mb-0">Gantt Chart</h6>
                        <div class="small text-muted">${this.formatDate(minStart)} → ${this.formatDate(maxFinish)}</div>
                    </div>
                    <div class="gantt-chart" style="position: relative;">
                        <div class="gantt-time-scale" style="position: relative; height: 30px; border-bottom: 2px solid #dee2e6; margin-bottom: 10px;">`;

        timeMarkers.forEach(marker => {
            const label = this.formatTimeScaleLabel(marker.time, divisionHours);
            html += `
                <div class="gantt-time-marker" style="position: absolute; left: ${marker.position}%; border-left: 1px solid #adb5bd; height: 100%;">
                    <div class="gantt-time-label" style="position: absolute; top: 5px; left: 2px; font-size: 10px; color: #6c757d; white-space: nowrap;">
                        ${this.escapeHtml(label)}
                    </div>
                </div>`;
        });

        html += `</div>
                        <svg class="gantt-dependencies-layer" style="position: absolute; top: 30px; left: 0; width: 100%; height: calc(100% - 30px); pointer-events: none; z-index: 1;">
                            <defs>`;

        Object.keys(depTypeColors).forEach(depType => {
            html += `<marker id="arrowhead-${depType}" markerWidth="10" markerHeight="10" refX="9" refY="3" orient="auto">
                        <polygon points="0 0, 10 3, 0 6" fill="${depTypeColors[depType]}" />
                     </marker>`;
        });

        html += `</defs>`;

        dependencyArrows.forEach(arrow => {
            const rowHeight = 42, barHeight = 22;
            const fromY = arrow.fromY * rowHeight + barHeight / 2;
            const toY = arrow.toY * rowHeight + barHeight / 2;
            const midX = (arrow.fromX + arrow.toX) / 2;
            const curveOffset = Math.abs(arrow.toX - arrow.fromX) * 0.3;
            const controlY = (fromY + toY) / 2;
            const path = `M ${arrow.fromX} ${fromY} Q ${midX} ${controlY - curveOffset}, ${arrow.toX} ${toY}`;
            
            html += `<path d="${path}" stroke="${arrow.color}" stroke-width="2" fill="none" marker-end="url(#arrowhead-${arrow.depType})" opacity="0.7" />
                     <text x="${midX}" y="${controlY - curveOffset - 8}" font-size="9" fill="${arrow.color}" text-anchor="middle" font-weight="bold">${this.escapeHtml(arrow.depType)}</text>`;
        });

        html += `</svg>`;

        items.forEach(({ task, ownerName, start, finish }) => {
            const leftPct = ((start.getTime() - minStart.getTime()) / rangeMs) * 100;
            const widthPct = Math.max(1.5, ((finish.getTime() - start.getTime()) / rangeMs) * 100);
            const isCritical = task.schedule && task.schedule.isCritical;
            const slackSeconds = (task.schedule && task.schedule.slack) || 0;
            const slackHours = slackSeconds ? Math.round(slackSeconds / 3600) : 0;
            const tooltip = [
                `Task: ${task.name}`, `Owner: ${ownerName}`,
                start ? `Start: ${this.formatDate(start)}` : '',
                finish ? `Finish: ${this.formatDate(finish)}` : '',
                slackHours ? `Slack: ${slackHours}h` : '',
                isCritical ? 'Critical path' : ''
            ].filter(Boolean).join(' | ');

            html += `
                <div class="gantt-row" style="position: relative; z-index: 2;">
                    <div class="gantt-bar ${isCritical ? 'critical' : ''}" style="left:${leftPct}%; width:${widthPct}%" title="${this.escapeHtml(tooltip)}">
                        <span class="gantt-bar-label">${this.escapeHtml(task.name)} <small class="text-light opacity-75">(${this.escapeHtml(ownerName)})</small></span>
                    </div>
                    <div class="gantt-row-caption">
                        ${this.escapeHtml(this.formatDate(start))} → ${this.escapeHtml(this.formatDate(finish))}
                        ${isCritical ? '<span class="badge bg-danger ms-2">Critical</span>' : ''}
                        ${slackHours ? `<span class="badge bg-secondary ms-2">Slack: ${slackHours}h</span>` : ''}
                    </div>
                </div>`;
        });

        html += `</div></div>
                <div class="col-md-3">
                    <div class="card">
                        <div class="card-header"><h6 class="mb-0">Legend</h6></div>
                        <div class="card-body">
                            <div class="mb-3">
                                <strong>Task Colors:</strong>
                                <div class="mt-2">
                                    <div class="d-flex align-items-center mb-2">
                                        <div class="gantt-bar" style="width: 20px; height: 16px; position: relative; left: 0; margin-right: 8px;"></div>
                                        <span class="small">Normal Task</span>
                                    </div>
                                    <div class="d-flex align-items-center">
                                        <div class="gantt-bar critical" style="width: 20px; height: 16px; position: relative; left: 0; margin-right: 8px;"></div>
                                        <span class="small">Critical Path</span>
                                    </div>
                                </div>
                            </div>
                            <div>
                                <strong>Dependency Types:</strong>
                                <div class="mt-2">`;

        const depTypeNames = { 'FS': 'Finish-to-Start', 'FF': 'Finish-to-Finish', 'SS': 'Start-to-Start', 'SF': 'Start-to-Finish' };
        Object.entries(depTypeColors).forEach(([depType, color]) => {
            html += `<div class="d-flex align-items-center mb-2">
                        <div style="width: 20px; height: 3px; background: ${color}; margin-right: 8px;"></div>
                        <span class="small"><strong>${depType}</strong>: ${depTypeNames[depType]}</span>
                     </div>`;
        });

        html += `</div></div></div></div></div></div>`;
        container.innerHTML = html;
    },

    formatTimeScaleLabel(date, divisionHours) {
        if (divisionHours >= 24) {
            return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
        } else if (divisionHours >= 12) {
            return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' }) + ' ' + 
                   date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' });
        }
        return date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' });
    }
});

