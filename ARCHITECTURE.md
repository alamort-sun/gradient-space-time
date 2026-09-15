# ARCHITECTURE

> gradient-codec defines valid geometry. gradient-space-time persists and replays under that geometry. No prediction, provider, database row, or renderer supersedes the codec.

## The law

- `gradient-codec` is the authoritative implementation of Vector13D types, enum semantics, invariants, and validity checks.
- Persistence does not confer validity. Every inserted, reconstructed, replayed, or generated Vector13D state retains codec provenance and must pass codec validation.
- Reducers are the transaction boundary and the only mechanism for table mutation. Reducers stay deterministic, bounded, and side-effect-free.

## Architecture boundary

```
External source / STT / client / worker
        ↓
gradient-codec canonicalizes and validates Vector13D
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

## Component responsibilities

| Component | Owns | Never does |
|---|---|---|
| gradient-codec (pinned commit) | Vector13D type, enums, invariants, validation, observed baselines, routing policy | — |
| SpacetimeDB module (this repo) | Tables, reducers, subscriptions, provenance, compaction | LLM calls, network I/O, GPU work, training |
| External workers | JEPA inference, MoE routing, LLM generation, rendering, training, heavy analysis | Direct table mutation (must go through reducers) |
| gradient-speak | Live rendering from subscriptions | Treating unvalidated model output as canonical state |

## Worker protocol

Durable work-intent queue, not in-memory dispatch:

1. A client or reducer writes a `generation_requests` row (valid input state + budget ceiling + intent).
2. External workers subscribe to `generation_requests`, claim work, and execute outside the database.
3. The worker submits its result via the `record_generation_result` reducer.
4. The reducer validates any proposed Vector13D output through gradient-codec (or verifies the supplied validation receipt per authority policy) and records the outcome: accepted / rerouted / abstained / rejected.

## Provenance and trace/replay

Every accepted state is replayable from provenance alone:

- `gradient_state_events` are immutable and append-only.
- Each event carries codec version/commit, schema version, content hash, and a validation receipt reference.
- `state_transitions` record explicit x_t → x_t+1 edges with changed-field metadata.
- `trajectories` link ordered histories via root/terminal references — never unbounded ID arrays.
- `compaction_records` make summarization reproducible: what was reduced, retained, expired, and why.

Replay rule: reconstructing any historical state must reproduce the same content hash, or the replay is invalid.

## Live subscription bridge (gradient-speak)

gradient-speak subscribes to:

- `gradient_state_events` (filtered to codec-valid rows)
- `latest_trajectory_state` (materialized current view)
- `trajectory_summaries` (bounded aggregates)

Unvalidated rows (e.g. `jepa_predictions`, rejected `generation_results`) are never rendered as canonical state.

## Versioning

- Codec pin recorded in [DEPENDENCIES.md](DEPENDENCIES.md); re-pin deltas are emitted by the Artemis evidence ledger.
- Schema version lives on every persisted row; migrations are additive and provenance-preserving.
