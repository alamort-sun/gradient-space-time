# gradient-space-time

> Vector13D states stored, queried, and reduced in SpaceTimeDB — the persistence and temporal-query layer of the Gradient ecosystem. Every state has a timestamp, a position, and a trajectory.

## Authority Boundary

`gradient-codec` is the authoritative implementation of Vector13D types, enum semantics, invariants, and validity checks.

This repository may learn from, store, query, route, or render codec-valid states. Learned predictions, database-derived patterns, and generated outputs are not authoritative and must be validated against the applicable pinned version of `gradient-codec`.

Persistence does not confer validity. Every inserted, reconstructed, replayed, or generated Vector13D state retains codec provenance and must pass codec validation.

## What it is

gradient-space-time encodes and decodes gradient-codec's Vector13D using SpaceTimeDB reducers, tables, and schemas. It gives the 13-field affective telemetry a **spacetime home**: states are stored with temporal coordinates, queried by time and region, and transformed by reducers into trajectory patterns, domain clusters, and energy flows.

## Why it works

Vector13D already carries time inside it:

| Field | Temporal meaning |
|---|---|
| frequency (2) | temporal rate |
| phase (3) | temporal position |
| resonance (7) | temporal harmonic |
| ozone_buffer (8) | energy level (time-dependent) |
| domain_wall (9) | connection state (persists or transitions over time) |
| gauge_coupling (12) | rotation mode (changes over time) |

SpaceTimeDB is purpose-built for this: spatial + temporal data with live queries, reducers, and typed schemas.

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

See [ARCHITECTURE.md](ARCHITECTURE.md), [SCHEMA.md](SCHEMA.md), [REDUCERS.md](REDUCERS.md).

## Integration points

- **[gradient-codec](https://github.com/alamort-sun/gradient-codec)** — the Vector13D type, the encoder; states flow in from codec output
- **[gradient-speak](https://github.com/alamort-sun/gradient-speak)** — visualization layer; SpaceTimeDB live queries drive live rendering
- **[gradient-jelle](https://github.com/alamort-sun/gradient-jelle)** — JEPA training data; historical queries supply state-pair trajectories

## Use cases

- **Real-time analytics** — SpaceTimeDB handles streaming inserts efficiently
- **Historical query** — "find all states where hue shifted pink→purple"
- **Pattern mining** — recurring transition patterns across time
- **JEPA training data** — hold out domain-formation trajectories for evaluation
- **Visualization feed** — live queries into gradient-speak's renderer

## Roadmap

- [x] Repo created (public) and transferred to alamort-sun
- [x] License applied (PolyForm Noncommercial 1.0.0) + COMMERCIAL.md
- [x] Module scaffold: tables, reducers, docs
- [ ] Pin SpacetimeDB SDK version and verify attribute syntax
- [ ] Implement reducers (trajectory, domain, energy, entropy, torsion)
- [ ] Codec integration: Vector13D → state-event insert pipeline
- [ ] Live query bridge to gradient-speak
- [ ] Historical dataset from observed baselines (n1, n2)
- [ ] STT demo integration: real-time states from audio

## Station assignments

| Station | Role |
|---|---|
| Artemis | Repo scaffold, schema design, codec integration |
| Saraswati | Reducer implementation, query design |
| Athena | License policy, schema review |
| Fantasia | Trace/replay: reproducible state history |
| Sun | Strategic direction, STT demo data source |

## License

Software source: **PolyForm Noncommercial 1.0.0** — see [LICENSE](LICENSE). Commercial use, hosting, deployment, distribution, resale, or incorporation into a commercial product/service requires a separate written license — see [COMMERCIAL.md](COMMERCIAL.md). Third-party dependency terms are recorded in [DEPENDENCIES.md](DEPENDENCIES.md).

## License and permitted use

The software is licensed under the [PolyForm Noncommercial License 1.0.0](LICENSE).

Individuals, hobbyists, students, independent researchers, educators, nonprofits, and community projects are welcome to use, modify, and share the software for noncommercial purposes. See [USE-POLICY.md](USE-POLICY.md).

Commercial use requires a separate written license. See [COMMERCIAL.md](COMMERCIAL.md).
