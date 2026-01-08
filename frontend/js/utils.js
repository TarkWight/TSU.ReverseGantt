const utils = {
    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    },

    humanizeEnum(value) {
        if (!value) return '-';
        const spaced = value.replace(/([A-Z])/g, ' $1');
        return spaced.charAt(0).toUpperCase() + spaced.slice(1);
    },

    formatDate(dateString) {
        if (!dateString) return '-';
        const date = new Date(dateString);
        return date.toLocaleDateString();
    },

    formatError(error) {
        if (!error) return 'Unknown error occurred';
        
        if (error.status && error.data) {
            let message = error.message || 'Request failed';
            if (error.data.errors && Array.isArray(error.data.errors)) {
                message += '\nValidation errors:\n' + error.data.errors.map(e => `  • ${e}`).join('\n');
            }
            if (error.endpoint) {
                message += `\n\nEndpoint: ${error.endpoint}`;
            }
            return message;
        }
        
        if (error.originalError) {
            return `${error.message}\n\nOriginal error: ${error.originalError.message || 'Network connection failed'}`;
        }
        
        return error.message || String(error);
    },

    showToast(message, type = 'info', details = null) {
        const toast = document.getElementById('toast');
        const toastBody = document.getElementById('toast-body');
        const toastTitle = document.getElementById('toast-title');
        
        let fullMessage = message;
        if (details) {
            if (typeof details === 'string') {
                fullMessage += '\n' + details;
            } else if (details.status) {
                fullMessage += ` (Status: ${details.status})`;
            }
        }
        
        toastBody.textContent = fullMessage;
        toastTitle.textContent = type === 'error' ? 'Error' : type === 'success' ? 'Success' : 'Info';
        
        toast.className = `toast ${type === 'error' ? 'bg-danger text-white' : type === 'success' ? 'bg-success text-white' : ''}`;
        
        toastBody.style.maxHeight = '200px';
        toastBody.style.overflowY = 'auto';
        toastBody.style.whiteSpace = 'pre-wrap';
        
        const bsToast = new bootstrap.Toast(toast, { delay: type === 'error' ? 8000 : 5000 });
        bsToast.show();
    },

    escapeCsvField(field) {
        if (field === null || field === undefined) return '';
        const str = String(field);
        if (str.includes(',') || str.includes('"') || str.includes('\n') || str.includes('\r')) {
            return `"${str.replace(/"/g, '""')}"`;
        }
        return str;
    }
};

Object.assign(app, {
    escapeHtml: utils.escapeHtml.bind(utils),
    humanizeEnum: utils.humanizeEnum.bind(utils),
    formatDate: utils.formatDate.bind(utils),
    formatError: utils.formatError.bind(utils),
    showToast: utils.showToast.bind(utils),
    escapeCsvField: utils.escapeCsvField.bind(utils)
});

