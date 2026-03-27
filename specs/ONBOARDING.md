# FOLYA Agent Onboarding Specification 🐱💼

All sub-agents (including Rust Coders, Analysts, and Auditors) must acknowledge and adhere to these core requirements before performing any task.

## 1. Safety & Security (The Redlines)
- **ZERO TOLERANCE FOR SECRETS**: You are strictly forbidden from committing or pushing any API keys, passwords, tokens, or credentials. Reference `specs/REDLINE.md`.
- **PRE-PUSH AUDIT**: The Orchestrator (Kate) will run `gitleaks` on every commit. Any secret found will result in immediate task termination and history scrubbing.

## 2. Development Workflow (Isolation Protocol)
- **REMOTE ISOLATION**: You must ONLY push to the `personal` remote (`kate-lucky/FOLYA.git`).
- **FEATURE BRANCHING**: You must create a new branch for every task: `feat/agent-<id>/<task-description>`.
- **NO DIRECT MERGES**: You are not authorized to merge into `kate-dev` or `main`. Your work will be reviewed and merged by the Orchestrator.

## 3. Environment & Tools
- **DOCKER ENVIRONMENT**: Follow the environment specs in `specs/ENVIRONMENT.md`.
- **BEST PRACTICES**: Adhere to `specs/BEST_PRACTICES.md` (Modular Rust, Zero-Copy, Result handling).

## 4. Acknowledgement
By proceeding with the assigned task, you confirm you have read the `specs/` directory and will maintain the high-tier professional standards of the FOLYA project.

---
*Created by Kate (Professional Secretary & Agent Orchestrator)*
