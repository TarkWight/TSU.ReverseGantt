Object.assign(app, {
    async showCreateProject() {
        document.getElementById('project-form').reset();
        document.getElementById('project-form-id').value = '';
        document.getElementById('project-modal-title').textContent = 'Create Project';
        
        const leaderGroup = document.getElementById('project-form-leader-group');
        if (auth.isTeacher()) {
            leaderGroup.style.display = 'block';
            document.getElementById('project-form-leader-id').required = true;
            
            await this.populateLeaderDropdown();
        } else {
            leaderGroup.style.display = 'none';
            document.getElementById('project-form-leader-id').required = false;
        }
        
        const modal = new bootstrap.Modal(document.getElementById('project-modal'));
        modal.show();
    },

    async populateLeaderDropdown() {
        const select = document.getElementById('project-form-leader-id');
        select.innerHTML = '<option value="">Loading students...</option>';
        
        try {
            const users = await api.getAllUsers();
            const students = users.filter(u => u.globalRole === 'student');
            
            select.innerHTML = '<option value="">Select student...</option>';
            students.forEach(student => {
                const option = document.createElement('option');
                option.value = student.id;
                option.textContent = `${student.name} (${student.email})`;
                select.appendChild(option);
            });
            
            if (students.length === 0) {
                select.innerHTML = '<option value="">No students available</option>';
                select.disabled = true;
            } else {
                select.disabled = false;
            }
        } catch (error) {
            console.error('Failed to load students:', error);
            select.innerHTML = '<option value="">Failed to load students</option>';
            select.disabled = true;
        }
    },

    showEditProject(projectId) {
        api.getProject(projectId).then(project => {
            document.getElementById('project-form-id').value = project.id;
            document.getElementById('project-form-name').value = project.name;
            document.getElementById('project-form-description').value = project.description || '';
            document.getElementById('project-form-start-date').value = project.startDate ? project.startDate.split('T')[0] : '';
            document.getElementById('project-form-due-date').value = project.dueDate ? project.dueDate.split('T')[0] : '';
            
            document.getElementById('project-modal-title').textContent = 'Edit Project';
            
            document.getElementById('project-form-leader-group').style.display = 'none';
            
            const modal = new bootstrap.Modal(document.getElementById('project-modal'));
            modal.show();
        }).catch(error => {
            const errorMsg = app.formatError(error);
            app.showToast('Failed to load project', 'error', error);
            console.error('Error loading project for edit:', error);
        });
    },

    async saveProject() {
        const form = document.getElementById('project-form');
        if (!form.checkValidity()) {
            form.reportValidity();
            return;
        }

        const id = document.getElementById('project-form-id').value;
        const project = {
            name: document.getElementById('project-form-name').value,
            description: document.getElementById('project-form-description').value,
            startDate: document.getElementById('project-form-start-date').value || null,
            dueDate: document.getElementById('project-form-due-date').value,
        };

        if (auth.isTeacher() && !id) {
            const leaderId = document.getElementById('project-form-leader-id').value;
            if (!leaderId) {
                app.showToast('Please specify a leader', 'error');
                return;
            }
            project.leaderId = leaderId;
        }

        try {
            if (id) {
                await api.updateProject(id, project);
                app.showToast('Project updated successfully', 'success');
            } else {
                await api.createProject(project);
                app.showToast('Project created successfully', 'success');
            }
            
            bootstrap.Modal.getInstance(document.getElementById('project-modal')).hide();
            
            if (id) {
                app.showProjectDetails(id);
            } else {
                app.showProjects();
            }
        } catch (error) {
            const errorMsg = app.formatError(error);
            app.showToast('Failed to save project', 'error', error);
            console.error('Error saving project:', error);
        }
    },

    async deleteProject(projectId) {
        if (!confirm('Are you sure you want to delete this project? This action cannot be undone.')) {
            return;
        }

        try {
            await api.deleteProject(projectId);
            app.showToast('Project deleted successfully', 'success');
            app.showProjects();
        } catch (error) {
            if (error.status === 403 || error.message.includes('403') || error.message.includes('Forbidden')) {
                app.showToast('You do not have permission to delete this project', 'error');
            } else {
                const errorMsg = app.formatError(error);
                app.showToast('Failed to delete project', 'error', error);
                console.error('Error deleting project:', error);
            }
        }
    },
});

