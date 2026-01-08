Object.assign(app, {
    async showAddMember() {
        document.getElementById('member-form').reset();
        
        const select = document.getElementById('member-form-user-id');
        select.innerHTML = '<option value="">Loading users...</option>';
        
        try {
            const users = await api.getAllUsers();
            
            const currentMemberIds = new Set((this.currentProjectMembers || []).map(m => m.userId));
            const availableUsers = users.filter(u => !currentMemberIds.has(u.id));
            
            select.innerHTML = '<option value="">Select a user...</option>';
            availableUsers.forEach(user => {
                const option = document.createElement('option');
                option.value = user.id;
                option.textContent = `${user.name} (${user.email})`;
                select.appendChild(option);
            });
            
            if (availableUsers.length === 0) {
                select.innerHTML = '<option value="">No available users</option>';
                select.disabled = true;
            } else {
                select.disabled = false;
            }
        } catch (error) {
            console.error('Failed to load users:', error);
            select.innerHTML = '<option value="">Failed to load users</option>';
            select.disabled = true;
        }
        
        const modal = new bootstrap.Modal(document.getElementById('member-modal'));
        modal.show();
    },

    async addMember() {
        const form = document.getElementById('member-form');
        if (!form.checkValidity()) {
            form.reportValidity();
            return;
        }

        const userId = document.getElementById('member-form-user-id').value;
        const tagsStr = document.getElementById('member-form-tags').value;
        const tags = tagsStr ? tagsStr.split(',').map(t => t.trim()).filter(t => t) : [];

        const membership = {
            userId: userId,
            tags: tags.length > 0 ? tags : undefined,
        };

        try {
            await api.createMembership(app.currentProjectId, membership);
            app.showToast('Member added successfully', 'success');
            bootstrap.Modal.getInstance(document.getElementById('member-modal')).hide();
            
            await app.loadProjectMembers();
            await app.showProjectTab('members');
        } catch (error) {
            const errorMsg = app.formatError(error);
            app.showToast('Failed to add member', 'error', error);
            console.error('Error adding member:', error);
        }
    },

    async removeMember(membershipId) {
        if (!confirm('Are you sure you want to remove this member from the project?')) {
            return;
        }

        try {
            await api.deleteMembership(membershipId);
            app.showToast('Member removed successfully', 'success');
            
            // Reload members
            await app.loadProjectMembers();
            await app.showProjectTab('members');
        } catch (error) {
            if (error.message.includes('Cannot remove the only leader') || 
                (error.data && error.data.message && error.data.message.includes('only leader'))) {
                app.showToast('Cannot remove the only project leader', 'error');
            } else {
                const errorMsg = app.formatError(error);
                app.showToast('Failed to remove member', 'error', error);
                console.error('Error removing member:', error);
            }
        }
    },

    async makeLeader(userId) {
        if (!confirm('Are you sure you want to make this user the project leader? You will lose leader status if you are currently the leader.')) {
            return;
        }

        try {
            await api.changeProjectLeader(app.currentProjectId, userId.toString());
            app.showToast('Project leader changed successfully', 'success');
            
            await app.loadProjectMembers();
            await app.showProjectTab('members');
        } catch (error) {
            const errorMsg = app.formatError(error);
            app.showToast('Failed to change leader', 'error', error);
            console.error('Error changing leader:', error);
        }
    },
});

