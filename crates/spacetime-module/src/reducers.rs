//! SpacetimeDB reducers — transaction boundary for A2 plane objects.
//!
//! Thin insert wrappers. No trajectory_id, no request_id master key, no
//! provider identity, no raw egress text.
//!
//! NEVER inside reducers:
//! - LLM API calls, network I/O, Hugging Face downloads
//! - GPU jobs, JEPA training, long-running inference
//! - Filesystem work, unbounded historical scans
//! - Non-replayable hidden side effects

use crate::tables::*;
use crate::types::{require_digest_hex, ClosureCode};
use spacetimedb::{reducer, ReducerContext, Table, Timestamp};

/// Issue an `ActiveLease` bound to a capsule digest (no capsule bytes on plane).
///
/// `expires_at_micros` is absolute micros-since-unix-epoch (timestamp as arg;
/// no wall-clock read inside the reducer beyond `ctx.timestamp` for issued_at).
#[reducer]
pub fn issue_active_lease(
    ctx: &ReducerContext,
    capsule_digest: String,
    policy_version: u32,
    epoch: u64,
    expires_at_micros: i64,
) -> Result<(), String> {
    require_digest_hex("capsule_digest", &capsule_digest)?;
    let expires_at = Timestamp::from_micros_since_unix_epoch(expires_at_micros);
    if expires_at <= ctx.timestamp {
        return Err("expires_at must be strictly after issued_at".to_string());
    }

    let lease_id = next_lease_id(ctx);
    ctx.db.active_leases().insert(ActiveLease {
        lease_id,
        capsule_digest,
        policy_version,
        epoch,
        issued_at: ctx.timestamp,
        expires_at,
    });
    Ok(())
}

/// Issue a single-use `SignalingGrant` for one act digest under a lease.
#[reducer]
pub fn issue_signaling_grant(
    ctx: &ReducerContext,
    act_digest: String,
    lease_ref: u64,
    policy_version: u32,
    epoch: u64,
    expires_at_micros: i64,
) -> Result<(), String> {
    require_digest_hex("act_digest", &act_digest)?;
    let expires_at = Timestamp::from_micros_since_unix_epoch(expires_at_micros);
    if expires_at <= ctx.timestamp {
        return Err("expires_at must be strictly after issued_at".to_string());
    }
    // Lease must exist (purpose-scoped ref; not a biography join key).
    if ctx.db.active_leases().lease_id().find(lease_ref).is_none() {
        return Err(format!("lease_ref {lease_ref} not found"));
    }

    ctx.db.signaling_grants().insert(SignalingGrant {
        act_digest,
        lease_ref,
        policy_version,
        epoch,
        issued_at: ctx.timestamp,
        expires_at,
    });
    Ok(())
}

/// Append a `ClosureReceipt` for a closed act (no text / state / trajectory).
#[reducer]
pub fn record_closure_receipt(
    ctx: &ReducerContext,
    act_digest: String,
    code: ClosureCode,
    epoch: u64,
    retention_until_micros: i64,
) -> Result<(), String> {
    require_digest_hex("act_digest", &act_digest)?;
    let retention_until = Timestamp::from_micros_since_unix_epoch(retention_until_micros);
    if retention_until < ctx.timestamp {
        return Err("retention_until must not precede closed_at".to_string());
    }

    let receipt_id = next_receipt_id(ctx);
    ctx.db.closure_receipts().insert(ClosureReceipt {
        receipt_id,
        act_digest,
        code,
        epoch,
        closed_at: ctx.timestamp,
        retention_until,
    });
    Ok(())
}

/// Emit a `NotaryManifest` over a closed receipt window (R3 floor applies).
#[reducer]
pub fn emit_notary_manifest(
    ctx: &ReducerContext,
    window_start_micros: i64,
    window_len_secs: u64,
    cohort_count: u32,
    suppression_floor: u32,
    suppression_spec: String,
    ttl_expires_micros: i64,
) -> Result<(), String> {
    if window_len_secs == 0 {
        return Err("window_len_secs must be > 0".to_string());
    }
    // Suppress below floor: do not emit a joinable rare cohort.
    if cohort_count < suppression_floor {
        return Err(format!(
            "cohort_count {cohort_count} below suppression_floor {suppression_floor}"
        ));
    }

    let window_start = Timestamp::from_micros_since_unix_epoch(window_start_micros);
    let ttl_expires = Timestamp::from_micros_since_unix_epoch(ttl_expires_micros);
    let manifest_id = next_manifest_id(ctx);
    ctx.db.notary_manifests().insert(NotaryManifest {
        manifest_id,
        window_start,
        window_len_secs,
        cohort_count,
        suppression_floor,
        suppression_spec,
        emitted_at: ctx.timestamp,
        ttl_expires,
    });
    Ok(())
}

// --- ID generation (deterministic, bounded) ---
// TODO: Replace with SpacetimeDB auto-increment / sequence when available.

fn next_lease_id(ctx: &ReducerContext) -> u64 {
    ctx.db.active_leases().count() + 1
}

fn next_receipt_id(ctx: &ReducerContext) -> u64 {
    ctx.db.closure_receipts().count() + 1
}

fn next_manifest_id(ctx: &ReducerContext) -> u64 {
    ctx.db.notary_manifests().count() + 1
}
