# SCHEMA

> Append-only state events with explicit provenance. Never mutable opaque state blobs.

## Schema philosophy

- Favor **append-only state events** and explicit provenance over mutable state.
- Every persisted valid state carries:

```text
state ID · trajectory ID or predecessor reference · logical sequence number
· event timestamp · codec version/commit · schema version
· canonical payload · content hash
· validation outcome · validation receipt / reason code
· source type / provenance
```

- `DomainWall` and `GaugeCoupling` are **typed enum-safe values**, not free-form strings, at the codec boundary.
- Strongly typed SpacetimeDB table rows. No assumption of PostgreSQL JSONB syntax or semantics — SpacetimeDB-native types only.
- TODO: verify exact SpacetimeDB type mappings (timestamp, hash, array, enum support) against the pinned SDK version before first build.

## Tables

### gradient_state_events

Immutable codec-valid Vector13D state events.

```text
state_id            identity
trajectory_id       reference to trajectories (nullable for unassigned)
predecessor_id      reference to prior state event (nullable for roots)
sequence_number     logical order within trajectory
event_timestamp     time of observation
codec_commit        pinned gradient-codec commit (e.g. 9f4b4d5)
schema_version      this schema's version
payload             canonical serialized Vector13D
content_hash        hash of canonical payload
validation_outcome  accepted | rejected
receipt_id          reference to validation_receipts
source_type         stt | manual | replay | worker | baseline
```

### state_transitions

Explicit x_t → x_t+1 records.

```text
transition_id       identity
from_state_id       reference to gradient_state_events
to_state_id         reference to gradient_state_events
transition_time     timestamp of the transition
duration_ms         time between observations
changed_fields      list of field names that changed
domain_wall_before  Linked | Broken | Gradient
domain_wall_after   Linked | Broken | Gradient
gauge_before        Static | Spinning | Oscillating
gauge_after         Static | Spinning | Oscillating
receipt_id          validation reference
```

### trajectories

Metadata and root/terminal references for ordered histories. Never depends solely on unbounded arrays of state IDs.

```text
trajectory_id       identity
root_state_id       first state event
terminal_state_id   latest state event (bounded materialized reference)
started_at          timestamp
ended_at            timestamp (nullable while open)
label               e.g. 'n1-baseline', 'shower', 'stt-demo'
```

### validation_receipts

```text
receipt_id          identity
codec_commit        pinned codec version
outcome             accepted | rejected
reason_code         invariant / rule identifier (TODO: enumerate against codec source)
canonical_hash      content hash of the validated payload
validated_at        timestamp
source              provenance of the validation call
```

### latest_trajectory_state

Optional materialized current-state view for fast live consumers.

```text
trajectory_id       identity (one row per trajectory)
state_id            reference to current gradient_state_events
as_of               materialization timestamp
```

### trajectory_summaries

Bounded / materialized summaries.

```text
trajectory_id       identity
entropy_delta       first→latest entropy change
ozone_band          energy band summary
torsion_motion      skew evolution summary
domain_transitions  count of domain_wall changes
coupling_transitions count of gauge_coupling changes
updated_at          materialization timestamp
```

### jepa_predictions

Non-authoritative model predictions.

```text
prediction_id       identity
model_version       JEPA artifact manifest identifier
input_state_hash    content hash of the input state
predicted_payload   predicted representation (latent or state)
confidence          model-reported confidence
uncertainty         model-reported uncertainty
authoritative       always false — predictions never validate geometry
```

### routing_decisions

```text
decision_id         identity
request_id          linkage to generation_requests
selected_expert     provider/model selected
budget_snapshot     ceiling + spent at decision time
rationale_code      MoE rationale identifier
outcome             accepted | rerouted | deferred | abstained | rejected
```

### generation_requests

Durable work-intent queue for external workers.

```text
request_id          identity
input_state_id      reference to gradient_state_events
intent              generation | inference | analysis
budget_ceiling_usd  max spend authorized
created_at          timestamp
status              queued | claimed | completed | expired
```

### generation_results

```text
result_id           identity
request_id          linkage to generation_requests
worker_id           provenance of the worker
result_payload      worker output
validation_status   accepted | rerouted | abstained | rejected
receipt_id          validation reference
```

### compaction_records

```text
compaction_id       identity
trajectory_id       affected trajectory
action              reduced | retained | expired | summarized
details             reproducible description of what was done
provenance          references needed to reproduce the compaction
```
