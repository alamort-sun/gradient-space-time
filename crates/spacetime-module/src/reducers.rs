//! SpacetimeDB reducers — the transaction boundary.
//!
//! Reducers are the ONLY mechanism for table mutation.
//! Keep reducers deterministic, bounded, side-effect-free.
//!
//! NEVER inside reducers:
//! - LLM API calls, network I/O, Hugging Face downloads
//! - GPU jobs, JEPA training, long-running inference
//! - Filesystem work, unbounded historical scans
//! - Non-replayable hidden side effects

use spacetimedb::{reducer, ReducerContext, Table, Timestamp};
use crate::tables::*;
use crate::types::*;

/// Accept canonical state + codec validation receipt.
///
/// External source / STT / client / worker
///   → gradient-codec canonicalizes and validates Vector15D
///   → this reducer commits the validated state event
#[reducer]
pub fn submit_validated_state(
    ctx: &ReducerContext,
    trajectory_id: u64,
    sequence_number: u64,
    payload: String,
    content_hash: String,
    codec_version: String,
    schema_version: String,
    validation_receipt: String,
    source_type: SourceType,
) {
    let state_id = next_state_id(ctx);

    gradient_state_events::insert(GradientStateEvent {
        state_id,
        trajectory_id,
        sequence_number,
        event_timestamp: Timestamp::now(),
        codec_version,
        schema_version,
        payload,
        content_hash,
        validation_outcome: ValidationOutcome::Accepted,
        validation_receipt,
        source_type,
    });

    // Update latest trajectory state.
    latest_trajectory_state::delete(trajectory_id);
    latest_trajectory_state::insert(LatestTrajectoryState {
        trajectory_id,
        state_id,
        content_hash: content_hash.clone(),
        updated_at: Timestamp::now(),
    });
}

/// Create explicit transition metadata after validation.
#[reducer]
pub fn record_transition(
    ctx: &ReducerContext,
    trajectory_id: u64,
    from_state_id: u64,
    to_state_id: u64,
) {
    let transition_id = next_transition_id(ctx);

    state_transitions::insert(StateTransition {
        transition_id,
        from_state_id,
        to_state_id,
        trajectory_id,
        timestamp: Timestamp::now(),
    });
}

/// Store non-authoritative prediction, confidence.
#[reducer]
pub fn record_jepa_prediction(
    ctx: &ReducerContext,
    input_state_hash: String,
    model_version: String,
    predicted_representation: String,
    confidence: f64,
    uncertainty: f64,
) {
    let prediction_id = next_prediction_id(ctx);

    jepa_predictions::insert(JepaPrediction {
        prediction_id,
        input_state_hash,
        model_version,
        predicted_representation,
        confidence,
        uncertainty,
        is_authoritative: false, // NEVER authoritative
        timestamp: Timestamp::now(),
    });
}

/// Create durable request with budget ceiling.
#[reducer]
pub fn enqueue_generation_request(
    ctx: &ReducerContext,
    input_state_id: u64,
    budget_ceiling: f64,
    latency_requirement_ms: Option<u32>,
    request_type: String,
) {
    let request_id = next_request_id(ctx);

    generation_requests::insert(GenerationRequest {
        request_id,
        input_state_id,
        budget_ceiling,
        latency_requirement_ms,
        request_type,
        status: "pending".to_string(),
        created_at: Timestamp::now(),
    });
}

/// Receive worker result. Validate through codec.
///
/// Worker submits result through this reducer.
/// gradient-codec validates the result before persistence as valid.
#[reducer]
pub fn record_generation_result(
    ctx: &ReducerContext,
    request_id: u64,
    generated_text: Option<String>,
    proposed_state: Option<String>,
    validation_status: GenerationStatus,
    validation_receipt: String,
) {
    let result_id = next_result_id(ctx);

    generation_results::insert(GenerationResult {
        result_id,
        request_id,
        generated_text,
        proposed_state,
        validation_status,
        validation_receipt,
        completed_at: Timestamp::now(),
    });

    // If the result was accepted and has a proposed state, persist it.
    // TODO: Wire codec validation here. The codec is the law.
    // Deserialize proposed_state as Vector15D (crate vector13d::Vector15D alias Vector13D),
    // call validate() / try_new*_checked paths; reject non-finite including magnetic poles.
    // Prefer schema_version == types::CODEC_SCHEMA_VERSION ("v15d").
}

/// Reproducible compaction record and summary.
#[reducer]
pub fn compact_trajectory(
    ctx: &ReducerContext,
    trajectory_id: u64,
    states_before: u64,
    states_after: u64,
    retained_summary: String,
    expired_summary: String,
) {
    let compaction_id = next_compaction_id(ctx);

    compaction_records::insert(CompactionRecord {
        compaction_id,
        trajectory_id,
        states_before,
        states_after,
        retained_summary,
        expired_summary,
        timestamp: Timestamp::now(),
    });
}

/// Bounded materialized indicators.
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
) {
    trajectory_summaries::delete(trajectory_id);
    trajectory_summaries::insert(TrajectorySummary {
        trajectory_id,
        state_count,
        avg_entropy,
        avg_coherence,
        domain_wall,
        gauge_coupling,
        avg_hue,
        updated_at: Timestamp::now(),
    });
}

// --- ID generation (deterministic, bounded) ---
// TODO: Replace with proper SpacetimeDB auto-increment or sequence.
// These are placeholder implementations using table row count.

fn next_state_id(ctx: &ReducerContext) -> u64 {
    gradient_state_events::count() + 1
}

fn next_transition_id(ctx: &ReducerContext) -> u64 {
    state_transitions::count() + 1
}

fn next_prediction_id(ctx: &ReducerContext) -> u64 {
    jepa_predictions::count() + 1
}

fn next_request_id(ctx: &ReducerContext) -> u64 {
    generation_requests::count() + 1
}

fn next_result_id(ctx: &ReducerContext) -> u64 {
    generation_results::count() + 1
}

fn next_compaction_id(ctx: &ReducerContext) -> u64 {
    compaction_records::count() + 1
}
