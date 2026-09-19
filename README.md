# gradient-space-time

> Durable trajectory, query, and live-subscription layer for codec-valid Vector15D states (fields 14–15: universal magnetic poles), using SpacetimeDB.

The persistence and synchronization substrate for the Neural-Representation Boundary. Every state that enters this system is validated by [gradient-codec](https://github.com/alamort-sun/gradient-codec) before it is committed. Persistence does not confer validity.

## Architecture

```
External source / STT / client / worker
        ↓
gradient-codec canonicalizes and validates Vector15D
        ↓
SpacetimeDB reducer commits state event and validation receipt
        ↓
Subscriptions notify clients/workers
        ↓
Outside worker runs JEPA inference, MoE routing, LLM calls,
rendering, large analyses, or model training
        ↓
Worker submits result through another reducer
        ↓
gradient-codec validates the result before persistence as valid
```

## Module Structure

```
gradient-space-time/
├── crates/spacetime-module/
│   ├── src/lib.rs          — module entry
│   ├── src/types.rs        — DomainWall, GaugeCoupling, enums
│   ├── src/tables.rs       — 10 typed table definitions
│   └── src/reducers.rs     — 7 reducer functions
├── client/                 — Rust client/worker bindings (TODO)
├── ARCHITECTURE.md
├── SCHEMA.md
├── REDUCERS.md
└── DEPENDENCIES.md
```

## Reducers

Reducers are the transaction boundary and the **only** mechanism for table mutation. They are deterministic, bounded, and side-effect-free.

| Reducer | Purpose |
|---------|---------|
| `submit_validated_state` | Accept canonical state + codec validation receipt |
| `record_transition` | Create explicit x_t → x_t+1 transition metadata |
| `record_jepa_prediction` | Store non-authoritative prediction, confidence |
| `enqueue_generation_request` | Create durable request with budget ceiling |
| `record_generation_result` | Receive worker result, validate through codec |
| `compact_trajectory` | Reproducible compaction record and summary |
| `update_trajectory_summary` | Bounded materialized indicators |

## Tables

10 tables — append-only state events, transitions, trajectories, validation receipts, materialized views, JEPA predictions, routing decisions, generation queue/results, and compaction records. See [SCHEMA.md](SCHEMA.md).

## Authority Boundary

`gradient-codec` is the authoritative implementation of Vector15D
types, enum semantics, invariants, and validity checks.

This repository may learn from, store, query, route, or render
codec-valid states. Learned predictions, database-derived patterns,
and generated outputs are not authoritative and must be validated
against the applicable pinned version of `gradient-codec`.

Persistence does not confer validity.
Every inserted, reconstructed, replayed, or generated Vector15D
state retains codec provenance and must pass codec validation.

## License

- **Software**: PolyForm Noncommercial 1.0.0 — see [LICENSE](LICENSE)
- **Commercial use**: requires a separate written license — see [COMMERCIAL.md](COMMERCIAL.md)

## Status

Schema target: `v15d` (Vector15D). Pre-alpha. Scaffold under construction. SpacetimeDB module skeletons written but not compiled. TODO: pin exact SpacetimeDB version, wire codec validation, implement client bindings.
