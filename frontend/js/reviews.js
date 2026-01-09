Object.assign(app, {
    async loadTaskReview(taskId, task, isOwner) {
        const reviewSection = document.getElementById('review-section');
        if (!reviewSection) return;

        try {
            const review = await api.getReview(taskId);
            const isLeader = this.isProjectLeader();
            const canReview = auth.isTeacher() || isLeader;
            const taskStatus = task.status?.toLowerCase() || '';
            const needsReview = (taskStatus === 'needsreview');

            let html = '';

            if (!review) {
                html += '<p class="text-muted">No review yet</p>';
            } else {
                const reviewerName = await this.getUserName(review.reviewerId).catch(() => review.reviewerId);
                const decisionLabel = review.decision ? this.humanizeEnum(review.decision) : 'Pending';
                const decisionBadge = review.decision === 'accepted' ? 'bg-success' : 
                                     review.decision === 'rejected' ? 'bg-danger' : 'bg-warning';
                
                html += `
                    <div class="card">
                        <div class="card-body">
                            <div class="d-flex justify-content-between align-items-start mb-2">
                                <div>
                                    <strong>Reviewer:</strong> ${this.escapeHtml(reviewerName)}<br>
                                    <strong>Decision:</strong> <span class="badge ${decisionBadge}">${this.escapeHtml(decisionLabel)}</span>
                                </div>
                                <small class="text-muted">${this.formatDate(review.createdAt)}</small>
                            </div>
                            ${review.comment ? `
                                <div class="mt-2">
                                    <strong>Comment:</strong>
                                    <p class="mb-0">${this.escapeHtml(review.comment)}</p>
                                </div>
                            ` : ''}
                        </div>
                    </div>`;
            }

            if (canReview && needsReview && (!review || !review.decision)) {
                html += `
                    <div class="card mt-3">
                        <div class="card-body">
                            <h6>Review Task</h6>
                            <form id="review-form" onsubmit="app.submitReview('${taskId}'); return false;">
                                <div class="mb-3">
                                    <label class="form-label">Decision *</label>
                                    <div>
                                        <div class="form-check form-check-inline">
                                            <input class="form-check-input" type="radio" name="review-decision" id="review-accept" value="accepted" required>
                                            <label class="form-check-label" for="review-accept">Approve</label>
                                        </div>
                                        <div class="form-check form-check-inline">
                                            <input class="form-check-input" type="radio" name="review-decision" id="review-reject" value="rejected" required>
                                            <label class="form-check-label" for="review-reject">Reject</label>
                                        </div>
                                    </div>
                                </div>
                                <div class="mb-3">
                                    <label for="review-comment" class="form-label">Comment</label>
                                    <textarea class="form-control" id="review-comment" rows="3" placeholder="Enter your review comment..."></textarea>
                                </div>
                                <button type="submit" class="btn btn-primary">Submit Review</button>
                            </form>
                        </div>
                    </div>`;
            }

            reviewSection.innerHTML = html;
        } catch (error) {
            console.error('Failed to load review:', error);
            reviewSection.innerHTML = '<p class="text-danger">Failed to load review</p>';
        }
    },

    async sendTaskToReview(taskId) {
        if (!confirm('Send this task to review? The status will be changed to NeedsReview.')) {
            return;
        }

        try {
            await api.updateTask(taskId, { status: 'needsReview' });
            this.showToast('Task sent to review successfully', 'success');
            
            const task = await api.getTask(taskId);
            await this.renderTaskDetails(task);
        } catch (error) {
            this.showToast('Failed to send task to review', 'error', error);
            console.error('Error sending task to review:', error);
        }
    },

    async submitReview(taskId) {
        const decision = document.querySelector('input[name="review-decision"]:checked')?.value;
        const comment = document.getElementById('review-comment')?.value || null;

        if (!decision) {
            this.showToast('Please select a decision', 'error');
            return;
        }

        try {
            const currentUserId = localStorage.getItem('userId');
            
            await api.createReview(taskId, {
                reviewerId: currentUserId,
                decision: decision,
                comment: comment
            });

            const newStatus = decision === 'accepted' ? 'done' : 'inProgress';
            await api.updateTask(taskId, { status: newStatus });

            this.showToast(`Task ${decision === 'accepted' ? 'approved' : 'rejected'} successfully`, 'success');
            
            const task = await api.getTask(taskId);
            await this.renderTaskDetails(task);
        } catch (error) {
            this.showToast('Failed to submit review', 'error', error);
            console.error('Error submitting review:', error);
        }
    },

    async getUserName(userId) {
        try {
            const user = await api.getUser(userId);
            return user.name;
        } catch (error) {
            return userId;
        }
    }
});

