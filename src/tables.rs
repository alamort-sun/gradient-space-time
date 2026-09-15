//! Typed table skeletons.
//!
//! Field lists mirror SCHEMA.md. TODO: replace placeholder types
//! (identity, timestamps, hashes) with the pinned SpacetimeDB SDK's
//! native types once the version is pinned.

// TODO: apply the SDK's table attribute macro to each struct below.
// Verify syntax against the pinned SpacetimeDB SDK examples.

/// Immutable codec-valid Vector13D state events.
pub struct GradientStateEvent {
    pub state_id: (),           // TODO: SDK identity type
    pub trajectory_id: Option<()>,
    pub predecessor_id: Option<()>,
    pub sequence_number: u64,
    pub event_timestamp: (),    // TODO: SDK timestamp type
    pub codec_commit: String,
    pub schema_version: u32,
    pub payload: (),            // TODO: canonical Vector13D representation
    pub content_hash: [u8; 32], // TODO: verify hash type/size policy
    pub validation_outcome: bool,
    pub receipt_id: (),
    pub source_type: SourceType,
}

pub enum SourceType {
    Stt,
    Manual,
    Replay,
    Worker,
    Baseline,
}

/// Explicit x_t → x_t+1 transition records.
pub struct StateTransition {
    pub transition_id: (),
    pub from_state_id: (),
    pub to_state_id: (),
    pub transition_time: (),
    pub duration_ms: i64,
    pub changed_fields: Vec<String>, // TODO: verify array support in pinned SDK
    pub domain_wall_before: crate::DomainWall,
    pub domain_wall_after: crate::DomainWall,
    pub gauge_before: crate::GaugeCoupling,
    pub gauge_after: crate::GaugeCoupling,
    pub receipt_id: (),
}

/// Trajectory metadata — root/terminal references, never unbounded ID arrays.
pub struct Trajectory {
    pub trajectory_id: (),
    pub root_state_id: (),
    pub terminal_state_id: (),
    pub started_at: (),
    pub ended_at: Option<()>,
    pub label: String,
}

/// Codec validation receipts.
pub struct ValidationReceipt {
    pub receipt_id: (),
    pub codec_commit: String,
    pub outcome: bool,
    pub reason_code: String, // TODO: enumerate against codec source
    pub canonical_hash: [u8; 32],
    pub validated_at: (),
    pub source: String,
}

/// Materialized current-state view for live consumers.
pub struct LatestTrajectoryState {
    pub trajectory_id: (),
    pub state_id: (),
    pub as_of: (),
}

/// Bounded materialized summaries.
pub struct TrajectorySummary {
    pub trajectory_id: (),
    pub entropy_delta: f64,
    pub ozone_band: String,
    pub torsion_motion: String,
    pub domain_transitions: u32,
    pub coupling_transitions: u32,
    pub updated_at: (),
}

/// Non-authoritative JEPA predictions.
pub struct JepaPrediction {
    pub prediction_id: (),
    pub model_version: String,
    pub input_state_hash: [u8; 32],
    pub predicted_payload: (), // TODO: representation per MODEL_SPEC from gradient-jelle
    pub confidence: f64,
    pub uncertainty: f64,
    pub authoritative: bool, // always false
}

/// MoE routing decisions.
pub struct RoutingDecision {
    pub decision_id: (),
    pub request_id: (),
    pub selected_expert: String,
    pub budget_snapshot: f64, // TODO: budget structure per gradient-codec BudgetState
    pub rationale_code: String,
    pub outcome: String, // accepted | rerouted | deferred | abstained | rejected
}

/// Durable work-intent queue for external workers.
pub struct GenerationRequest {
    pub request_id: (),
    pub input_state_id: (),
    pub intent: String, // generation | inference | analysis
    pub budget_ceiling_usd: f64,
    pub created_at: (),
    pub status: String, // queued | claimed | completed | expired
}

/// External worker results with codec validation status.
pub struct GenerationResult {
    pub result_id: (),
    pub request_id: (),
    pub worker_id: String,
    pub result_payload: (),
    pub validation_status: String, // accepted | rerouted | abstained | rejected
    pub receipt_id: (),
}

/// Reproducible compaction records.
pub struct CompactionRecord {
    pub compaction_id: (),
    pub trajectory_id: (),
    pub action: String, // reduced | retained | expired | summarized
    pub details: String,
    pub provenance: String,
}
