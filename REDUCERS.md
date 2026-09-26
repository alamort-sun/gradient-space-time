# Reducers — gradient-space-time (A2 plane)

Reducers are the transaction boundary and the **only** mechanism for table
mutation. They are deterministic, bounded, and side-effect-free.

All biography reducers (`submit_validated_state`, `record_transition`,
`record_jepa_prediction`, `enqueue_generation_request`,
`record_generation_result`, `compact_trajectory`,
`update_trajectory_summary`) are **deleted**. No reducer takes
`trajectory_id`.

Timestamps other than `issued_at` / `closed_at` / `emitted_at` (taken from
`ctx.timestamp`) arrive as micros-since-unix-epoch arguments (or via
`Window.start_micros`).

Args use `gradient-plane` newtypes (`LeaseId`, `ActDigest`, `CapsuleDigest`,
`PolicyVersion`, `SecurityEpoch`, `ReceiptId`, `ManifestId`, `Window`,
`SuppressionSpec`, `ClosureCode`).

## issue_active_lease

Issue an `ActiveLease` bound to a capsule digest (no capsule bytes on plane).

```rust
#[reducer]
pub fn issue_active_lease(
    ctx: &ReducerContext,
    capsule_digest: CapsuleDigest,   // 64-char hex sha256 (validated)
    policy_version: PolicyVersion,
    epoch: SecurityEpoch,
    expires_at_micros: i64,          // must be > ctx.timestamp
) -> Result<(), String>
```

Inserts into `active_leases`.

## issue_signaling_grant

Issue a single-use `SignalingGrant` for one act under an existing lease.

```rust
#[reducer]
pub fn issue_signaling_grant(
    ctx: &ReducerContext,
    act_digest: ActDigest,
    lease_ref: LeaseId,              // must exist in active_leases
    policy_version: PolicyVersion,
    epoch: SecurityEpoch,
    expires_at_micros: i64,
) -> Result<(), String>
```

Inserts into `signaling_grants`. Fail-closed if `lease_ref` missing.

## record_closure_receipt

Append a `ClosureReceipt` for a closed act (no text / state / trajectory).

```rust
#[reducer]
pub fn record_closure_receipt(
    ctx: &ReducerContext,
    act_digest: ActDigest,
    code: ClosureCode,
    epoch: SecurityEpoch,
    retention_until_micros: i64,
) -> Result<(), String>
```

Inserts into `closure_receipts`.

## emit_notary_manifest

Emit a `NotaryManifest` over a closed receipt window. Rejects cohorts below
the suppression floor (no rare joinable aggregate).

```rust
#[reducer]
pub fn emit_notary_manifest(
    ctx: &ReducerContext,
    window: Window,                  // start_micros + len_secs (> 0)
    cohort_count: u32,
    suppression_floor: u32,
    suppression_spec: SuppressionSpec,
    ttl_expires_micros: i64,
) -> Result<(), String>
```

Inserts into `notary_manifests`.

## Codec gate (still present)

`validation::prepare_validated_payload` remains for Vector15D deserialize →
`validate` → sha256. It is **not** wired to a plane biography insert anymore;
callers preparing capsule content or act commitments reuse it. Plane reducers
re-validate digest hex via `ActDigest` / `CapsuleDigest::from_hex`.

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
- Writing `trajectory_id`, provider identity, or raw egress text

External workers / capsules handle content. Reducers only commit plane objects.
