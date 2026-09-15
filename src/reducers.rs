//! Reducer skeletons.
//!
//! Reducers are the transaction boundary and the only mechanism for table
//! mutation. Deterministic, bounded, side-effect-free. Never: LLM calls,
//! network I/O, downloads, GPU jobs, training, long-running inference,
//! filesystem work, unbounded historical scans, non-replayable side effects.

// TODO: apply the SDK's reducer attribute macro to each function below.
// Verify syntax and Context/argument conventions against the pinned SDK.

/// Accept a canonical state + codec validation receipt. Revalidate or
/// verify per authority policy. Append a state event, link predecessor /
/// trajectory when valid, update bounded current and summary views.
pub fn submit_validated_state(
    _ctx: (),                 // TODO: SDK Context type
    _canonical_state: (),     // TODO: canonical Vector13D payload
    _validation_receipt: (),  // TODO: receipt structure
) {
    // TODO: authority policy decision — revalidate in-module vs verify
    // supplied receipt. Either way: rejected states are recorded, not
    // silently dropped (provenance for the rejection).
    todo!("see REDUCERS.md §submit_validated_state")
}

/// Create explicit x_t → x_t+1 transition metadata after validation.
pub fn record_transition(
    _ctx: (),
    _from_state_id: (),
    _to_state_id: (),
) {
    // TODO: compute changed_fields, domain_wall before/after, gauge
    // before/after from the two referenced state events.
    todo!("see REDUCERS.md §record_transition")
}

/// Store a non-authoritative JEPA prediction (confidence, model version,
/// input-state hash). Marked non-authoritative at write time.
pub fn record_jepa_prediction(
    _ctx: (),
    _model_version: String,
    _input_state_hash: [u8; 32],
    _predicted_payload: (),
    _confidence: f64,
    _uncertainty: f64,
) {
    todo!("see REDUCERS.md §record_jepa_prediction")
}

/// Create a durable generation request (work-intent queue for workers).
pub fn enqueue_generation_request(
    _ctx: (),
    _input_state_id: (),
    _intent: String,
    _budget_ceiling_usd: f64,
) {
    todo!("see REDUCERS.md §enqueue_generation_request")
}

/// Receive a result from an outside worker. Validate any proposed state
/// through gradient-codec. Record accepted / rerouted / abstained / rejected.
pub fn record_generation_result(
    _ctx: (),
    _request_id: (),
    _worker_id: String,
    _result_payload: (),
) {
    // TODO: codec validation of any proposed Vector13D output before
    // persistence as valid. Rejected results are recorded with receipts.
    todo!("see REDUCERS.md §record_generation_result")
}

/// Produce a reproducible compaction record and summary. Never destroy
/// authoritative provenance without an explicit retention policy.
pub fn compact_trajectory(
    _ctx: (),
    _trajectory_id: (),
) {
    todo!("see REDUCERS.md §compact_trajectory")
}

/// Update bounded materialized indicators: entropy delta, ozone-buffer
/// movement, torsion movement, domain-wall transitions, coupling transitions.
pub fn update_trajectory_summary(
    _ctx: (),
    _trajectory_id: (),
) {
    // TODO: bounded window or cursor-based update — never unbounded scans.
    todo!("see REDUCERS.md §update_trajectory_summary")
}
