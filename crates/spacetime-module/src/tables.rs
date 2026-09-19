//! SpacetimeDB table definitions.
//!
//! Favor append-only state events with explicit provenance over mutable
//! opaque blobs. Every persisted valid state carries provenance and a
//! validation receipt.

use spacetimedb::table;
use crate::types::*;

/// Immutable codec-valid Vector15D state events.
///
/// Every persisted valid state carries:
/// state ID · trajectory ID · logical sequence number
/// · event timestamp · codec version/commit · schema version
/// · canonical payload · content hash
/// · validation outcome · validation receipt / reason code
/// · source type / provenance
#[table(name = gradient_state_events, public)]
pub struct GradientStateEvent {
    /// Unique state ID.
    #[primary_key]
    pub state_id: u64,
    /// Trajectory ID (group of related states).
    pub trajectory_id: u64,
    /// Logical sequence number within trajectory.
    pub sequence_number: u64,
    /// Event timestamp.
    pub event_timestamp: Timestamp,
    /// Codec version/commit hash.
    pub codec_version: String,
    /// Schema version.
    pub schema_version: String,
    /// Canonical Vector15D payload (serialized).
    pub payload: String,
    /// Content hash (sha256 of payload).
    pub content_hash: String,
    /// Validation outcome.
    pub validation_outcome: ValidationOutcome,
    /// Validation receipt / reason code.
    pub validation_receipt: String,
    /// Source type / provenance.
    pub source_type: SourceType,
}

/// Explicit x_t → x_t+1 transition records.
#[table(name = state_transitions, public)]
pub struct StateTransition {
    #[primary_key]
    pub transition_id: u64,
    /// Predecessor state ID.
    pub from_state_id: u64,
    /// Successor state ID.
    pub to_state_id: u64,
    /// Trajectory ID.
    pub trajectory_id: u64,
    /// Transition timestamp.
    pub timestamp: Timestamp,
}

/// Trajectory metadata and root/terminal references.
#[table(name = trajectories, public)]
pub struct Trajectory {
    #[primary_key]
    pub trajectory_id: u64,
    /// Root state ID (first state in trajectory).
    pub root_state_id: Option<u64>,
    /// Terminal state ID (last state, if closed).
    pub terminal_state_id: Option<u64>,
    /// Whether the trajectory is closed.
    pub is_closed: bool,
    /// Created timestamp.
    pub created_at: Timestamp,
    /// Closure value (0.0 - 1.0).
    pub closure: f64,
}

/// Codec commit/version, outcome, invariant info.
#[table(name = validation_receipts, public)]
pub struct ValidationReceipt {
    #[primary_key]
    pub receipt_id: u64,
    /// State ID validated.
    pub state_id: u64,
    /// Codec commit hash.
    pub codec_commit: String,
    /// Codec schema version.
    pub codec_schema_version: String,
    /// Whether validation passed.
    pub is_valid: bool,
    /// Reason code.
    pub reason_code: String,
    /// Timestamp.
    pub timestamp: Timestamp,
}

/// Optional materialized current-state view.
#[table(name = latest_trajectory_state, public)]
pub struct LatestTrajectoryState {
    #[primary_key]
    pub trajectory_id: u64,
    /// Latest state ID.
    pub state_id: u64,
    /// Content hash of latest state.
    pub content_hash: String,
    /// Updated timestamp.
    pub updated_at: Timestamp,
}

/// Bounded materialized summaries.
#[table(name = trajectory_summaries, public)]
pub struct TrajectorySummary {
    #[primary_key]
    pub trajectory_id: u64,
    /// Number of states.
    pub state_count: u64,
    /// Average entropy across trajectory.
    pub avg_entropy: f64,
    /// Average coherence.
    pub avg_coherence: f64,
    /// Domain wall state (constant per trajectory if Linked throughout).
    pub domain_wall: DomainWall,
    /// Gauge coupling mode.
    pub gauge_coupling: GaugeCoupling,
    /// Average hue.
    pub avg_hue: f64,
    /// Updated timestamp.
    pub updated_at: Timestamp,
}

/// Non-authoritative JEPA predictions.
#[table(name = jepa_predictions, public)]
pub struct JepaPrediction {
    #[primary_key]
    pub prediction_id: u64,
    /// Input state hash.
    pub input_state_hash: String,
    /// Model version.
    pub model_version: String,
    /// Predicted representation (serialized).
    pub predicted_representation: String,
    /// Confidence (0.0 - 1.0).
    pub confidence: f64,
    /// Uncertainty estimate.
    pub uncertainty: f64,
    /// Explicitly non-authoritative.
    pub is_authoritative: bool, // always false
    /// Timestamp.
    pub timestamp: Timestamp,
}

/// MoE routing decisions.
#[table(name = routing_decisions, public)]
pub struct RoutingDecisionRecord {
    #[primary_key]
    pub routing_id: u64,
    /// Selected expert/provider.
    pub selected_expert: String,
    /// Budget snapshot.
    pub budget_snapshot: String,
    /// Rationale code.
    pub rationale_code: String,
    /// Request linkage (generation request ID, if any).
    pub request_id: Option<u64>,
    /// Result outcome.
    pub result_outcome: String,
    /// Timestamp.
    pub timestamp: Timestamp,
}

/// Durable work-intent queue for external workers.
#[table(name = generation_requests, public)]
pub struct GenerationRequest {
    #[primary_key]
    pub request_id: u64,
    /// Input state ID.
    pub input_state_id: u64,
    /// Budget ceiling (USD).
    pub budget_ceiling: f64,
    /// Latency requirement (ms).
    pub latency_requirement_ms: Option<u32>,
    /// Request type.
    pub request_type: String,
    /// Status: pending, in_progress, complete, failed.
    pub status: String,
    /// Created timestamp.
    pub created_at: Timestamp,
}

/// External result records with codec validation status.
#[table(name = generation_results, public)]
pub struct GenerationResult {
    #[primary_key]
    pub result_id: u64,
    /// Original generation request ID.
    pub request_id: u64,
    /// Generated text (if any).
    pub generated_text: Option<String>,
    /// Proposed output state (serialized Vector15D).
    pub proposed_state: Option<String>,
    /// Codec validation status.
    pub validation_status: GenerationStatus,
    /// Validation receipt.
    pub validation_receipt: String,
    /// Completed timestamp.
    pub completed_at: Timestamp,
}

/// What was reduced, retained, expired, or summarized.
#[table(name = compaction_records, public)]
pub struct CompactionRecord {
    #[primary_key]
    pub compaction_id: u64,
    /// Trajectory ID compacted.
    pub trajectory_id: u64,
    /// Number of states before compaction.
    pub states_before: u64,
    /// Number of states after compaction.
    pub states_after: u64,
    /// What was retained.
    pub retained_summary: String,
    /// What was expired.
    pub expired_summary: String,
    /// Timestamp.
    pub timestamp: Timestamp,
}
