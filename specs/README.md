# FOLYA Specification Guide: The Source of Truth 🐱💼

This document explains the structure, purpose, and access control for the **FOLYA** specification system. All project participants (Orchestrator, Sub-Agents, and Human Supervisors) must understand these guidelines.

## 1. Why do these specifications exist?
The FOLYA project operates in a **high-security, multi-agent environment**. To prevent merge conflicts, data corruption, and catastrophic secret leaks, we require a "Hard Blueprint" that overrides any individual agent's default behavior. 

These specs turn a collection of AI models into a synchronized, professional engineering team.

## 2. Directory Structure & Purpose

| Path | Purpose | Target Audience |
| :--- | :--- | :--- |
| **`specs/ONBOARDING.md`** | The universal contract and safety rules. | All Sub-Agents |
| **`specs/REDLINE.md`** | Absolute prohibitions (Zero-Secrets, Branch Protection). | All Agents & Orchestrator |
| **`specs/WORKFLOW.md`** | The isolation protocol (Fork-and-PR, Port 443). | All Agents & Orchestrator |
| **`specs/roles/`** | Granular responsibilities for specialized agents. | Specific Sub-Agents |
| **`specs/ENVIRONMENT.md`** | The technical Docker and toolchain blueprint. | DevOps & Rust Coders |
| **`specs/BEST_PRACTICES.md`** | Coding standards (Zero-Copy Rust, Error Handling). | Rust Coders & Auditors |
| **`specs/EXTERNAL_RESOURCES.md`**| Authorized APIs, Crates, and Databases. | Data Engineers & Analysts |

## 3. How to Read These Files
- **Mandatory Sequence**: All newly spawned agents must read `ONBOARDING.md` first, followed by `REDLINE.md` and their assigned role in `roles/`.
- **Precedence**: In case of conflict, `REDLINE.md` and `WORKFLOW.md` always take precedence over role-specific instructions.
- **Verification**: The Orchestrator (Kate) will audit all contributions against these specifications. Non-compliant work will be rejected.

## 4. Who Can Read Them?
- **Public/Upstream**: All specifications are pushed to the `eBioRing/FOLYA` upstream repository for transparency and human review.
- **Agents**: All agents have read-only access to the `specs/` directory within their local workspace.
- **Human Supervisor**: Unka Malloc maintains final authority to edit or override any specification.

---
*Created by Kate (Professional Secretary & Agent Orchestrator)*
