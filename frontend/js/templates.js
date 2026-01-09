const templates = {
    
    navbar: `
        <div class="container-fluid">
            <a class="navbar-brand" href="#" onclick="app.showProjects(); return false;">
                <i class="bi bi-diagram-3"></i> Reverse Gantt
            </a>
            <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav">
                <span class="navbar-toggler-icon"></span>
            </button>
            <div class="collapse navbar-collapse" id="navbarNav">
                <ul class="navbar-nav me-auto">
                    <li class="nav-item">
                        <a class="nav-link" href="#" onclick="app.showProjects(); return false;" id="nav-projects">
                            <i class="bi bi-folder"></i> <span id="nav-projects-text">My Projects</span>
                        </a>
                    </li>
                    <li class="nav-item" id="nav-teachers">
                        <a class="nav-link" href="#" onclick="app.showTeachers(); return false;">
                            <i class="bi bi-person-badge"></i> Teachers
                        </a>
                    </li>
                </ul>
                <ul class="navbar-nav">
                    <li class="nav-item dropdown me-2">
                        <a class="nav-link position-relative" href="#" id="notificationDropdown" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                            <i class="bi bi-bell fs-5"></i>
                            <span id="notification-badge" class="position-absolute top-0 start-100 translate-middle badge rounded-pill bg-danger">0</span>
                        </a>
                        <ul class="dropdown-menu dropdown-menu-end notification-dropdown">
                            <li class="dropdown-header d-flex justify-content-between align-items-center">
                                <span><i class="bi bi-bell"></i> Уведомления</span>
                                <button class="btn btn-sm btn-link p-0" onclick="notifications.markAllAsRead(); return false;">Прочитать все</button>
                            </li>
                            <li><hr class="dropdown-divider"></li>
                            <div id="notification-list">
                                <li class="text-center text-muted py-3"><i class="bi bi-inbox"></i> Нет уведомлений</li>
                            </div>
                        </ul>
                    </li>
                    <li class="nav-item dropdown">
                        <a class="nav-link dropdown-toggle" href="#" id="userDropdown" role="button" data-bs-toggle="dropdown">
                            <i class="bi bi-person-circle"></i> <span id="user-name">User</span>
                        </a>
                        <ul class="dropdown-menu dropdown-menu-end">
                            <li><a class="dropdown-item" href="#" onclick="app.showSettings(); return false;"><i class="bi bi-gear"></i> Settings</a></li>
                            <li><hr class="dropdown-divider"></li>
                            <li><a class="dropdown-item" href="#" onclick="auth.logout(); return false;"><i class="bi bi-box-arrow-right"></i> Logout</a></li>
                        </ul>
                    </li>
                </ul>
            </div>
        </div>
    `,

    loginPage: `
        <div class="row justify-content-center">
            <div class="col-md-4">
                <div class="card shadow">
                    <div class="card-body p-5">
                        <h2 class="card-title text-center mb-4">
                            <i class="bi bi-box-arrow-in-right"></i> Login
                        </h2>
                        <form id="login-form">
                            <div class="mb-3">
                                <label for="login-email" class="form-label">Email</label>
                                <input type="email" class="form-control" id="login-email" required>
                            </div>
                            <div class="mb-3">
                                <label for="login-password" class="form-label">Password</label>
                                <input type="password" class="form-control" id="login-password" required>
                            </div>
                            <div class="d-grid">
                                <button type="submit" class="btn btn-primary">
                                    <i class="bi bi-box-arrow-in-right"></i> Login
                                </button>
                            </div>
                            <div class="text-center mt-3">
                                <a href="#" onclick="app.showForgotPassword(); return false;">Forgot password?</a>
                            </div>
                            <div class="text-center mt-2">
                                <a href="#" onclick="app.showRegister(); return false;">Don't have an account? Register</a>
                            </div>
                        </form>
                        <div id="login-error" class="alert alert-danger mt-3"></div>
                    </div>
                </div>
            </div>
        </div>
    `,

    forgotPasswordPage: `
        <div class="row justify-content-center">
            <div class="col-md-5">
                <div class="card shadow">
                    <div class="card-body p-5">
                        <h2 class="card-title text-center mb-4">
                            <i class="bi bi-key"></i> Forgot Password
                        </h2>
                        <div id="forgot-password-step-1">
                            <p class="text-muted">Enter your email address to receive a password reset code.</p>
                            <form id="forgot-password-form">
                                <div class="mb-3">
                                    <label for="forgot-email" class="form-label">Email</label>
                                    <input type="email" class="form-control" id="forgot-email" required>
                                </div>
                                <div class="d-grid">
                                    <button type="submit" class="btn btn-primary">
                                        <i class="bi bi-envelope"></i> Send Reset Code
                                    </button>
                                </div>
                            </form>
                            <div class="text-center mt-3">
                                <a href="#" onclick="app.showNoEmailAccess(); return false;">No access to email?</a>
                            </div>
                            <div class="text-center mt-2">
                                <a href="#" onclick="app.showLogin(); return false;">Back to Login</a>
                            </div>
                        </div>
                        <div id="forgot-password-step-2" style="display: none;">
                            <div class="alert alert-info">
                                <i class="bi bi-info-circle"></i> Check your email for the 6-digit verification code. It expires in 15 minutes.
                            </div>
                            <form id="confirm-reset-form">
                                <div class="mb-3">
                                    <label for="reset-code" class="form-label">6-Digit Code</label>
                                    <input type="text" class="form-control" id="reset-code" maxlength="6" pattern="[0-9]{6}" required>
                                </div>
                                <div class="mb-3">
                                    <label for="new-password" class="form-label">New Password</label>
                                    <input type="password" class="form-control" id="new-password" required>
                                    <div class="form-text">Must be at least 8 characters with uppercase, lowercase, digit, and special character</div>
                                </div>
                                <div class="mb-3">
                                    <label for="new-password-confirm" class="form-label">Confirm New Password</label>
                                    <input type="password" class="form-control" id="new-password-confirm" required>
                                </div>
                                <div class="d-grid">
                                    <button type="submit" class="btn btn-primary">
                                        <i class="bi bi-check-circle"></i> Reset Password
                                    </button>
                                </div>
                            </form>
                            <div class="text-center mt-3">
                                <a href="#" onclick="app.showForgotPassword(); return false;">Back</a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    `,

    noEmailAccessPage: `
        <div class="row justify-content-center">
            <div class="col-md-5">
                <div class="card shadow">
                    <div class="card-body p-5">
                        <h2 class="card-title text-center mb-4">
                            <i class="bi bi-person-check"></i> No Email Access
                        </h2>
                        <p class="text-muted">If you don't have access to your email, you can request a password reset from a teacher.</p>
                        <p class="text-muted">Enter your email address below to send a request to teachers for approval.</p>
                        <form id="no-email-access-form">
                            <div class="mb-3">
                                <label for="no-email-email" class="form-label">Email</label>
                                <input type="email" class="form-control" id="no-email-email" required>
                            </div>
                            <div class="d-grid">
                                <button type="submit" class="btn btn-warning">
                                    <i class="bi bi-send"></i> Request Teacher Approval
                                </button>
                            </div>
                        </form>
                        <div class="text-center mt-3">
                            <a href="#" onclick="app.showLogin(); return false;">Back to Login</a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    `,

    registerPage: `
        <div class="row justify-content-center">
            <div class="col-md-4">
                <div class="card shadow">
                    <div class="card-body p-5">
                        <h2 class="card-title text-center mb-4">
                            <i class="bi bi-person-plus"></i> Register
                        </h2>
                        <form id="register-form">
                            <div class="mb-3">
                                <label for="register-email" class="form-label">Email</label>
                                <input type="email" class="form-control" id="register-email" required>
                            </div>
                            <div class="mb-3">
                                <label for="register-name" class="form-label">Name</label>
                                <input type="text" class="form-control" id="register-name" required>
                            </div>
                            <div class="mb-3">
                                <label for="register-password" class="form-label">Password</label>
                                <input type="password" class="form-control" id="register-password" required>
                                <div class="form-text">Must be at least 8 characters with uppercase, lowercase, digit, and special character</div>
                            </div>
                            <div class="mb-3">
                                <label for="register-password-confirm" class="form-label">Confirm Password</label>
                                <input type="password" class="form-control" id="register-password-confirm" required>
                            </div>
                            <div class="d-grid">
                                <button type="submit" class="btn btn-primary">
                                    <i class="bi bi-person-plus"></i> Register
                                </button>
                            </div>
                            <div class="text-center mt-3">
                                <a href="#" onclick="app.showLogin(); return false;">Already have an account? Login</a>
                            </div>
                        </form>
                        <div id="register-error" class="alert alert-danger mt-3"></div>
                    </div>
                </div>
            </div>
        </div>
    `,

    projectsPage: `
        <div class="d-flex justify-content-between align-items-center mb-4">
            <h2><i class="bi bi-folder"></i> <span id="projects-title">My Projects</span></h2>
            <button class="btn btn-primary" onclick="app.showCreateProject();">
                <i class="bi bi-plus-circle"></i> Create Project
            </button>
        </div>
        <div id="projects-filter" class="mb-3">
            <div class="form-check form-switch">
                <input class="form-check-input" type="checkbox" id="filter-my-projects" onchange="app.filterMyProjects(this.checked);">
                <label class="form-check-label" for="filter-my-projects">Show only my projects</label>
            </div>
        </div>
        <div id="projects-list" class="row"></div>
    `,

    projectDetailsPage: `
        <div class="d-flex justify-content-between align-items-center mb-4">
            <div>
                <button class="btn btn-outline-secondary mb-2" onclick="app.showProjects();">
                    <i class="bi bi-arrow-left"></i> Back to Projects
                </button>
                <h2 id="project-name">Project Name</h2>
            </div>
            <div id="project-actions"></div>
        </div>
        <div class="card mb-4">
            <div class="card-body">
                <h5 class="card-title">Project Information</h5>
                <p id="project-description"></p>
                <div class="row">
                    <div class="col-md-3"><strong>Start Date:</strong> <span id="project-start-date">-</span></div>
                    <div class="col-md-3"><strong>Due Date:</strong> <span id="project-due-date">-</span></div>
                    <div class="col-md-3"><strong>Created:</strong> <span id="project-created-at">-</span></div>
                </div>
            </div>
        </div>
        <ul class="nav nav-tabs mb-3" id="project-tabs">
            <li class="nav-item"><a class="nav-link active" href="#" onclick="app.showProjectTab('overview').catch(console.error); return false;">Overview</a></li>
            <li class="nav-item"><a class="nav-link" href="#" onclick="app.showProjectTab('members').catch(console.error); return false;">Members</a></li>
            <li class="nav-item"><a class="nav-link" href="#" onclick="app.showProjectTab('tasks').catch(console.error); return false;">Tasks</a></li>
            <li class="nav-item"><a class="nav-link" href="#" onclick="app.showProjectTab('gantt').catch(console.error); return false;">Gantt Chart</a></li>
        </ul>
        <div class="mb-3" id="reverse-schedule-button-container">
            <button class="btn btn-success" onclick="app.runReverseScheduleForCurrentProject();">
                <i class="bi bi-calculator"></i> Calculate Reverse Schedule
            </button>
        </div>
        <div id="project-tab-content"></div>
    `,

    teachersPage: `
        <div class="d-flex justify-content-between align-items-center mb-4">
            <h2><i class="bi bi-person-badge"></i> Teachers Panel</h2>
            <button class="btn btn-outline-primary" onclick="app.showTeacherPasswordResetRequests();">
                <i class="bi bi-key-fill"></i> Password Reset Requests
            </button>
        </div>
        <div class="card mb-4">
            <div class="card-header"><h5 class="mb-0"><i class="bi bi-diagram-3"></i> Reverse Schedule</h5></div>
            <div class="card-body">
                <p class="text-muted">Calculate reverse schedule for a project based on dependencies and hierarchy.</p>
                <div class="row">
                    <div class="col-md-6">
                        <label for="reverse-schedule-project" class="form-label">Select Project</label>
                        <select class="form-select" id="reverse-schedule-project">
                            <option value="">Loading projects...</option>
                        </select>
                    </div>
                    <div class="col-md-6 d-flex align-items-end">
                        <button class="btn btn-primary" onclick="app.runReverseSchedule();">
                            <i class="bi bi-play-circle"></i> Calculate Schedule
                        </button>
                    </div>
                </div>
                <div id="reverse-schedule-result" class="mt-3"></div>
            </div>
        </div>
        <div class="card">
            <div class="card-header"><h5 class="mb-0"><i class="bi bi-people"></i> All Students</h5></div>
            <div class="card-body">
                <p class="text-muted">View all students and promote them to teachers.</p>
                <div id="users-list"><p class="text-muted">Loading users...</p></div>
            </div>
        </div>
    `,

    taskDetailsPage: `
        <div class="mb-4">
            <button class="btn btn-outline-secondary" onclick="app.showProjectDetails(app.currentProjectId);">
                <i class="bi bi-arrow-left"></i> Back to Project
            </button>
        </div>
        <div class="card">
            <div class="card-body">
                <div class="d-flex justify-content-between align-items-center mb-3">
                    <h3 id="task-name">Task Name</h3>
                    <div id="task-actions"></div>
                </div>
                <div id="task-details-content"></div>
            </div>
        </div>
    `,

    modals: {
        project: `
            <div class="modal fade" id="project-modal" tabindex="-1">
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title" id="project-modal-title">Create Project</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
                        </div>
                        <div class="modal-body">
                            <form id="project-form">
                                <input type="hidden" id="project-form-id">
                                <div class="mb-3">
                                    <label for="project-form-name" class="form-label">Name *</label>
                                    <input type="text" class="form-control" id="project-form-name" required>
                                </div>
                                <div class="mb-3">
                                    <label for="project-form-description" class="form-label">Description</label>
                                    <textarea class="form-control" id="project-form-description" rows="3"></textarea>
                                </div>
                                <div class="row">
                                    <div class="col-md-6 mb-3">
                                        <label for="project-form-start-date" class="form-label">Start Date</label>
                                        <input type="date" class="form-control" id="project-form-start-date">
                                    </div>
                                    <div class="col-md-6 mb-3">
                                        <label for="project-form-due-date" class="form-label">Due Date *</label>
                                        <input type="date" class="form-control" id="project-form-due-date" required>
                                    </div>
                                </div>
                                <div id="project-form-leader-group" class="mb-3">
                                    <label for="project-form-leader-id" class="form-label">Leader (Student) *</label>
                                    <select class="form-select" id="project-form-leader-id" required>
                                        <option value="">Select student...</option>
                                    </select>
                                    <div class="form-text">Teacher must specify a student leader</div>
                                </div>
                            </form>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancel</button>
                            <button type="button" class="btn btn-primary" onclick="app.saveProject();">Save</button>
                        </div>
                    </div>
                </div>
            </div>
        `,

        member: `
            <div class="modal fade" id="member-modal" tabindex="-1">
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title">Add Member</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
                        </div>
                        <div class="modal-body">
                            <form id="member-form">
                                <div class="mb-3">
                                    <label for="member-form-user-id" class="form-label">Select User</label>
                                    <select class="form-select" id="member-form-user-id" required>
                                        <option value="">Loading users...</option>
                                    </select>
                                </div>
                                <div class="mb-3">
                                    <label for="member-form-tags" class="form-label">Tags (comma-separated)</label>
                                    <input type="text" class="form-control" id="member-form-tags" placeholder="frontend, backend, devops">
                                </div>
                            </form>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancel</button>
                            <button type="button" class="btn btn-primary" onclick="app.addMember();">Add Member</button>
                        </div>
                    </div>
                </div>
            </div>
        `,

        task: `
            <div class="modal fade" id="task-modal" tabindex="-1">
                <div class="modal-dialog modal-lg">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title" id="task-modal-title">Create Task</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
                        </div>
                        <div class="modal-body">
                            <form id="task-form">
                                <input type="hidden" id="task-form-id">
                                <div class="mb-3">
                                    <label for="task-form-name" class="form-label">Name *</label>
                                    <input type="text" class="form-control" id="task-form-name" required>
                                </div>
                                <div class="mb-3">
                                    <label for="task-form-description" class="form-label">Description</label>
                                    <textarea class="form-control" id="task-form-description" rows="3"></textarea>
                                </div>
                                <div class="row">
                                    <div class="col-md-6 mb-3">
                                        <label for="task-form-task-type" class="form-label">Task Type *</label>
                                        <select class="form-select" id="task-form-task-type" required>
                                            <option value="">Select type...</option>
                                            <option value="task">Task</option>
                                            <option value="feature">Feature</option>
                                        </select>
                                    </div>
                                    <div class="col-md-6 mb-3">
                                        <label for="task-form-priority" class="form-label">Priority *</label>
                                        <select class="form-select" id="task-form-priority" required>
                                            <option value="">Select priority...</option>
                                            <option value="low">Low</option>
                                            <option value="normal">Normal</option>
                                            <option value="high">High</option>
                                            <option value="critical">Critical</option>
                                        </select>
                                    </div>
                                </div>
                                <div class="row">
                                    <div class="col-md-6 mb-3">
                                        <label for="task-form-estimated-duration" class="form-label">Estimated Duration (hours)</label>
                                        <input type="number" class="form-control" id="task-form-estimated-duration" min="0">
                                    </div>
                                    <div class="col-md-6 mb-3">
                                        <label for="task-form-buffer" class="form-label">Buffer (hours)</label>
                                        <input type="number" class="form-control" id="task-form-buffer" min="0" value="0">
                                    </div>
                                </div>
                                <div class="row">
                                    <div class="col-md-6 mb-3" id="task-form-hardness-group">
                                        <label for="task-form-hardness" class="form-label">Hardness</label>
                                        <select class="form-select" id="task-form-hardness">
                                            <option value="soft">Soft</option>
                                            <option value="hard">Hard</option>
                                        </select>
                                        <div class="form-text">Only project leaders can set hardness</div>
                                    </div>
                                </div>
                                <div class="mb-3">
                                    <label for="task-form-parent-task-id" class="form-label">Parent Task</label>
                                    <select class="form-select" id="task-form-parent-task-id">
                                        <option value="">None (root task)</option>
                                    </select>
                                </div>
                                <div id="task-form-status-group" class="mb-3">
                                    <label for="task-form-status" class="form-label">Status</label>
                                    <select class="form-select" id="task-form-status">
                                        <option value="planned">Planned</option>
                                        <option value="inProgress">InProgress</option>
                                        <option value="needsReview">NeedsReview</option>
                                        <option value="accepted">Accepted</option>
                                        <option value="rejected">Rejected</option>
                                        <option value="blocked">Blocked</option>
                                        <option value="done">Done</option>
                                    </select>
                                </div>
                                <div id="task-form-dates-group" class="row">
                                    <div class="col-md-6 mb-3">
                                        <label for="task-form-progress" class="form-label">Progress (%)</label>
                                        <input type="number" class="form-control" id="task-form-progress" min="0" max="100" value="0">
                                    </div>
                                </div>
                                <div id="task-form-owner-group" class="mb-3">
                                    <label for="task-form-owner-id" class="form-label">Owner (Student) *</label>
                                    <select class="form-select" id="task-form-owner-id"></select>
                                    <div class="form-text">Teacher must specify a task owner</div>
                                </div>
                            </form>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancel</button>
                            <button type="button" class="btn btn-primary" onclick="app.saveTask();">Save</button>
                        </div>
                    </div>
                </div>
            </div>
        `,

        dependency: `
            <div class="modal fade" id="dependency-modal" tabindex="-1">
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title">Add Dependency</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
                        </div>
                        <div class="modal-body">
                            <form id="dependency-form">
                                <div class="mb-3">
                                    <label for="dependency-form-to-task" class="form-label">Depends on Task</label>
                                    <select class="form-select" id="dependency-form-to-task" required></select>
                                </div>
                                <div class="mb-3">
                                    <label for="dependency-form-type" class="form-label">Dependency Type</label>
                                    <select class="form-select" id="dependency-form-type" required>
                                        <option value="FS">Finish-to-Start (FS)</option>
                                        <option value="FF">Finish-to-Finish (FF)</option>
                                        <option value="SS">Start-to-Start (SS)</option>
                                        <option value="SF">Start-to-Finish (SF)</option>
                                    </select>
                                </div>
                                <div class="mb-3">
                                    <label for="dependency-form-min-gap" class="form-label">Min Gap (hours)</label>
                                    <input type="number" class="form-control" id="dependency-form-min-gap" value="0" min="0">
                                </div>
                            </form>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancel</button>
                            <button type="button" class="btn btn-primary" onclick="app.addDependency();">Add Dependency</button>
                        </div>
                    </div>
                </div>
            </div>
        `,

        settings: `
            <div class="modal fade" id="settings-modal" tabindex="-1">
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title"><i class="bi bi-gear"></i> Settings</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
                        </div>
                        <div class="modal-body">
                            <div class="mb-3">
                                <h6>Email Notifications</h6>
                                <p class="text-muted small">Receive email notifications when your tasks are updated or reviewed.</p>
                                <div class="form-check form-switch">
                                    <input class="form-check-input" type="checkbox" id="email-notifications-toggle">
                                    <label class="form-check-label" for="email-notifications-toggle">Enable email notifications</label>
                                </div>
                            </div>
                            <hr>
                            <div class="mb-3">
                                <h6><i class="bi bi-key"></i> Password</h6>
                                <button type="button" class="btn btn-outline-primary btn-sm" onclick="app.showChangePasswordInSettings();">
                                    <i class="bi bi-key"></i> Change Password
                                </button>
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Close</button>
                            <button type="button" class="btn btn-primary" onclick="app.saveSettings();">Save Settings</button>
                        </div>
                    </div>
                </div>
            </div>
        `
    },

    toast: `
        <div id="toast" class="toast" role="alert">
            <div class="toast-header">
                <strong class="me-auto" id="toast-title">Notification</strong>
                <button type="button" class="btn-close" data-bs-dismiss="toast"></button>
            </div>
            <div class="toast-body" id="toast-body"></div>
        </div>
    `
};

