# Reducers — gradient-space-time

Reducers are the transaction boundary and the **only** mechanism for table mutation. They are deterministic, bounded, and side-effect-free.

## submit_validated_state

Accept canonical state + codec validation receipt.

```rust
#[reducer]
pub fn submit_validated_state(
    ctx: &ReducerContext,
    trajectory_id: u64,
    sequence_number: u64,
    payload: String,          // canonical Vector15D (serialized JSON)
    content_hash: String,    // sha256 of payload
    codec_version: String,    // codec commit hash
    schema_version: String,
    validation_receipt: String,
    source_type: SourceType,
)
```

Inserts into `gradient_state_events` and updates `latest_trajectory_state`.

## record_transition

Create explicit transition metadata after validation.

```rust
#[reducer]
pub fn record_transition(
    ctx: &ReducerContext,
    trajectory_id: u64,
    from_state_id: u64,
    to_state_id: u64,
)
```

Inserts into `state_transitions`.

## record_jepa_prediction

Store non-authoritative prediction, confidence.

```rust
#[reducer]
pub fn record_jepa_prediction(
    ctx: &ReducerContext,
    input_state_hash: String,
    model_version: String,
    predicted_representation: String,
    confidence: f64,
    uncertainty: f64,
)
```

Inserts into `jepa_predictions`. The `is_authoritative` field is always `false`.

## enqueue_generation_request

Create durable request with budget ceiling.

```rust
#[reducer]
pub fn enqueue_generation_request(
    ctx: &ReducerContext,
    input_state_id: u64,
    budget_ceiling: f64,
    latency_requirement_ms: Option<u32>,
    request_type: String,
)
```

Inserts into `generation_requests` with status "pending".

## record_generation_result

Receive worker result. Validate through codec.

```rust
#[reducer]
pub fn record_generation_result(
    ctx: &ReducerContext,
    request_id: u64,
    generated_text: Option<String>,
    proposed_state: Option<String>,
    validation_status: GenerationStatus,
    validation_receipt: String,
)
```

Inserts into `generation_results`. If the result was accepted and has a proposed state, it should be persisted via `submit_validated_state` after codec validation. TODO: Wire codec validation.

## compact_trajectory

Reproducible compaction record and summary.

```rust
#[reducer]
pub fn compact_trajectory(
    ctx: &ReducerContext,
    trajectory_id: u64,
    states_before: u64,
    states_after: u64,
    retained_summary: String,
    expired_summary: String,
)
```

Inserts into `compaction_records`. The compaction itself must be deterministic and reproducible.

## update_trajectory_summary

Bounded materialized indicators.

```rust
#[reducer]
pub fn update_trajectory_summary(
    ctx: &ReducerContext,
    trajectory_id: u64,
    state_count: u64,
    avg_entropy: f64,
    avg_coherence: f64,
    domain_wall: DomainWall,
    gauge_coupling: GaugeCoupling,
    avg_hue: f64,
)
```

Upserts into `trajectory_summaries`.

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
