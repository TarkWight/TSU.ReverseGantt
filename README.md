# Reverse Gantt System — Backend Prototype

## Project Overview
**Reverse Gantt System** is an academic project management system that uses a **reverse-planning approach** —  
tasks and dependencies are scheduled **backwards from the final deadline** to ensure proper sequencing and risk control.

This prototype represents the **backend service** built with **Rust (Axum)**.  
A web or mobile client (e.g., SwiftUI) may later be integrated.

---

## Tech Stack
- **Backend:** Rust, Axum 0.8, Tokio, Serde
- **Configuration:** `.env` file with server and port
- **Data exchange:** JSON

---

## API Endpoints
| Method | Path        | Description                          |
|--------|-------------|--------------------------------------|
| `GET`  |             | Root endpoint, test API availability |
| `GET`  | `/health`   | Health check                         |
| `GET`  | `/projects` | Retrieve list of projects            |
| `POST` | `/projects` | Create a new project                 |

---

## Architecture
- `main.rs` — entry point, server setup, routing
...

---

## Commit Message Style (Symbolic Annotation)

| Symbol |             Meaning             |     Equivalent Word      |            Example            |
|:------:|:-------------------------------:|:------------------------:|:-----------------------------:|
|  `+`   |  Addition of new code or files  |    add, create, init     | `+ new authentication module` |
|  `-`   |       Removal or cleanup        | remove, delete, cleanup  |      `- old test files`       |
|  `*`   |      Update or improvement      | update, modify, refactor |    `* UI layout adjusted`     |
|  `!`   |   Bug fix or error correction   |   fix, bugfix, hotfix    |   `! login issue resolved`    |
|  `#`   | Meta, merge, or version-related |   merge, bump, release   |     `# merge dev branch`      |
|  `~`   |    Minor or cosmetic change     |   tweak, adjust, minor   |     `~ small text fixes`      |
|  `=`   |     Configuration or setup      |      config, setup       |    `= .env example added`     |
|  `%`   |   Optimization or performance   |      perf, optimize      |     `% faster DB queries`     |
|  `?`   |      Documentation updates      |       docs, readme       |  `? expand API usage guide`   |
|  `^`   |     Tests added or updated      |      test, coverage      |  `^ add /health route tests`  |
|  `&`   |    CI/CD or build automation    |       ci, pipeline       |   `& enable GitHub Actions`   |
|  `$`   | Dependencies or package changes |      deps, upgrade       |    `$ bump Axum to 0.8.6`     |

---

## Author
**Arseniy Gonda**  
Individual project for the course *"Foundations of Direct Design"*  
Tomsk State University, Higher IT School (HITS), 2025
