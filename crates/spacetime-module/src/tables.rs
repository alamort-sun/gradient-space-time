//! SpacetimeDB table definitions — Saraswati A2 plane objects only.
//!
//! Biography set (11 tables keyed by `trajectory_id` / request chains) deleted.
//! Capsule content (state events, transitions, JEPA predictions, raw text)
//! does **not** live here — see SCHEMA.md TODOs. Plane holds leases, grants,
//! receipts, and manifests.

use crate::types::ClosureCode;
use spacetimedb::{table, Timestamp};

/// Right to hold/work one trajectory capsule for a bounded window.
/// Immutable once issued. Carries no trajectory content / no trajectory_id.
#[table(name = active_leases, public)]
pub struct ActiveLease {
    /// Opaque lease id (never reused). Storage stand-in for `LeaseId`.
    #[primary_key]
    pub lease_id: u64,
    /// sha256 hex of capsule content — binds without storing capsule bytes.
    pub capsule_digest: String,
    /// Monotonic policy version.
    pub policy_version: u32,
    /// Fenced security epoch.
    pub epoch: u64,
    /// Issue time.
    pub issued_at: Timestamp,
    /// Mandatory expiry — no perpetual leases.
    pub expires_at: Timestamp,
}

/// Authorization for exactly one external signaling act.
/// Durable form is digest-family only (no provider, prompt, cost, request_id).
#[table(name = signaling_grants, public)]
pub struct SignalingGrant {
    /// sha256 hex act digest (primary; one grant per act).
    #[primary_key]
    pub act_digest: String,
    /// Lease this act serves (purpose-scoped equality only).
    pub lease_ref: u64,
    pub policy_version: u32,
    pub epoch: u64,
    pub issued_at: Timestamp,
    /// Single-use by construction: expiry <= one act horizon.
    pub expires_at: Timestamp,
}

/// Append-only, bounded-retention record that an act closed with a code.
/// No state IDs, text, provider, or trajectory_id.
#[table(name = closure_receipts, public)]
pub struct ClosureReceipt {
    #[primary_key]
    pub receipt_id: u64,
    /// What closed — bound, not described.
    pub act_digest: String,
    pub code: ClosureCode,
    pub epoch: u64,
    pub closed_at: Timestamp,
    /// Bounded retention; deletion event required at expiry (TODO ledger).
    pub retention_until: Timestamp,
}

/// Manifest over a closed window of receipts (analytics/export crossing).
/// Compaction history folds in here — not a trajectory_id-keyed row.
#[table(name = notary_manifests, public)]
pub struct NotaryManifest {
    #[primary_key]
    pub manifest_id: u64,
    /// Window start (inclusive).
    pub window_start: Timestamp,
    /// Window length in seconds.
    pub window_len_secs: u64,
    /// Cohort count (suppressed below floor at emission time).
    pub cohort_count: u32,
    /// Suppression floor applied (R3: 100).
    pub suppression_floor: u32,
    /// Opaque suppression notes (rare-category / timing / joinability).
    pub suppression_spec: String,
    pub emitted_at: Timestamp,
    /// Deletion complete by this deadline (recorded event — TODO).
    pub ttl_expires: Timestamp,
}
