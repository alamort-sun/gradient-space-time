//! SpacetimeDB reducers — the transaction boundary (spacetimedb 1.12.0 API).
//!
//! Reducers are the ONLY mechanism for table mutation.
//! Keep reducers deterministic, bounded, side-effect-free.
//!
//! NEVER inside reducers:
//! - LLM API calls, network I/O, Hugging Face downloads
//! - GPU jobs, JEPA training, long-running inference
//! - Filesystem work, unbounded historical scans
//! - Non-replayable hidden side effects
//!
//! In spacetimedb 1.12.0 tables are reached through the context: `ctx.db.{table}()`
//! returns a handle implementing `spacetimedb::Table` (.insert/.delete/.count/.iter).
//! The row timestamp comes from `ctx.timestamp` (Timestamp::now() is stubbed/panics).

use crate::tables::*;
use crate::types::*;
use spacetimedb::{reducer, ReducerContext, Table};

/// Accept canonical state + codec validation receipt.
///
/// External source / STT / client / worker
///    → payload deserialized as Vector15D
///    → Vector15D::validate() (codec authority)
///    → content hash computed / checked
///    → this reducer commits only on pass; Err aborts the transaction
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
) -> Result<(), String> {
    // Gate: deserialize → validate → hash. Never trust the receipt alone.
    let (_state, computed_hash) = crate::validation::prepare_validated_payload(&payload)?;
    if content_hash != computed_hash {
        return Err("content_hash mismatch: provided does not match sha256(payload)".to_string());
    }

    let state_id = next_state_id(ctx);
    let ts = ctx.timestamp;

    ctx.db.gradient_state_events().insert(GradientStateEvent {
        state_id,
        trajectory_id,
        sequence_number,
        event_timestamp: ts,
        codec_version,
        schema_version,
        payload,
        content_hash: computed_hash.clone(),
        validation_outcome: ValidationOutcome::Accepted,
        validation_receipt,
        source_type,
    });

    // Update latest trajectory state (upsert by primary key).
    ctx.db
        .latest_trajectory_state()
        .insert(LatestTrajectoryState {
            trajectory_id,
            state_id,
            content_hash: computed_hash,
            updated_at: ts,
        });

    Ok(())
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
    ctx.db.state_transitions().insert(StateTransition {
        transition_id,
        from_state_id,
        to_state_id,
        trajectory_id,
        timestamp: ctx.timestamp,
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
    ctx.db.jepa_predictions().insert(JepaPrediction {
        prediction_id,
        input_state_hash,
        model_version,
        predicted_representation,
        confidence,
        uncertainty,
        is_authoritative: false, // NEVER authoritative
        timestamp: ctx.timestamp,
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
    ctx.db.generation_requests().insert(GenerationRequest {
        request_id,
        input_state_id,
        budget_ceiling,
        latency_requirement_ms,
        request_type,
        status: "pending".to_string(),
        created_at: ctx.timestamp,
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
) -> Result<(), String> {
    // Codec is the law: if proposed_state is present, validate through vecGradient
    // before persisting as accepted. On validation failure, abort the transaction
    // so the result is not persisted at all (fail-closed).
    if let Some(state_json) = &proposed_state {
        // Validate through the codec gate: deserialize + Vector15D::validate()
        let (_state, _hash) = crate::validation::prepare_validated_payload(state_json)?;
    }

    let result_id = next_result_id(ctx);
    ctx.db.generation_results().insert(GenerationResult {
        result_id,
        request_id,
        generated_text,
        proposed_state,
        validation_status,
        validation_receipt,
        completed_at: ctx.timestamp,
    });

    Ok(())
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
    ctx.db.compaction_records().insert(CompactionRecord {
        compaction_id,
        trajectory_id,
        states_before,
        states_after,
        retained_summary,
        expired_summary,
        timestamp: ctx.timestamp,
    });
}

/// Bounded materialized indicators.
///
/// The reducer arguments carry the codec categorical enums through their
/// SpacetimeDB column wrappers (DomainWall/GaugeCoupling themselves cannot
/// implement SpacetimeType from inside the codec crate, so the boundary absorbs
/// the storage contract). The wrapper unwraps to the canonical codec enum.
#[reducer]
pub fn update_trajectory_summary(
    ctx: &ReducerContext,
    trajectory_id: u64,
    state_count: u64,
    avg_entropy: f64,
    avg_coherence: f64,
    domain_wall: DomainWallColumn,
    gauge_coupling: GaugeCouplingColumn,
    avg_hue: f64,
) {
    // Upsert by primary key (trajectory_id): delete the prior row, then insert.
    ctx.db.trajectory_summaries().delete(TrajectorySummary {
        trajectory_id,
        state_count: 0,
        avg_entropy: 0.0,
        avg_coherence: 0.0,
        domain_wall: DomainWall::default().into(),
        gauge_coupling: GaugeCoupling::default().into(),
        avg_hue: 0.0,
        updated_at: ctx.timestamp,
    });
    ctx.db.trajectory_summaries().insert(TrajectorySummary {
        trajectory_id,
        state_count,
        avg_entropy,
        avg_coherence,
        domain_wall,
        gauge_coupling,
        avg_hue,
        updated_at: ctx.timestamp,
    });
}

// --- ID generation (deterministic, bounded) ---
// TODO: Replace with proper SpacetimeDB auto-increment or sequence.
// These are placeholder implementations using table row count.

fn next_state_id(ctx: &ReducerContext) -> u64 {
    ctx.db.gradient_state_events().count() + 1
}

fn next_transition_id(ctx: &ReducerContext) -> u64 {
    ctx.db.state_transitions().count() + 1
}

fn next_prediction_id(ctx: &ReducerContext) -> u64 {
    ctx.db.jepa_predictions().count() + 1
}

fn next_request_id(ctx: &ReducerContext) -> u64 {
    ctx.db.generation_requests().count() + 1
}

fn next_result_id(ctx: &ReducerContext) -> u64 {
    ctx.db.generation_results().count() + 1
}

fn next_compaction_id(ctx: &ReducerContext) -> u64 {
    ctx.db.compaction_records().count() + 1
}
