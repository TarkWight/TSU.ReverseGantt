Object.assign(app, {
    showForgotPassword() {
        this.currentView = 'forgotPassword';
        document.getElementById('app-content').innerHTML = templates.forgotPasswordPage;
        
        // Initialize password toggles
        setTimeout(() => {
            this.initPasswordToggle('new-password');
            this.initPasswordToggle('new-password-confirm');
        }, 0);
        
        document.getElementById('forgot-password-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            const email = document.getElementById('forgot-email').value;
            
            if (!email || !email.includes('@')) {
                this.showToast('Please enter a valid email address', 'error');
                return;
            }
            
            try {
                await api.requestPasswordReset(email);
                this.showToast('Reset code sent to your email (if account exists). Check your inbox!', 'success');
                
                // Show step 2
                document.getElementById('forgot-password-step-1').style.display = 'none';
                document.getElementById('forgot-password-step-2').style.display = 'block';
                
                // Store email for step 2
                this._forgotPasswordEmail = email;
                
                // Focus on code input
                setTimeout(() => {
                    const codeInput = document.getElementById('reset-code');
                    if (codeInput) codeInput.focus();
                }, 100);
            } catch (error) {
                this.showToast('Failed to send reset code', 'error', error);
                console.error('Error requesting password reset:', error);
            }
        });
        
        document.getElementById('confirm-reset-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            const code = document.getElementById('reset-code').value;
            const newPassword = document.getElementById('new-password').value;
            const newPasswordConfirm = document.getElementById('new-password-confirm').value;
            
            if (newPassword !== newPasswordConfirm) {
                this.showToast('Passwords do not match', 'error');
                return;
            }
            
            if (!auth.validatePassword(newPassword)) {
                return;
            }
            
            try {
                await api.confirmPasswordReset(this._forgotPasswordEmail, code, newPassword);
                this.showToast('Password reset successfully! Please login with your new password.', 'success');
                
                // Clear the email and reload login page
                delete this._forgotPasswordEmail;
                setTimeout(() => {
                    window.location.hash = '#login';
                    window.location.reload();
                }, 1500);
            } catch (error) {
                this.showToast('Failed to reset password', 'error', error);
                console.error('Error confirming password reset:', error);
            }
        });
    },

    showNoEmailAccess() {
        this.currentView = 'noEmailAccess';
        document.getElementById('app-content').innerHTML = templates.noEmailAccessPage;
        
        document.getElementById('no-email-access-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            const email = document.getElementById('no-email-email').value;
            
            try {
                await api.requestPasswordResetViaTeacher(email);
                this.showToast('Request sent to teachers for approval', 'success');
                this.showLogin();
            } catch (error) {
                this.showToast('Failed to send request', 'error', error);
                console.error('Error requesting teacher password reset:', error);
            }
        });
    },

    async showChangePasswordModal() {
        const modalHtml = `
            <div class="modal fade" id="change-password-modal" tabindex="-1">
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title"><i class="bi bi-key"></i> Change Password</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
                        </div>
                        <div class="modal-body">
                            <form id="change-password-form">
                                <div class="mb-3">
                                    <label for="current-password" class="form-label">Current Password</label>
                                    <input type="password" class="form-control" id="current-password" required>
                                </div>
                                <div class="mb-3">
                                    <label for="change-new-password" class="form-label">New Password</label>
                                    <input type="password" class="form-control" id="change-new-password" required>
                                    <div class="form-text">Must be at least 8 characters with uppercase, lowercase, digit, and special character</div>
                                </div>
                                <div class="mb-3">
                                    <label for="change-new-password-confirm" class="form-label">Confirm New Password</label>
                                    <input type="password" class="form-control" id="change-new-password-confirm" required>
                                </div>
                            </form>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancel</button>
                            <button type="button" class="btn btn-primary" onclick="app.submitChangePassword();">Change Password</button>
                        </div>
                    </div>
                </div>
            </div>
        `;
        
        // Remove existing modal if any
        const existing = document.getElementById('change-password-modal');
        if (existing) existing.remove();
        
        document.body.insertAdjacentHTML('beforeend', modalHtml);
        const modal = new bootstrap.Modal(document.getElementById('change-password-modal'));
        modal.show();
        
        // Initialize password toggles after modal is shown
        modal._element.addEventListener('shown.bs.modal', () => {
            this.initPasswordToggle('current-password');
            this.initPasswordToggle('change-new-password');
            this.initPasswordToggle('change-new-password-confirm');
        }, { once: true });
    },

    async submitChangePassword() {
        const oldPassword = document.getElementById('current-password').value;
        const newPassword = document.getElementById('change-new-password').value;
        const newPasswordConfirm = document.getElementById('change-new-password-confirm').value;
        
        if (newPassword !== newPasswordConfirm) {
            this.showToast('Passwords do not match', 'error');
            return;
        }
        
        if (!auth.validatePassword(newPassword)) {
            return;
        }
        
        try {
            await api.changePassword(oldPassword, newPassword);
            this.showToast('Password changed successfully!', 'success');
            
            const modal = bootstrap.Modal.getInstance(document.getElementById('change-password-modal'));
            modal.hide();
        } catch (error) {
            this.showToast('Failed to change password', 'error', error);
            console.error('Error changing password:', error);
        }
    },
});

