// Password visibility toggle functionality
Object.assign(app, {
    /**
     * Initialize password visibility toggle for a password input field
     * @param {string} inputId - ID of the password input field
     */
    initPasswordToggle(inputId) {
        const input = document.getElementById(inputId);
        if (!input) return;

        const container = input.parentElement;
        if (!container.classList.contains('password-input-container')) {
            // Wrap input in container if not already wrapped
            const wrapper = document.createElement('div');
            wrapper.className = 'password-input-container position-relative';
            input.parentNode.insertBefore(wrapper, input);
            wrapper.appendChild(input);
            
            // Add toggle button
            const toggleBtn = document.createElement('button');
            toggleBtn.type = 'button';
            toggleBtn.className = 'btn btn-sm password-toggle-btn';
            toggleBtn.innerHTML = '<i class="bi bi-eye"></i>';
            toggleBtn.setAttribute('aria-label', 'Toggle password visibility');
            toggleBtn.onclick = () => this.togglePasswordVisibility(inputId);
            wrapper.appendChild(toggleBtn);
        }
    },

    /**
     * Toggle password visibility for a field
     * @param {string} inputId - ID of the password input field
     */
    togglePasswordVisibility(inputId) {
        const input = document.getElementById(inputId);
        if (!input) return;

        const container = input.parentElement;
        const toggleBtn = container.querySelector('.password-toggle-btn');
        const icon = toggleBtn.querySelector('i');

        if (input.type === 'password') {
            input.type = 'text';
            icon.className = 'bi bi-eye-slash';
            toggleBtn.setAttribute('aria-label', 'Hide password');
        } else {
            input.type = 'password';
            icon.className = 'bi bi-eye';
            toggleBtn.setAttribute('aria-label', 'Show password');
        }
    },

    /**
     * Initialize all password toggles on current page
     */
    initAllPasswordToggles() {
        const passwordInputs = document.querySelectorAll('input[type="password"]');
        passwordInputs.forEach(input => {
            if (input.id) {
                this.initPasswordToggle(input.id);
            }
        });
    }
});

