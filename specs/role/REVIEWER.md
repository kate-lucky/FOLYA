# FOLYA Role Specification: REVIEWER 🐱💼

This document defines the mandatory audit procedures for the **Reviewer** role (the Orchestrator's primary function during code integration).

## 1. Core Mission
The Reviewer's absolute priority is to **Identify and Neutralize Secrets**. Any code contribution from a sub-agent must be scrutinized for sensitive data before any merge.

## 2. Audit Checklist
Before any feature branch is merged into the `personal:kate-dev` branch, the Reviewer MUST check for:
- **API Keys**: E.g., `sk-..., AKIA..., ...secret...`
- **Secrets**: Password strings, database connection strings (with credentials).
- **URLs**: Sensitive endpoint URLs or internal IP addresses (unless specifically required).
- **Hardcoded Tokens**: Any session or authentication tokens.

## 3. Secret Management Protocol
If a secret is found in a sub-agent's contribution:
1.  **REJECT IMMEDIATELY**: The branch must be rejected and the sub-agent instructed to scrub the history.
2.  **EXTRACT & STORE**: If the secret is required for the project to function:
    - **Local Storage ONLY**: Store the secret in a local file (e.g., `workspace/secrets.json`) that is **GIT-IGNORED**.
    - **Environment Variables**: Replace the hardcoded secret with a reference to an environment variable or a config file listed in `.gitignore`.
3.  **NEVER COMMIT**: No secret, once identified, shall ever touch the remote repositories (`Origin` or `Personal`).

## 4. Automated Verification (Future)
The Reviewer is encouraged to use tools like `trufflehog` or `gitleaks` locally to ensure no secrets have slipped through.

---
*Created by Kate (Professional Secretary & Agent Orchestrator)*
