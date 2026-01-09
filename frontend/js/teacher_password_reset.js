// Teacher-specific password reset functionality
Object.assign(app, {
    async showTeacherPasswordResetRequests() {
        if (!auth.isTeacher()) {
            this.showToast('Access denied: Teachers only', 'error');
            return;
        }

        const modalHtml = `
            <div class="modal fade" id="teacher-password-reset-modal" tabindex="-1">
                <div class="modal-dialog modal-lg">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title"><i class="bi bi-key-fill"></i> Password Reset Requests</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
                        </div>
                        <div class="modal-body">
                            <div id="password-reset-requests-list">
                                <div class="text-center"><div class="spinner-border" role="status"></div></div>
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Close</button>
                        </div>
                    </div>
                </div>
            </div>
        `;

        // Remove existing modal if any
        const existing = document.getElementById('teacher-password-reset-modal');
        if (existing) existing.remove();

        document.body.insertAdjacentHTML('beforeend', modalHtml);
        const modal = new bootstrap.Modal(document.getElementById('teacher-password-reset-modal'));
        modal.show();

        try {
            const requests = await api.getTeacherPasswordResetRequests();
            this.renderPasswordResetRequests(requests);
        } catch (error) {
            document.getElementById('password-reset-requests-list').innerHTML = `
                <div class="alert alert-danger">
                    <i class="bi bi-exclamation-triangle"></i> Failed to load requests
                </div>
            `;
            this.showToast('Failed to load password reset requests', 'error', error);
            console.error('Error loading password reset requests:', error);
        }
    },

    renderPasswordResetRequests(requests) {
        const container = document.getElementById('password-reset-requests-list');

        if (!requests || requests.length === 0) {
            container.innerHTML = `
                <div class="alert alert-info">
                    <i class="bi bi-info-circle"></i> No pending password reset requests
                </div>
            `;
            return;
        }

        const html = requests.map(req => `
            <div class="card mb-3">
                <div class="card-body">
                    <div class="d-flex justify-content-between align-items-start">
                        <div>
                            <h6 class="card-title mb-1">${this.escapeHtml(req.userName)}</h6>
                            <p class="text-muted mb-2"><small>${this.escapeHtml(req.userEmail)}</small></p>
                            <p class="text-muted mb-0"><small>Requested: ${new Date(req.createdAt).toLocaleString()}</small></p>
                        </div>
                        <div>
                            <button class="btn btn-sm btn-success me-2" onclick="app.approvePasswordResetRequest('${req.id}', '${this.escapeHtml(req.userName)}');">
                                <i class="bi bi-check-circle"></i> Approve
                            </button>
                            <button class="btn btn-sm btn-danger" onclick="app.rejectPasswordResetRequest('${req.id}', '${this.escapeHtml(req.userName)}');">
                                <i class="bi bi-x-circle"></i> Reject
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        `).join('');

        container.innerHTML = html;
    },

    async approvePasswordResetRequest(requestId, userName) {
        const newPassword = prompt(`Enter new password for ${userName}:`);
        if (!newPassword) return;

        if (!auth.validatePassword(newPassword)) {
            return;
        }

        const notes = prompt('Optional notes (visible to student):');

        try {
            await api.teacherApprovePasswordReset(requestId, newPassword, notes || undefined);
            this.showToast('Password reset approved', 'success');
            this.showTeacherPasswordResetRequests(); // Refresh list
        } catch (error) {
            this.showToast('Failed to approve password reset', 'error', error);
            console.error('Error approving password reset:', error);
        }
    },

    async rejectPasswordResetRequest(requestId, userName) {
        const reason = prompt(`Reason for rejecting ${userName}'s request:`);
        if (!reason) return;

        try {
            await api.teacherRejectPasswordReset(requestId, reason);
            this.showToast('Password reset request rejected', 'success');
            this.showTeacherPasswordResetRequests(); // Refresh list
        } catch (error) {
            this.showToast('Failed to reject password reset', 'error', error);
            console.error('Error rejecting password reset:', error);
        }
    },

    async showTeacherSetStudentPassword(studentId, studentName) {
        if (!auth.isTeacher()) {
            this.showToast('Access denied: Teachers only', 'error');
            return;
        }

        const newPassword = prompt(`Enter new password for ${studentName}:`);
        if (!newPassword) return;

        if (!auth.validatePassword(newPassword)) {
            return;
        }

        try {
            await api.teacherSetStudentPassword(studentId, newPassword);
            this.showToast(`Password set for ${studentName}`, 'success');
        } catch (error) {
            this.showToast('Failed to set student password', 'error', error);
            console.error('Error setting student password:', error);
        }
    },
});

