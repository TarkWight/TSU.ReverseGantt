const API_BASE_URL = '';

const api = {
    async request(endpoint, options = {}) {
        const token = localStorage.getItem('token');
        const headers = {
            'Content-Type': 'application/json',
            ...options.headers,
        };

        if (token) {
            headers['Authorization'] = `Bearer ${token}`;
        }

        const config = {
            ...options,
            headers,
        };

        try {
            const response = await fetch(`${API_BASE_URL}${endpoint}`, config);
            
            // Handle 401 - Unauthorized
            if (response.status === 401) {
                localStorage.removeItem('token');
                localStorage.removeItem('userId');
                localStorage.removeItem('globalRole');
                localStorage.removeItem('userName');
                window.location.hash = '#login';
                app.showToast('Session expired. Please login again.', 'error');
                throw new Error('Unauthorized: Session expired');
            }

            let data;
            const contentType = response.headers.get('content-type');
            if (contentType && contentType.includes('application/json')) {
                data = await response.json();
            } else {
                const text = await response.text();
                data = { message: text || `HTTP ${response.status} ${response.statusText}` };
            }

            if (!response.ok) {
                // Build detailed error message
                let errorMessage = data.message || `HTTP ${response.status}`;
                
                // Add validation errors if present
                if (data.errors && Array.isArray(data.errors)) {
                    errorMessage += ': ' + data.errors.join(', ');
                } else if (data.error) {
                    errorMessage += ': ' + data.error;
                }
                
                // Add status text for context
                if (response.statusText && !errorMessage.includes(response.statusText)) {
                    errorMessage += ` (${response.statusText})`;
                }
                
                const error = new Error(errorMessage);
                error.status = response.status;
                error.data = data;
                error.endpoint = endpoint;
                
                console.error('API Error Details:', {
                    endpoint,
                    status: response.status,
                    statusText: response.statusText,
                    data,
                    error: errorMessage
                });
                
                throw error;
            }

            return data;
        } catch (error) {
            // If it's already our custom error, re-throw it
            if (error.status || error.endpoint) {
                throw error;
            }
            
            // Handle network errors and other fetch errors
            console.error('API Request Error:', {
                endpoint,
                error: error.message,
                stack: error.stack
            });
            
            const detailedError = new Error(`Network error: ${error.message || 'Failed to connect to server'}`);
            detailedError.originalError = error;
            detailedError.endpoint = endpoint;
            throw detailedError;
        }
    },

    // Auth endpoints
    async login(email, password) {
        return this.request('/login', {
            method: 'POST',
            body: JSON.stringify({ email, password }),
        });
    },

    async register(email, name, password) {
        return this.request('/register', {
            method: 'POST',
            body: JSON.stringify({ email, name, password }),
        });
    },

    // Users endpoints
    async getAllUsers() {
        return this.request('/users');
    },

    async getUser(id) {
        return this.request(`/users/${id}`);
    },

    async updateEmailNotifications(enabled) {
        return this.request('/users/me/email-notifications', {
            method: 'PATCH',
            body: JSON.stringify({ enabled }),
        });
    },

    // Projects endpoints
    async getProjects() {
        return this.request('/projects');
    },

    async getProject(id) {
        return this.request(`/projects/${id}`);
    },

    async createProject(project) {
        return this.request('/projects', {
            method: 'POST',
            body: JSON.stringify(project),
        });
    },

    async updateProject(id, project) {
        return this.request(`/projects/${id}`, {
            method: 'PATCH',
            body: JSON.stringify(project),
        });
    },

    async deleteProject(id) {
        return this.request(`/projects/${id}`, {
            method: 'DELETE',
        });
    },

    async getProjectStats(id) {
        return this.request(`/projects/${id}/stats`);
    },

    // Tasks endpoints
    async getTasksByProject(projectId) {
        return this.request(`/projects/${projectId}/tasks`);
    },

    async getTask(id) {
        const taskId = typeof id === 'string' ? id : String(id);
        return this.request(`/tasks/${taskId}`);
    },

    async createTask(projectId, task) {
        return this.request(`/projects/${projectId}/tasks`, {
            method: 'POST',
            body: JSON.stringify(task),
        });
    },

    async updateTask(id, task) {
        return this.request(`/tasks/${id}`, {
            method: 'PATCH',
            body: JSON.stringify(task),
        });
    },

    async deleteTask(id) {
        return this.request(`/tasks/${id}`, {
            method: 'DELETE',
        });
    },

    // Export endpoints
    async exportProjectTasks(projectId) {
        return this.request(`/export/projects/${projectId}/tasks`);
    },

    // Dependencies endpoints
    async getDependencies(taskId) {
        return this.request(`/tasks/${taskId}/dependencies`);
    },

    async createDependency(fromTaskId, dependency) {
        return this.request(`/tasks/${fromTaskId}/dependencies`, {
            method: 'POST',
            body: JSON.stringify(dependency),
        });
    },

    async deleteDependency(taskId, depId) {
        return this.request(`/tasks/${taskId}/dependencies/${depId}`, {
            method: 'DELETE',
        });
    },

    // Artifacts
    async getArtifacts(taskId) {
        return this.request(`/tasks/${taskId}/artifacts`);
    },

    async createArtifact(taskId, payload) {
        return this.request(`/tasks/${taskId}/artifacts`, {
            method: 'POST',
            body: JSON.stringify(payload),
        });
    },

    async deleteArtifact(taskId, artifactId) {
        return this.request(`/tasks/${taskId}/artifacts/${artifactId}`, {
            method: 'DELETE',
        });
    },

    // Memberships endpoints
    async getProjectMemberships(projectId) {
        return this.request(`/memberships/projects/${projectId}`);
    },

    async getUserMemberships() {
        return this.request('/memberships/me');
    },

    async createMembership(projectId, membership) {
        return this.request(`/memberships/projects/${projectId}`, {
            method: 'POST',
            body: JSON.stringify(membership),
        });
    },

    async updateMembershipTags(membershipId, tags) {
        return this.request(`/memberships/${membershipId}/tags`, {
            method: 'PATCH',
            body: JSON.stringify({ tags }),
        });
    },

    async changeProjectLeader(projectId, newLeaderId) {
        return this.request(`/memberships/projects/${projectId}/leader`, {
            method: 'POST',
            body: JSON.stringify({ newLeaderId }),
        });
    },

    async deleteMembership(membershipId) {
        return this.request(`/memberships/${membershipId}`, {
            method: 'DELETE',
        });
    },

    // Assignments endpoints
    async getTaskAssignments(taskId) {
        return this.request(`/assignments/tasks/${taskId}`);
    },

    async getUserAssignments() {
        return this.request('/assignments/me');
    },

    async createAssignment(taskId, assignment) {
        return this.request(`/assignments/tasks/${taskId}`, {
            method: 'POST',
            body: JSON.stringify(assignment),
        });
    },

    async deleteAssignment(assignmentId) {
        return this.request(`/assignments/${assignmentId}`, {
            method: 'DELETE',
        });
    },

    // Schedule endpoints
    async reverseSchedule(projectId) {
        return this.request(`/projects/${projectId}/schedule/reverse`, {
            method: 'POST',
        });
    },

    // Users management endpoints
    async promoteToTeacher(userId) {
        return this.request(`/users/${userId}/promote-to-teacher`, {
            method: 'POST',
        });
    },

    // Review endpoints
    async getReview(taskId) {
        return this.request(`/tasks/${taskId}/review`);
    },

    async createReview(taskId, payload) {
        return this.request(`/tasks/${taskId}/review`, {
            method: 'POST',
            body: JSON.stringify(payload),
        });
    },

    async get(endpoint) {
        return this.request(endpoint);
    },

    async post(endpoint, body = null) {
        return this.request(endpoint, {
            method: 'POST',
            body: body ? JSON.stringify(body) : undefined,
        });
    },

    // Password Reset
    async requestPasswordReset(email) {
        return this.request('/password-reset/request', { 
            method: 'POST', 
            body: JSON.stringify({ email }) 
        });
    },

    async confirmPasswordReset(email, code, newPassword) {
        console.log('[API] Sending password reset confirmation:', { 
            email, 
            codeLength: code.length, 
            passwordLength: newPassword.length 
        });
        return this.request('/password-reset/confirm', { 
            method: 'POST', 
            body: JSON.stringify({ email, code, newPassword }) 
        });
    },

    async changePassword(oldPassword, newPassword) {
        return this.request('/password-reset/change', { 
            method: 'PATCH', 
            body: JSON.stringify({ oldPassword, newPassword }) 
        });
    },

    async requestPasswordResetViaTeacher(email) {
        return this.request('/password-reset/request-teacher', { 
            method: 'POST', 
            body: JSON.stringify({ email }) 
        });
    },

    async getTeacherPasswordResetRequests() {
        return this.request('/password-reset/teacher/requests', { method: 'GET' });
    },

    async teacherApprovePasswordReset(requestId, newPassword, notes) {
        return this.request(`/password-reset/teacher/requests/${requestId}/approve`, { 
            method: 'POST', 
            body: JSON.stringify({ newPassword, notes }) 
        });
    },

    async teacherRejectPasswordReset(requestId, reason) {
        return this.request(`/password-reset/teacher/requests/${requestId}/reject`, { 
            method: 'POST', 
            body: JSON.stringify({ reason }) 
        });
    },

    async teacherSetStudentPassword(studentId, newPassword) {
        return this.request(`/password-reset/teacher/students/${studentId}/set-password`, { 
            method: 'POST', 
            body: JSON.stringify({ newPassword }) 
        });
    },
};

