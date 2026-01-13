# Axionism

Framework to build an Adaptive Vault Manager Algorithm for Morpho Blue Protocol.

## Data Sources

- Collateral

Bybit: Orderbook, PublicTrades, Liquidations, FundingRates





```shell
axionism/
│
├── CHANGELOG.md                  # tracker of changes for each version
├── Cargo.toml                    # workspace root
├── LICENSE                       # apache v2.0
├── MANIFEST.in                   # cargo related instructions
├── Makefile.toml                 # specs for katex-header and docs
├── README.md                     # this file
├── katex-header.html             # enable KaTeX for math symbols on docs.rs
├── pyproject.toml                # specs for integration with python
├── rustfmt.toml                  # rust formatting specs
│
├── axios-rs/
│   ├── src/                      # Domain primitives
│   │   ├── types.rs              # Market, Vault, Position, IRM params
│   │   ├── morpho.rs             # Protocol-specific logic (LLTV, utilization curves)
│   │   └── lib.rs
│   │
│   ├── data/                     # Data acquisition layer
│   │   ├── rpc.rs                # alloy-based EVM calls
│   │   ├── indexer.rs            # Subgraph/custom indexer client
│   │   ├── oracles.rs            # Chainlink, Pyth, etc.
│   │   └── cache.rs              # Redis/TimescaleDB connectors
│   │
│   ├── analytics/                # Processing + competitor profiling
│   │   ├── market_state.rs       # Aggregated market snapshots
│   │   ├── competitor.rs         # Vault/allocator behavior extraction
│   │   ├── flow_analysis.rs      # Supply/withdraw patterns, MEV detection
│   │   └── risk.rs               # VaR, liquidation probability, correlation
│   │
│   ├── game/                     # Game-theoretic modeling
│   │   ├── payoff.rs             # Utility functions (yield, risk-adjusted return)
│   │   ├── equilibrium.rs        # Nash/Stackelberg solvers
│   │   ├── mechanism.rs          # IRM response modeling
│   │   └── simulation.rs         # Agent-based Monte Carlo
│   │
│   ├── allocator/                # Policy/strategy layer
│   │   ├── policy.rs             # Trait-based allocator interface
│   │   ├── strategies/
│   │   │   ├── greedy.rs
│   │   │   ├── mvo.rs            # Mean-variance optimization
│   │   │   ├── rl.rs             # RL policy wrapper (calls Python)
│   │   │   └── game_theoretic.rs # Equilibrium-aware allocation
│   │   └── rebalancer.rs         # Execution logic, gas optimization
│   │
│   ├── monitor/                  # Observability
│   │   ├── metrics.rs            # Prometheus/OTEL exports
│   │   ├── alerts.rs             # Threshold-based triggers
│   │   └── pnl.rs                # Real-time P&L tracking
│
├── axios-py/
│   └── morpho_strategist/
│       ├── __init__.py
│       ├── backtest.py           # Vectorbt/custom backtester
│       ├── rl/                   # Gymnasium env, PPO/SAC policies
│       │   ├── env.py
│       │   └── train.py
│       ├── notebooks/            # Research & visualization
│       └── cli.py                # Typer-based CLI
│
└── config/
    ├── markets.toml              # Morpho market configs
    └── strategy.toml             # Allocator parameters
```

