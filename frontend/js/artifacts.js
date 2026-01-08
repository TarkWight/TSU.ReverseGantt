Object.assign(app, {
    async loadTaskArtifacts(taskId, { canEdit }) {
        try {
            const artifacts = await api.getArtifacts(taskId);
            const listDiv = document.getElementById('artifacts-list');
            const formContainer = document.getElementById('artifacts-form-container');

            if (!artifacts || artifacts.length === 0) {
                listDiv.innerHTML = '<p class="text-muted">No artifacts yet.</p>';
            } else {
                let html = '<ul class="list-group">';
                artifacts.forEach(a => {
                    html += `<li class="list-group-item d-flex justify-content-between align-items-center">
                        <div>
                            <strong>${this.escapeHtml(a.name)}</strong>
                            <div class="small text-muted">${this.escapeHtml(a.kind)}</div>
                            <div><a href="${this.escapeHtml(a.uri)}" target="_blank" rel="noopener">${this.escapeHtml(a.uri)}</a></div>
                        </div>
                        ${canEdit ? `<button class="btn btn-sm btn-outline-danger" onclick="app.deleteArtifact('${taskId}', '${a.id}')"><i class="bi bi-trash"></i></button>` : ''}
                    </li>`;
                });
                html += '</ul>';
                listDiv.innerHTML = html;
            }

            if (canEdit) {
                formContainer.innerHTML = `
                    <div class="card mt-2">
                        <div class="card-body">
                            <h6 class="card-title">Add Artifact</h6>
                            <div class="row g-2">
                                <div class="col-md-4">
                                    <input type="text" class="form-control" id="artifact-name" placeholder="Name">
                                </div>
                                <div class="col-md-5">
                                    <input type="text" class="form-control" id="artifact-uri" placeholder="Link or path">
                                </div>
                                <div class="col-md-2">
                                    <input type="text" class="form-control" id="artifact-kind" placeholder="Kind">
                                </div>
                                <div class="col-md-1 d-grid">
                                    <button class="btn btn-primary" onclick="app.addArtifact('${taskId}')"><i class="bi bi-plus"></i></button>
                                </div>
                            </div>
                        </div>
                    </div>`;
            } else {
                formContainer.innerHTML = '';
            }
        } catch (error) {
            console.error('Failed to load artifacts:', error);
        }
    },

    async addArtifact(taskId) {
        const name = document.getElementById('artifact-name').value.trim();
        const uri = document.getElementById('artifact-uri').value.trim();
        const kind = document.getElementById('artifact-kind').value.trim() || 'document';

        if (!name || !uri) {
            this.showToast('Name and link are required', 'error');
            return;
        }

        try {
            await api.createArtifact(taskId, { name, uri, kind });
            this.showToast('Artifact added', 'success');
            await this.loadTaskArtifacts(taskId, { canEdit: true });
        } catch (error) {
            this.showToast('Failed to add artifact', 'error', this.formatError(error));
            console.error('Error adding artifact:', error);
        }
    },

    async deleteArtifact(taskId, artifactId) {
        if (!confirm('Delete this artifact?')) return;
        try {
            await api.deleteArtifact(taskId, artifactId);
            this.showToast('Artifact deleted', 'success');
            await this.loadTaskArtifacts(taskId, { canEdit: true });
        } catch (error) {
            this.showToast('Failed to delete artifact', 'error', this.formatError(error));
            console.error('Error deleting artifact:', error);
        }
    }
});

