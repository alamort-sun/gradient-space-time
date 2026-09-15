# REDUCERS

> Reducers are the transaction boundary and the only mechanism for table mutation. Deterministic, bounded, side-effect-free.

## Never inside reducers

```text
LLM API calls
Network I/O
Hugging Face downloads
GPU jobs
JEPA training
Long-running inference
Filesystem work
Unbounded historical scans
Global clustering across unrestricted history
Non-replayable hidden side effects
```

Those belong in clients or external workers that subscribe to durable request/event tables and later submit deterministic results through reducers.

## Reducers

### submit_validated_state

Accepts a canonical state + supplied codec validation receipt. Revalidates or verifies according to authority policy. Appends a state event. Links predecessor / trajectory when valid. Updates bounded current and summary views.

### record_transition

Creates explicit transition metadata after validation: predecessor, successor, duration, changed fields, domain-wall before/after, gauge-coupling before/after.

### record_jepa_prediction

Stores a non-authoritative model prediction: confidence, model version, input-state hash. The row is marked non-authoritative at write time; predictions never validate geometry.

### enqueue_generation_request

Creates a durable request with budget ceiling and valid input state. Workers subscribe to this table; no in-memory dispatch.

### record_generation_result

Receives a result from an outside worker. Validates any proposed Vector13D state through gradient-codec. Records accepted, rerouted, abstained, or rejected.

### compact_trajectory

Produces a reproducible compaction record and summary. Never destroys authoritative provenance without an explicit retention policy.

### update_trajectory_summary

Updates bounded materialized indicators: entropy delta, ozone-buffer movement, torsion movement, domain-wall transition count, coupling transition count.

## Analysis reducers (planned)

Beyond persistence, bounded reducers for structured queries:

- **Trajectory reducer** — compress state sequences into transition patterns
- **Domain reducer** — aggregate by domain_wall regime (Linked / Broken / Gradient)
- **Energy reducer** — compute energy flux from ozone_buffer deltas over time
- **Entropy reducer** — track entropy change trajectories (n1→n2 reference: >55% drop)
- **Torsion reducer** — track skew evolution (torsion → 0 = upright axis)

All analysis reducers must remain bounded: windowed or cursor-based, never unbounded historical scans.
