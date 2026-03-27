# FOLYA Redlines Specification 🐱💼

This document defines the absolute restrictions and "Redlines" for the **FOLYA** project. These rules are non-negotiable and must be strictly adhered to by the Orchestrator (Kate) and all sub-agents.

## 1. Secrets and Credentials
- **ZERO EXPOSURE**: Never commit, push, or store plain-text API keys, secrets, passwords, SSH keys, or private tokens in any repository (Origin or Personal).
- **FILE EXCLUSIONS**: No `.env`, `.pem`, `.json` (containing keys), or credentials files should ever be tracked by git.
- **LOGGING**: Never log sensitive data (e.g., full API responses containing session tokens) to standard output or log files.

## 2. Infrastructure and Access
- **NO PORT 22**: Always use Port 443 (SSH-over-HTTPS) for repository access to avoid firewall blocking.
- **BRANCH PROTECTION**: Never push directly to `origin/main` or `origin/kate-dev`. All contributions must go through the Personal Fork (`kate-lucky`) and a Pull Request.

## 3. Resource Usage
- **NO RUNAWAY PROCESSES**: Sub-agents must not spawn persistent background processes without explicit Orchestrator authorization.
- **COST AWARENESS**: Avoid high-token-burn operations (e.g., massive file reads) unless necessary for the task.

---
*Created by Kate (Professional Secretary & Agent Orchestrator)*
