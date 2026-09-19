# Architecture — gradient-space-time

## Overview

gradient-space-time is the durable trajectory, query, and live-subscription layer for codec-valid Vector15D states. It uses [SpacetimeDB](https://spacetimedb.com) as the transactional persistence and synchronization substrate.

## Architecture Boundary

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

## Design Principles

1. **Reducers are the transaction boundary.** The only mechanism for table mutation. Deterministic, bounded, side-effect-free.

2. **Append-only state events with explicit provenance.** Every persisted valid state carries state ID, trajectory ID, sequence number, timestamp, codec version, schema version, payload, content hash, validation outcome, validation receipt, and source type.

3. **Typed enum-safe values.** DomainWall and GaugeCoupling are categorical types. Never flattened to scalars. Strongly typed SpacetimeDB table rows.

4. **No JSONB assumptions.** Do not assume PostgreSQL JSONB syntax/semantics. Use SpacetimeDB-native types.

5. **Persistence does not confer validity.** Every inserted, reconstructed, replayed, or generated Vector15D state retains codec provenance and must pass codec validation.

## What NEVER Goes Inside Reducers

- LLM API calls
- Network I/O
- Hugging Face downloads
- GPU jobs
- JEPA training
- Long-running inference
- Filesystem work
- Unbounded historical scans
- Non-replayable hidden side effects

External workers handle all of the above. Reducers only commit validated results.

## SpacetimeDB Integration

- Use SpacetimeDB Rust modules and generated Rust client bindings.
- Do NOT fork or modify SpacetimeDB internals.
- Pin the exact SpacetimeDB version. See DEPENDENCIES.md.

## Live Subscription Bridge

Clients and workers subscribe to SpacetimeDB tables for live updates. This enables:
- gradient-speak rendering to subscribe to state changes for live glyph updates
- External workers to pick up generation requests via subscription
- Real-time trajectory monitoring

## Provenance and Replay

Every state event carries its codec version and validation receipt. This enables:
- Deterministic replay of trajectories
- Provenance tracking across the full pipeline
- Auditability of what was validated, when, and against which codec version


## Codec geometry (v15d)

Persisted states are **Vector15D**. `magnetic_north` / `magnetic_south` are universal polar anchors for every shell. Persistence stores codec-valid JSON only; it does not confer validity. Old 13-field payloads upgrade by defaulting poles to 0.0.
