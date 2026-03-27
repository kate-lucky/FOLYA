# FOLYA Development Workflow Specification 🐱💼

This document defines the mandatory git workflow for all agents (including the Orchestrator and all sub-agents) to ensure code isolation, safety, and human oversight.

## 1. Role-Based Repository Access

- **Upstream (Source of Truth)**: `https://github.com/eBioRing/FOLYA.git` (Remote: `origin`)
- **Isolation/Agent Fork**: `https://github.com/kate-lucky/FOLYA.git` (Remote: `personal`)

## 2. Agent Branching Protocol

### For the Orchestrator (Kate):
1.  **Branch Management**: Authorize and manage sub-agent feature branches.
2.  **Consolidation**: Pull completed feature branches from the `personal` remote to the local `workspace/folya` for review and testing.
3.  **Strict Isolation**: **NEVER** push directly to the `origin` repository. All contributions must be pushed to the `personal` remote (`kate-lucky/FOLYA`).
4.  **Merge Request (PR)**: Create a Pull Request (PR) from `kate-lucky/FOLYA:kate-dev` to `eBioRing/FOLYA:kate-dev`. The Orchestrator will provide the PR link to the Human Supervisor for final merge.

### For Sub-Agents:

## 3. Step-by-Step Task Workflow

| Step | Action | Responsibility | Target Branch/Repo |
| :--- | :--- | :--- | :--- |
| **1** | Initialize Task | Orchestrator | Local `kate-dev` |
| **2** | Create Feature Branch | Sub-Agent | `personal:feat/...` |
| **3** | Development & Atomic Commits | Sub-Agent | `personal:feat/...` |
| **4** | Push to Agent Fork | Sub-Agent | `personal` Repository |
| **5** | Review & Local Merge | Orchestrator | Local `kate-dev` |
| **6** | Push to Personal kate-dev | Orchestrator | `personal:kate-dev` |
| **7** | Create Merge Request | Orchestrator | `origin:kate-dev` |

## 4. Automated Cost Tracking (The Auditor)
1.  **Background Monitoring**: A local cron job runs a script (`workspace/folya/tools/cost_auditor.sh`) every 4 hours to aggregate token usage and requests across all active agents.
2.  **Milestone Logging**: The `REVIEWER` role (Orchestrator) automatically executes the cost tracking script before and after every merge to calculate the "Delta" for the specific task.
3.  **Automated Reporting**: The cost auditor script automatically updates `MILESTONES.md` with the latest consumption data.

## 5. Safety Constraints
- **NO parallel local edits**: Only one agent may hold the "write lock" on a specific file at a time.
- **NO direct pushes to origin/main**: The `main` branch is reserved for stable releases and is managed exclusively by the Human Supervisor (Unka).
- **NO plain text secrets**: All API keys and deployment secrets must remain outside the git history.

---
*Created by Kate (Professional Secretary & Agent Orchestrator)*
