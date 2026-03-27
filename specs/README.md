# FOLYA Specification System: Entrance & Usage Guide 🐱💼

This is the **Entrance File** for all project participants. It explains the purpose of every specification file in this directory and how they should be utilized by the Orchestrator, Sub-Agents, and Human Supervisors.

## 1. Document Directory & Usage

| File Path | Usage & Purpose | Target Audience |
| :--- | :--- | :--- |
| **`specs/README.md`** | **Entrance**: High-level guide and table of contents. Start here. | Everyone |
| **`specs/ONBOARDING.md`**| **Contract**: Mandatory safety and behavior agreement. | All Sub-Agents |
| **`specs/REDLINE.md`** | **Restrictions**: Hard prohibitions (No secrets, Port 443). | All Agents |
| **`specs/WORKFLOW.md`** | **Isolation**: Fork-and-PR logic and branch management. | All Agents |
| **`specs/roles/*.md`** | **Mission**: Specialized responsibilities for each agent. | Assigned Agent |
| **`specs/ENVIRONMENT.md`**| **Blueprint**: Technical Docker and toolchain environment. | DevOps/Rust Coder |
| **`specs/BEST_PRACTICES.md`**| **Standards**: Coding quality and performance rules. | Rust Coder/Auditor |
| **`specs/EXTERNAL_RESOURCES.md`**| **Inventory**: Authorized APIs, Crates, and Databases. | Data Eng/Analyst |

## 2. Mandatory Reading Sequence
1.  **Orchestration Logic**: Read `README.md` (this file) to understand the landscape.
2.  **Safety First**: Read `ONBOARDING.md` and `REDLINE.md` to establish non-negotiable boundaries.
3.  **Process Awareness**: Read `WORKFLOW.md` to understand how to contribute code safely.
4.  **Technical Deep-Dive**: Read the relevant `ENVIRONMENT`, `BEST_PRACTICES`, or `EXTERNAL_RESOURCES` for your task.
5.  **Execution**: Read your specific role in `roles/`.

## 3. Policy Precedence
- **Safety Rules** (`REDLINE.md`) always override technical implementation details.
- **Workflow Protocol** (`WORKFLOW.md`) always overrides individual agent preferences.

---
*Created by Kate (Professional Secretary & Agent Orchestrator)*

---
*Created by Kate (Professional Secretary & Agent Orchestrator)*
