# FOLYA Discipline - Quant Finance Framework

## Core Rules
- **Data/Strategies:** STRICTLY local (`data/`, `memory/`). No cloud/exfil. Backups git-encrypted.
- **Code:** Abstract eng/math framework (modular: core/engine/backtest/risk). No hardcode secrets/tokens.
- **Secrets:** `config/secrets.yml` or .env (gitignored). Load runtime (e.g. pydantic-settings).
- **Engineering:** CI/CD ready (tests 90%+, docs/math proofs, versioned algos).
- **Math:** Rigorous (SDE models, VaR/ES, Sharpe>1.5 target, MC sims). No curve-fit.
- **Success:** Market alpha (live paper-trade first, risk<2%/trade).

## Workflow
1. Idea → Math spec → Backtest (hist 5y+) → Optimize → Paper → Live.
2. Log all (trades/P&L/lessons) → memory/folya/YYYY-MM-DD.md.
3. Review weekly (Sun 08:00 JST).

Main mission: Market success. 🚀