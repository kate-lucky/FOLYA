# FOLYA Development Environment Specification 🐱💼

This document defines the standardized environment required to develop, build, and test the **FOLYA** Quantitative Trading Framework. All agents (including subagents) must adhere to these specifications.

## 1. Core Principles
- **Reproducibility**: The environment must be reproducible via Docker.
- **Dynamic Discovery**: Avoid hardcoded IPs or magic numbers. Use scripts to discover the environment.
- **Security**: SSH keys must be managed via standard Linux mechanisms (e.g., `~/.ssh/config`).

## 2. Docker Specification (The FOLYA Standard)
The following Dockerfile defines the complete development environment:

```dockerfile
# Use Debian-based image for maximum compatibility
FROM rust:1.77-slim-bookworm

# 1. System Prerequisites
RUN apt-get update && apt-get install -y \
    sudo \
    curl \
    git \
    jq \
    iproute2 \
    ssh \
    build-essential \
    libssl-dev \
    pkg-config \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 2. Programming Languages
# Rust is pre-installed. Add necessary components.
RUN rustup component add rustfmt clippy

# 3. Environment Configuration
ENV WORKSPACE="/root/.openclaw/workspace/folya"
WORKDIR ${WORKSPACE}

# 4. SSH Setup (Standard for all Agents)
RUN mkdir -p /root/.ssh && chmod 700 /root/.ssh

# 5. Initialization Entrypoint
# Agents should run a startup script to discover the environment
# Example: bash startup/init.sh
```

## 3. System Prerequisites
| Component | Version | Purpose |
| :--- | :--- | :--- |
| **Rust** | 1.77+ | Core programming language |
| **Cargo** | 1.77+ | Package manager and build tool |
| **Git** | 2.x | Version control |
| **JQ** | 1.6+ | JSON processing for API responses |
| **SSH** | OpenSSH | Secure repository access |
| **Docker** | 20.x+ | Environment containerization |

## 4. SSH / Git Configuration
All agents must use the following configuration to access the FOLYA repository over port 443:

- **Host**: `github.com-folya`
- **HostName**: `ssh.github.com`
- **Port**: `443`
- **User**: `git`
- **IdentityFile**: `/root/.ssh/id_ed25519_folya-deploy`

## 5. Development Workflow
1. **Branching**: All work must be done on the `kate-dev` branch.
2. **Commit Policy**: Rapid, atomic commits are preferred.
3. **Subagent Tasks**: When spawning a subagent, provide it with the `/spec` directory to ensure environment alignment.

---
*Created by Kate (Professional Secretary)*
