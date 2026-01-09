Object.assign(app, {
    async showSettings() {
        try {
            const userId = localStorage.getItem('userId');
            const user = await api.getUser(userId);
            
            const toggle = document.getElementById('email-notifications-toggle');
            if (toggle) {
                toggle.checked = user.emailNotificationsEnabled !== false && user.emailNotificationsEnabled !== undefined;
            }
            
            const modal = new bootstrap.Modal(document.getElementById('settings-modal'));
            modal.show();
        } catch (error) {
            this.showToast('Failed to load settings', 'error', error);
            console.error('Error loading settings:', error);
        }
    },

    async saveSettings() {
        try {
            const toggle = document.getElementById('email-notifications-toggle');
            const enabled = toggle ? toggle.checked : true;
            
            await api.updateEmailNotifications(enabled);
            this.showToast(
                enabled ? 'Email notifications enabled' : 'Email notifications disabled',
                'success'
            );
            
            const modal = bootstrap.Modal.getInstance(document.getElementById('settings-modal'));
            modal.hide();
        } catch (error) {
            this.showToast('Failed to save settings', 'error', error);
            console.error('Error saving settings:', error);
        }
    },

    async showChangePasswordInSettings() {
        const modal = bootstrap.Modal.getInstance(document.getElementById('settings-modal'));
        if (modal) modal.hide();
        await this.showChangePasswordModal();
    }
});

