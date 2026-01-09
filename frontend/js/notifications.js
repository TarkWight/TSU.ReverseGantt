const notifications = {
    pollInterval: null,
    
    startPolling() {
        this.fetchNotifications();
        
        this.pollInterval = setInterval(() => {
            this.fetchNotifications();
        }, 30000);
    },
    
    stopPolling() {
        if (this.pollInterval) {
            clearInterval(this.pollInterval);
            this.pollInterval = null;
        }
    },
    
    async fetchNotifications() {
        try {
            const response = await api.get('/notifications?limit=20');
            this.updateUI(response.notifications, response.unreadCount);
        } catch (error) {
            console.error('Failed to fetch notifications:', error);
        }
    },
    
    updateUI(notificationsList, unreadCount) {
        const badge = document.getElementById('notification-badge');
        const list = document.getElementById('notification-list');
        
        if (unreadCount > 0) {
            badge.textContent = unreadCount > 99 ? '99+' : unreadCount;
            badge.style.display = 'block';
        } else {
            badge.style.display = 'none';
        }
        
        if (notificationsList.length === 0) {
            list.innerHTML = `
                <li class="text-center text-muted py-3">
                    <i class="bi bi-inbox"></i> Нет уведомлений
                </li>
            `;
            return;
        }
        
        list.innerHTML = notificationsList.map(n => this.renderNotification(n)).join('');
    },
    
    renderNotification(notification) {
        const isUnread = !notification.isRead;
        const timeAgo = this.formatTimeAgo(notification.createdAt);
        const icon = this.getNotificationIcon(notification.notificationType);
        const bgClass = isUnread ? 'bg-light' : '';
        
        return `
            <li>
                <a class="dropdown-item d-flex align-items-start ${bgClass}" href="#" 
                   onclick="notifications.handleClick('${notification.id}', '${notification.taskId || ''}', '${notification.projectId || ''}'); return false;">
                    <div class="notification-icon me-2 mt-1">
                        ${icon}
                    </div>
                    <div class="flex-grow-1">
                        <div class="notification-title fw-semibold small">${this.escapeHtml(notification.title)}</div>
                        ${notification.message ? `<div class="notification-message text-muted small text-truncate" style="max-width: 250px;">${this.escapeHtml(notification.message)}</div>` : ''}
                        <div class="notification-time text-muted" style="font-size: 0.75rem;">${timeAgo}</div>
                    </div>
                    ${isUnread ? '<span class="badge bg-primary rounded-pill ms-1">new</span>' : ''}
                </a>
            </li>
        `;
    },
    
    getNotificationIcon(type) {
        switch (type) {
            case 'task_submitted':
                return '<i class="bi bi-send text-info fs-5"></i>';
            case 'task_accepted':
                return '<i class="bi bi-check-circle text-success fs-5"></i>';
            case 'task_rejected':
                return '<i class="bi bi-x-circle text-danger fs-5"></i>';
            case 'project_accepted':
                return '<i class="bi bi-trophy text-success fs-5"></i>';
            case 'project_rejected':
                return '<i class="bi bi-exclamation-triangle text-warning fs-5"></i>';
            default:
                return '<i class="bi bi-bell text-secondary fs-5"></i>';
        }
    },
    
    formatTimeAgo(dateStr) {
        const date = new Date(dateStr);
        const now = new Date();
        const diff = Math.floor((now - date) / 1000);
        
        if (diff < 60) return 'только что';
        if (diff < 3600) return `${Math.floor(diff / 60)} мин. назад`;
        if (diff < 86400) return `${Math.floor(diff / 3600)} ч. назад`;
        if (diff < 604800) return `${Math.floor(diff / 86400)} дн. назад`;
        
        return date.toLocaleDateString('ru-RU');
    },
    
    async handleClick(notificationId, taskId, projectId) {
        try {
            await api.post(`/notifications/${notificationId}/read`);
            this.fetchNotifications(); // Refresh
        } catch (error) {
            console.error('Failed to mark notification as read:', error);
        }
        
        if (taskId && projectId) {
            app.currentProjectId = projectId;
            app.showTaskDetails(taskId);
        } else if (projectId) {
            app.showProjectDetails(projectId);
        }
        
        const dropdown = bootstrap.Dropdown.getInstance(document.getElementById('notificationDropdown'));
        if (dropdown) dropdown.hide();
    },
    
    async markAllAsRead() {
        try {
            await api.post('/notifications/read-all');
            this.fetchNotifications(); // Refresh
            app.showToast('Все уведомления прочитаны', 'success');
        } catch (error) {
            console.error('Failed to mark all as read:', error);
            app.showToast('Ошибка при отметке уведомлений', 'danger');
        }
    },
    
    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
};

