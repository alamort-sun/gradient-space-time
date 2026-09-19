# Schema — gradient-space-time

## Tables

### gradient_state_events

Immutable codec-valid Vector15D state events.

### Vector15D payload (codec law)

Persisted `payload` JSON is **Vector15D**. Fields 1–13 retain prior semantics. Fields 14–15:

| Field | Semantics |
|-------|-----------|
| `magnetic_north` | Universal polar pre-stress (north) — shared by all shells |
| `magnetic_south` | Universal polar pre-stress (south) — shared by all shells |

Poles are **not** seat-owned. Anaseos ivory-blue is DECLARED colour/spectrum only. Serde defaults missing poles to `0.0`. Prefer `codec_schema_version` / `schema_version` = `v15d`.



| Column | Type | Description |
|--------|------|-------------|
| state_id | u64 (PK) | Unique state ID |
| trajectory_id | u64 | Trajectory ID |
| sequence_number | u64 | Logical sequence within trajectory |
| event_timestamp | Timestamp | When event was committed |
| codec_version | String | Codec commit hash |
| schema_version | String | Schema version |
| payload | String | Canonical Vector15D (serialized JSON) |
| content_hash | String | sha256 of payload |
| validation_outcome | ValidationOutcome | Accepted / Rerouted / Abstained / Rejected |
| validation_receipt | String | Reason code / receipt |
| source_type | SourceType | Observed / Generated / Predicted / Synthetic / Replayed |

### state_transitions

Explicit x_t → x_t+1 records.

| Column | Type | Description |
|--------|------|-------------|
| transition_id | u64 (PK) | Unique transition ID |
| from_state_id | u64 | Predecessor state |
| to_state_id | u64 | Successor state |
| trajectory_id | u64 | Trajectory ID |
| timestamp | Timestamp | When transition was recorded |

### trajectories

Metadata and root/terminal references.

| Column | Type | Description |
|--------|------|-------------|
| trajectory_id | u64 (PK) | Trajectory ID |
| root_state_id | Option\<u64\> | First state (None if not yet set) |
| terminal_state_id | Option\<u64\> | Last state (None if open) |
| is_closed | bool | Whether trajectory is closed |
| created_at | Timestamp | Creation time |
| closure | f64 | Cycle completeness (0.0 - 1.0) |

### validation_receipts

Codec commit/version, outcome, invariant info.

| Column | Type | Description |
|--------|------|-------------|
| receipt_id | u64 (PK) | Receipt ID |
| state_id | u64 | State validated |
| codec_commit | String | Codec commit hash |
| codec_schema_version | String | Codec schema version |
| is_valid | bool | Whether validation passed |
| reason_code | String | Reason code |
| timestamp | Timestamp | When validated |

### latest_trajectory_state

Optional materialized current-state view.

| Column | Type | Description |
|--------|------|-------------|
| trajectory_id | u64 (PK) | Trajectory ID |
| state_id | u64 | Latest state ID |
| content_hash | String | Content hash of latest state |
| updated_at | Timestamp | When updated |

### trajectory_summaries

Bounded materialized summaries.

| Column | Type | Description |
|--------|------|-------------|
| trajectory_id | u64 (PK) | Trajectory ID |
| state_count | u64 | Number of states |
| avg_entropy | f64 | Average entropy |
| avg_coherence | f64 | Average coherence |
| domain_wall | DomainWall | Connection state |
| gauge_coupling | GaugeCoupling | Rotation mode |
| avg_hue | f64 | Average hue |
| updated_at | Timestamp | When updated |

### jepa_predictions

Non-authoritative model predictions.

| Column | Type | Description |
|--------|------|-------------|
| prediction_id | u64 (PK) | Prediction ID |
| input_state_hash | String | Input state hash |
| model_version | String | Model version |
| predicted_representation | String | Predicted state (serialized) |
| confidence | f64 | Confidence (0.0 - 1.0) |
| uncertainty | f64 | Uncertainty estimate |
| is_authoritative | bool | Always false |
| timestamp | Timestamp | When predicted |

### routing_decisions

MoE decision records.

| Column | Type | Description |
|--------|------|-------------|
| routing_id | u64 (PK) | Routing ID |
| selected_expert | String | Selected provider |
| budget_snapshot | String | Budget state snapshot |
| rationale_code | String | Routing rationale |
| request_id | Option\<u64\> | Generation request linkage |
| result_outcome | String | Result outcome |
| timestamp | Timestamp | When routed |

### generation_requests

Durable work-intent queue for external workers.

| Column | Type | Description |
|--------|------|-------------|
| request_id | u64 (PK) | Request ID |
| input_state_id | u64 | Input state |
| budget_ceiling | f64 | Budget ceiling (USD) |
| latency_requirement_ms | Option\<u32\> | Latency requirement |
| request_type | String | Request type |
| status | String | pending / in_progress / complete / failed |
| created_at | Timestamp | When created |

### generation_results

External result records with codec validation status.

| Column | Type | Description |
|--------|------|-------------|
| result_id | u64 (PK) | Result ID |
| request_id | u64 | Original request |
| generated_text | Option\<String\> | Generated text |
| proposed_state | Option\<String\> | Proposed output state |
| validation_status | GenerationStatus | Accepted / Rerouted / Abstained / Rejected |
| validation_receipt | String | Validation receipt |
| completed_at | Timestamp | When completed |

### compaction_records

What was reduced, retained, expired, or summarized.

| Column | Type | Description |
|--------|------|-------------|
| compaction_id | u64 (PK) | Compaction ID |
| trajectory_id | u64 | Trajectory compacted |
| states_before | u64 | States before compaction |
| states_after | u64 | States after compaction |
| retained_summary | String | What was retained |
| expired_summary | String | What was expired |
| timestamp | Timestamp | When compacted |

## Enums

### DomainWall
`Linked` | `Broken` | `Gradient`

### GaugeCoupling
`Static` | `Spinning` | `Oscillating`

### ValidationOutcome
`Accepted` | `Rerouted` | `Abstained` | `Rejected`

### SourceType
`Observed` | `Generated` | `Predicted` | `Synthetic` | `Replayed`

### GenerationStatus
`Accepted` | `Rerouted` | `Abstained` | `Rejected`

## Personality ledgers (live on gray-fog maincloud)

Agent personality for jelle seats is **not** stored in this scaffold module yet.
It lives on SpacetimeDB **maincloud** database `gray-fog` (sun-dance memory):

- **Federated** — keyed by `seat_id`
- **Timestamped** — `agent_memory_5d` append history
- **Lossy mutable** — `agent_memory_4d` replace/merge spine + `agent_memory_6d` fading cache

Jelle-serve pulls/pushes `personality:<seat>` there. Codec-valid Vector15D trajectories remain this repo's concern.
