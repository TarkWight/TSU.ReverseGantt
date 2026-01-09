Object.assign(app, {
    async showCreateTask() {
        document.getElementById('task-form').reset();
        document.getElementById('task-form-id').value = '';
        document.getElementById('task-modal-title').textContent = 'Create Task';
        
        document.getElementById('task-form-status-group').style.display = 'none';
        document.getElementById('task-form-dates-group').style.display = 'none';
        
        document.getElementById('task-form-task-type').required = true;
        document.getElementById('task-form-priority').required = true;
        
        await this.populateParentTaskDropdown();
        
        const ownerGroup = document.getElementById('task-form-owner-group');
        if (auth.isTeacher() || app.isProjectLeader()) {
            ownerGroup.style.display = 'block';
            document.getElementById('task-form-owner-id').required = true;
            
            await this.populateOwnerDropdown();
        } else {
            ownerGroup.style.display = 'none';
            document.getElementById('task-form-owner-id').required = false;
        }
        
        const hardnessGroup = document.getElementById('task-form-hardness-group');
        if (auth.isTeacher() || app.isProjectLeader()) {
            hardnessGroup.style.display = 'block';
        } else {
            hardnessGroup.style.display = 'none';
        }
        
        const modal = new bootstrap.Modal(document.getElementById('task-modal'));
        modal.show();
    },

    async populateOwnerDropdown() {
        const select = document.getElementById('task-form-owner-id');
        select.innerHTML = '<option value="">Select owner...</option>';
        
        if (this.currentProjectMembers) {
            const memberPromises = this.currentProjectMembers.map(async (member) => {
                try {
                    const user = await api.getUser(member.userId);
                    return { userId: member.userId, userName: user.name };
                } catch (error) {
                    console.error(`Failed to load user ${member.userId}:`, error);
                    return { userId: member.userId, userName: member.userId }; // Fallback to ID
                }
            });
            
            const membersWithNames = await Promise.all(memberPromises);
            
            membersWithNames.forEach(({ userId, userName }) => {
                const option = document.createElement('option');
                option.value = userId;
                option.textContent = userName;
                select.appendChild(option);
            });
        }
    },

    async populateParentTaskDropdown() {
        const select = document.getElementById('task-form-parent-task-id');
        select.innerHTML = '<option value="">None (root task)</option>';
        
        if (this.currentProjectTasks) {
            this.currentProjectTasks.forEach(task => {
                const option = document.createElement('option');
                option.value = task.id;
                option.textContent = task.name;
                select.appendChild(option);
            });
        }
    },

    async showEditTask(taskId) {
        try {
            const task = await api.getTask(taskId);
            
            document.getElementById('task-form-id').value = task.id;
            document.getElementById('task-form-name').value = task.name;
            document.getElementById('task-form-description').value = task.description || '';
            document.getElementById('task-form-task-type').value = task.taskType || '';
            document.getElementById('task-form-priority').value = task.priority || '';
            document.getElementById('task-form-estimated-duration').value = task.estimatedDuration ?
                Math.round(task.estimatedDuration / 3600) : '';
            document.getElementById('task-form-hardness').value = task.hardness || '';
            document.getElementById('task-form-buffer').value = task.buffer ?
                Math.round(task.buffer / 3600) : 0;
            
            await this.populateParentTaskDropdown();
            if (task.parentTaskId) {
                document.getElementById('task-form-parent-task-id').value = task.parentTaskId;
            }
            
            document.getElementById('task-form-status-group').style.display = 'block';
            document.getElementById('task-form-dates-group').style.display = 'block';
            document.getElementById('task-form-status').value = task.status || 'planned';
            document.getElementById('task-form-progress').value = task.progress || 0;
            
            document.getElementById('task-modal-title').textContent = 'Edit Task';
            
            document.getElementById('task-form-task-type').required = false;
            document.getElementById('task-form-priority').required = false;
            
            const ownerGroup = document.getElementById('task-form-owner-group');
            if (auth.isTeacher() || app.isProjectLeader()) {
                ownerGroup.style.display = 'block';
                document.getElementById('task-form-owner-id').required = true;
                
                await this.populateOwnerDropdown();
                
                try {
                    const assignments = await api.getTaskAssignments(task.id);
                    const ownerAssignment = assignments.find(a => a.role === 'owner');
                    if (ownerAssignment) {
                        document.getElementById('task-form-owner-id').value = ownerAssignment.userId;
                    }
                } catch (error) {
                    console.error('Failed to load current owner:', error);
                }
            } else {
                ownerGroup.style.display = 'none';
                document.getElementById('task-form-owner-id').required = false;
            }
            
            const hardnessGroup = document.getElementById('task-form-hardness-group');
            if (auth.isTeacher() || app.isProjectLeader()) {
                hardnessGroup.style.display = 'block';
                document.getElementById('task-form-hardness').value = task.hardness || 'soft';
            } else {
                hardnessGroup.style.display = 'none';
            }
            
            const modal = new bootstrap.Modal(document.getElementById('task-modal'));
            modal.show();
        } catch (error) {
            const errorMsg = app.formatError(error);
            app.showToast('Failed to load task', 'error', error);
            console.error('Error loading task for edit:', error);
        }
    },

    async saveTask() {
        const form = document.getElementById('task-form');
        if (!form.checkValidity()) {
            form.reportValidity();
            return;
        }

        const id = document.getElementById('task-form-id').value;
        const isEdit = !!id;
        
        if (isEdit) {
            const estimatedDurationHours = document.getElementById('task-form-estimated-duration').value;
            const bufferHours = document.getElementById('task-form-buffer').value;
            
            const task = {
                name: document.getElementById('task-form-name').value,
                description: document.getElementById('task-form-description').value || null,
                taskType: document.getElementById('task-form-task-type').value || null,
                priority: document.getElementById('task-form-priority').value || null,
                status: document.getElementById('task-form-status').value || null,
                estimatedDuration: estimatedDurationHours ? 
                    parseInt(estimatedDurationHours) * 3600 : null, // Convert hours to seconds
                hardness: document.getElementById('task-form-hardness-group').style.display !== 'none' ? 
                    (document.getElementById('task-form-hardness').value || null) : null,
                buffer: bufferHours ? 
                    parseInt(bufferHours) * 3600 : null, // Convert hours to seconds
                progress: document.getElementById('task-form-progress').value ? 
                    parseInt(document.getElementById('task-form-progress').value) : null,
            };
            
            try {
                console.debug('[tasks] update payload', task);
                await api.updateTask(id, task);
                
                if (auth.isTeacher() || app.isProjectLeader()) {
                    const newOwnerId = document.getElementById('task-form-owner-id').value;
                    if (newOwnerId) {
                        try {
                            const assignments = await api.getTaskAssignments(id);
                            const currentOwner = assignments.find(a => a.role === 'owner');
                            
                            if (!currentOwner || currentOwner.userId !== newOwnerId) {
                                if (currentOwner) {
                                    await api.deleteAssignment(currentOwner.id);
                                }
                                
                                await api.createAssignment(id, {
                                    userId: newOwnerId,
                                    role: 'owner'
                                });
                                
                                console.log(`[tasks] Owner updated: ${currentOwner?.userId} -> ${newOwnerId}`);
                            }
                        } catch (error) {
                            console.error('Failed to update task owner:', error);
                            app.showToast('Task updated but failed to change owner', 'warning');
                        }
                    }
                }
                
                app.showToast('Task updated successfully', 'success');
                bootstrap.Modal.getInstance(document.getElementById('task-modal')).hide();
                app.showTaskDetails(id);
            } catch (error) {
                const errorMsg = app.formatError(error);
                app.showToast('Failed to update task', 'error', errorMsg);
                console.error('Error updating task:', error);
            }
        } else {
            const estimatedDurationHours = document.getElementById('task-form-estimated-duration').value;
            const bufferHours = document.getElementById('task-form-buffer').value;
            
            const task = {
                name: document.getElementById('task-form-name').value,
                description: document.getElementById('task-form-description').value || null,
                taskType: document.getElementById('task-form-task-type').value,
                priority: document.getElementById('task-form-priority').value,
                estimatedDuration: estimatedDurationHours ? 
                    parseInt(estimatedDurationHours) * 3600 : null, // Convert hours to seconds
                hardness: document.getElementById('task-form-hardness-group').style.display !== 'none' ? 
                    (document.getElementById('task-form-hardness').value || null) : null,
                buffer: bufferHours ? 
                    parseInt(bufferHours) * 3600 : null, // Convert hours to seconds
                parentTaskId: document.getElementById('task-form-parent-task-id').value || null,
            };

            if (auth.isTeacher() || app.isProjectLeader()) {
                const ownerId = document.getElementById('task-form-owner-id').value;
                if (!ownerId) {
                    app.showToast('Please specify a task owner', 'error');
                    return;
                }
                task.ownerId = ownerId;
            }

            try {
                console.debug('[tasks] create payload', task);
                await api.createTask(app.currentProjectId, task);
                app.showToast('Task created successfully', 'success');
                bootstrap.Modal.getInstance(document.getElementById('task-modal')).hide();
                await app.loadProjectTasks();
                await app.showProjectTab('tasks');
            } catch (error) {
                const errorMsg = app.formatError(error);
                app.showToast('Failed to create task', 'error', errorMsg);
                console.error('Error creating task:', error);
            }
        }
    },

    async deleteTask(taskId) {
        if (!confirm('Are you sure you want to delete this task? All subtasks and dependencies will also be deleted.')) {
            return;
        }

        try {
            await api.deleteTask(taskId);
            app.showToast('Task deleted successfully', 'success');
            app.showProjectDetails(app.currentProjectId);
        } catch (error) {
            if (error.status === 403 || error.message.includes('403') || error.message.includes('Forbidden')) {
                app.showToast('You do not have permission to delete this task', 'error');
            } else {
                const errorMsg = app.formatError(error);
                app.showToast('Failed to delete task', 'error', error);
                console.error('Error deleting task:', error);
            }
        }
    },

    showAddDependency(taskId) {
        const select = document.getElementById('dependency-form-to-task');
        select.innerHTML = '<option value="">Select task...</option>';
        
        if (this.currentProjectTasks) {
            this.currentProjectTasks.forEach(task => {
                if (task.id !== taskId) {
                    const option = document.createElement('option');
                    option.value = task.id;
                    option.textContent = task.name;
                    select.appendChild(option);
                }
            });
        }
        
        document.getElementById('dependency-form').dataset.fromTaskId = taskId;
        
        const modal = new bootstrap.Modal(document.getElementById('dependency-modal'));
        modal.show();
    },

    async addDependency() {
        const form = document.getElementById('dependency-form');
        if (!form.checkValidity()) {
            form.reportValidity();
            return;
        }

        const fromTaskId = form.dataset.fromTaskId;
        const dependency = {
            toTaskId: document.getElementById('dependency-form-to-task').value,
            depType: document.getElementById('dependency-form-type').value,
            minGap: parseInt(document.getElementById('dependency-form-min-gap').value) || 0,
        };

        try {
            await api.createDependency(fromTaskId, dependency);
            app.showToast('Dependency added successfully', 'success');
            bootstrap.Modal.getInstance(document.getElementById('dependency-modal')).hide();
            app.loadTaskDependencies(fromTaskId);
        } catch (error) {
            const errorMsg = app.formatError(error);
            app.showToast('Failed to add dependency', 'error', error);
            console.error('Error adding dependency:', error);
        }
    },

    formatDateTimeForAPI(dateTimeLocal) {
        if (!dateTimeLocal) return null;
        const date = new Date(dateTimeLocal);
        if (isNaN(date.getTime())) return null;
        return date.toISOString();
    },

    async removeDependency(taskId, depId) {
        if (!confirm('Are you sure you want to remove this dependency?')) {
            return;
        }

        try {
            await api.deleteDependency(taskId, depId);
            app.showToast('Dependency removed successfully', 'success');
            app.loadTaskDependencies(taskId);
        } catch (error) {
            const errorMsg = app.formatError(error);
            app.showToast('Failed to remove dependency', 'error', error);
            console.error('Error removing dependency:', error);
        }
    },
});

