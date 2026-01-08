const auth = {
    init() {
        const token = localStorage.getItem('token');
        if (token) {
            if (jwt.isExpired(token)) {
                this.logout();
                app.showToast('Session expired. Please login again.', 'error');
                return;
            }
            
            const claims = jwt.getClaims(token);
            if (claims && claims.userId && claims.globalRole && claims.name) {
                localStorage.setItem('userId', claims.userId);
                localStorage.setItem('globalRole', claims.globalRole);
                localStorage.setItem('userName', claims.name);
                app.setUserInfo(claims.userId, claims.globalRole, claims.name);
                app.showProjects();
            } else {
                this.logout();
            }
        } else {
            app.showLogin();
        }
    },

    async login(email, password) {
        try {
            const response = await api.login(email, password);
            
            const claims = jwt.getClaims(response.token);
            if (!claims) {
                throw new Error('Failed to decode token');
            }
            
            localStorage.setItem('token', response.token);
            localStorage.setItem('userId', claims.userId);
            localStorage.setItem('globalRole', claims.globalRole);
            localStorage.setItem('userName', claims.name);
            
            app.setUserInfo(claims.userId, claims.globalRole, claims.name);
            app.showProjects();
            app.showToast('Login successful!', 'success');
        } catch (error) {
            const errorMsg = app.formatError(error);
            app.showToast('Login failed', 'error', error);
            console.error('Login error:', error);
            throw error;
        }
    },

    async register(email, name, password, passwordConfirm) {
        if (password !== passwordConfirm) {
            app.showToast('Passwords do not match', 'error');
            return;
        }

        if (!this.validatePassword(password)) {
            return;
        }

        try {
            const response = await api.register(email, name, password);
            
            const claims = jwt.getClaims(response.token);
            if (!claims) {
                throw new Error('Failed to decode token');
            }
            
            localStorage.setItem('token', response.token);
            localStorage.setItem('userId', claims.userId);
            localStorage.setItem('globalRole', claims.globalRole);
            localStorage.setItem('userName', claims.name);
            
            app.setUserInfo(claims.userId, claims.globalRole, claims.name);
            app.showProjects();
            app.showToast('Registration successful!', 'success');
        } catch (error) {
            const errorMsg = app.formatError(error);
            app.showToast('Registration failed', 'error', error);
            console.error('Registration error:', error);
            throw error;
        }
    },

    validatePassword(password) {
        if (password.length < 8) {
            app.showToast('Password must be at least 8 characters long', 'error');
            return false;
        }

        const hasLower = /[a-z]/.test(password);
        const hasUpper = /[A-Z]/.test(password);
        const hasDigit = /[0-9]/.test(password);
        const hasSpecial = /[^a-zA-Z0-9\s]/.test(password);

        if (!hasLower) {
            app.showToast('Password must contain at least one lowercase letter', 'error');
            return false;
        }
        if (!hasUpper) {
            app.showToast('Password must contain at least one uppercase letter', 'error');
            return false;
        }
        if (!hasDigit) {
            app.showToast('Password must contain at least one digit', 'error');
            return false;
        }
        if (!hasSpecial) {
            app.showToast('Password must contain at least one special character', 'error');
            return false;
        }

        return true;
    },

    logout() {
        localStorage.removeItem('token');
        localStorage.removeItem('userId');
        localStorage.removeItem('globalRole');
        localStorage.removeItem('userName');
        app.showLogin();
        app.showToast('Logged out successfully', 'success');
    },

    isAuthenticated() {
        return !!localStorage.getItem('token');
    },

    getUserId() {
        return localStorage.getItem('userId');
    },

    getGlobalRole() {
        return localStorage.getItem('globalRole');
    },

    isTeacher() {
        return this.getGlobalRole() === 'teacher';
    },

    isStudent() {
        return this.getGlobalRole() === 'student';
    },
};

