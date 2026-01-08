Object.assign(app, {
    async exportTasks(format) {
        if (!this.currentProjectId) {
            this.showToast('No project selected', 'error');
            return;
        }

        try {
            const project = await api.getProject(this.currentProjectId);
            const tasks = await api.exportProjectTasks(this.currentProjectId);
            
            const tasksWithOwners = await Promise.all(tasks.map(async (task) => {
                let ownerName = 'Unassigned';
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
                return { ...task, ownerName };
            }));

            switch (format) {
                case 'html': this.exportToHTML(project, tasksWithOwners); break;
                case 'pdf': this.exportToPDF(project, tasksWithOwners); break;
                case 'csv': this.exportToCSV(project, tasksWithOwners); break;
                default: this.showToast('Unknown export format', 'error');
            }
        } catch (error) {
            this.showToast('Failed to export tasks', 'error', error);
            console.error('Export error:', error);
        }
    },

    exportToHTML(project, tasks) {
        const html = `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${this.escapeHtml(project.name)} - Tasks Export</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; color: #333; }
        h1 { color: #0d6efd; border-bottom: 2px solid #0d6efd; padding-bottom: 10px; }
        .project-info { background: #f8f9fa; padding: 15px; border-radius: 5px; margin-bottom: 20px; }
        table { width: 100%; border-collapse: collapse; margin-top: 20px; }
        th, td { border: 1px solid #dee2e6; padding: 10px; text-align: left; }
        th { background: #0d6efd; color: white; font-weight: bold; }
        tr:nth-child(even) { background: #f8f9fa; }
        .critical { background: #dc3545 !important; color: white; }
        .badge { padding: 4px 8px; border-radius: 4px; font-size: 0.85em; }
        .badge-critical { background: #dc3545; color: white; }
        .badge-planned { background: #6c757d; color: white; }
        .badge-inprogress { background: #0d6efd; color: white; }
        .badge-done { background: #198754; color: white; }
        .badge-needsreview { background: #ffc107; color: #000; }
        @media print { body { margin: 0; } .no-print { display: none; } }
    </style>
</head>
<body>
    <h1>${this.escapeHtml(project.name)}</h1>
    <div class="project-info">
        <p><strong>Description:</strong> ${this.escapeHtml(project.description || 'N/A')}</p>
        <p><strong>Due Date:</strong> ${project.dueDate ? this.formatDate(new Date(project.dueDate)) : 'N/A'}</p>
        <p><strong>Export Date:</strong> ${this.formatDate(new Date())}</p>
        <p><strong>Total Tasks:</strong> ${tasks.length}</p>
    </div>
    <table>
        <thead>
            <tr><th>Name</th><th>Description</th><th>Type</th><th>Status</th><th>Priority</th><th>Owner</th><th>Start Date</th><th>Finish Date</th><th>Duration</th><th>Slack</th><th>Critical</th></tr>
        </thead>
        <tbody>
            ${tasks.map(task => {
                const startDate = task.schedule?.ls ? this.formatDate(new Date(task.schedule.ls)) : 'N/A';
                const finishDate = task.schedule?.lf ? this.formatDate(new Date(task.schedule.lf)) : 'N/A';
                const duration = task.estimatedDuration ? `${Math.round(task.estimatedDuration / 3600)}h` : 'N/A';
                const slack = task.schedule?.slack ? `${Math.round(task.schedule.slack / 3600)}h` : 'N/A';
                const isCritical = task.schedule?.isCritical || false;
                const statusClass = (task.status || '').toLowerCase().replace('_', '');
                return `<tr ${isCritical ? 'class="critical"' : ''}>
                    <td><strong>${this.escapeHtml(task.name)}</strong></td>
                    <td>${this.escapeHtml(task.description || '')}</td>
                    <td>${this.escapeHtml(task.taskType || 'Task')}</td>
                    <td><span class="badge badge-${statusClass}">${this.escapeHtml(this.humanizeEnum(task.status))}</span></td>
                    <td>${this.escapeHtml(this.humanizeEnum(task.priority))}</td>
                    <td>${this.escapeHtml(task.ownerName || 'Unassigned')}</td>
                    <td>${startDate}</td><td>${finishDate}</td><td>${duration}</td><td>${slack}</td>
                    <td>${isCritical ? '<span class="badge badge-critical">Critical</span>' : '-'}</td>
                </tr>`;
            }).join('')}
        </tbody>
    </table>
</body>
</html>`;

        const blob = new Blob([html], { type: 'text/html' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `${project.name.replace(/[^a-z0-9]/gi, '_')}_tasks_${new Date().toISOString().split('T')[0]}.html`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
        this.showToast('Tasks exported to HTML', 'success');
    },

    exportToPDF(project, tasks) {
        const html = this.generateHTMLForPDF(project, tasks);
        const printWindow = window.open('', '_blank');
        printWindow.document.write(html);
        printWindow.document.close();
        printWindow.onload = () => setTimeout(() => printWindow.print(), 250);
    },

    generateHTMLForPDF(project, tasks) {
        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>${this.escapeHtml(project.name)} - Tasks Export</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; color: #333; }
        h1 { color: #0d6efd; border-bottom: 2px solid #0d6efd; padding-bottom: 10px; }
        .project-info { background: #f8f9fa; padding: 15px; border-radius: 5px; margin-bottom: 20px; }
        table { width: 100%; border-collapse: collapse; margin-top: 20px; font-size: 10px; }
        th, td { border: 1px solid #dee2e6; padding: 6px; text-align: left; }
        th { background: #0d6efd; color: white; font-weight: bold; }
        tr:nth-child(even) { background: #f8f9fa; }
        .critical { background: #dc3545 !important; color: white; }
        @media print { body { margin: 0; } @page { margin: 1cm; } }
    </style>
</head>
<body>
    <h1>${this.escapeHtml(project.name)}</h1>
    <div class="project-info">
        <p><strong>Description:</strong> ${this.escapeHtml(project.description || 'N/A')}</p>
        <p><strong>Due Date:</strong> ${project.dueDate ? this.formatDate(new Date(project.dueDate)) : 'N/A'}</p>
        <p><strong>Export Date:</strong> ${this.formatDate(new Date())}</p>
        <p><strong>Total Tasks:</strong> ${tasks.length}</p>
    </div>
    <table>
        <thead><tr><th>Name</th><th>Status</th><th>Owner</th><th>Start</th><th>Finish</th><th>Duration</th><th>Critical</th></tr></thead>
        <tbody>
            ${tasks.map(task => {
                const startDate = task.schedule?.ls ? this.formatDate(new Date(task.schedule.ls)) : 'N/A';
                const finishDate = task.schedule?.lf ? this.formatDate(new Date(task.schedule.lf)) : 'N/A';
                const duration = task.estimatedDuration ? `${Math.round(task.estimatedDuration / 3600)}h` : 'N/A';
                const isCritical = task.schedule?.isCritical || false;
                return `<tr ${isCritical ? 'class="critical"' : ''}>
                    <td><strong>${this.escapeHtml(task.name)}</strong></td>
                    <td>${this.escapeHtml(this.humanizeEnum(task.status))}</td>
                    <td>${this.escapeHtml(task.ownerName || 'Unassigned')}</td>
                    <td>${startDate}</td><td>${finishDate}</td><td>${duration}</td>
                    <td>${isCritical ? 'Yes' : 'No'}</td>
                </tr>`;
            }).join('')}
        </tbody>
    </table>
</body>
</html>`;
    },

    exportToCSV(project, tasks) {
        const headers = ['Name', 'Description', 'Type', 'Status', 'Priority', 'Owner', 'Start Date', 'Finish Date', 'Duration (hours)', 'Slack (hours)', 'Critical', 'Progress (%)', 'Buffer (hours)'];

        const rows = tasks.map(task => {
            const startDate = task.schedule?.ls ? this.formatDate(new Date(task.schedule.ls)) : '';
            const finishDate = task.schedule?.lf ? this.formatDate(new Date(task.schedule.lf)) : '';
            const duration = task.estimatedDuration ? Math.round(task.estimatedDuration / 3600) : '';
            const slack = task.schedule?.slack ? Math.round(task.schedule.slack / 3600) : '';
            const buffer = task.buffer ? Math.round(task.buffer / 3600) : '';
            
            return [
                this.escapeCsvField(task.name || ''),
                this.escapeCsvField(task.description || ''),
                this.escapeCsvField(task.taskType || ''),
                this.escapeCsvField(this.humanizeEnum(task.status)),
                this.escapeCsvField(this.humanizeEnum(task.priority)),
                this.escapeCsvField(task.ownerName || 'Unassigned'),
                this.escapeCsvField(startDate),
                this.escapeCsvField(finishDate),
                duration, slack,
                task.schedule?.isCritical ? 'Yes' : 'No',
                task.progress || 0, buffer
            ];
        });

        const csvContent = [headers.join(','), ...rows.map(row => row.join(','))].join('\n');

        const blob = new Blob(['\ufeff' + csvContent], { type: 'text/csv;charset=utf-8;' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `${project.name.replace(/[^a-z0-9]/gi, '_')}_tasks_${new Date().toISOString().split('T')[0]}.csv`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
        this.showToast('Tasks exported to CSV', 'success');
    }
});

