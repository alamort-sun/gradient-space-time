# Schema — gradient-space-time (Saraswati A2 plane)

Plane-safe SpacetimeDB schema. **Four coordinator objects.** The former
11-table biography set and its `trajectory_id` master key are deleted.
Capsule content does not live on the plane.

Authority: `agent/specs/p1-codec-schema-lane-saraswati.md` §2–§3;
`agent/specs/susano-dry-audit-v13.md` (delete joinable fields; do not sprout
object-types on the same join key).

Storage stand-ins: opaque `u64` / hex digests until codec newtypes
(`LeaseId`, `ActDigest`, `ReceiptId`, `ManifestId`, …) land in
`gradient-codec`. Equality is purpose-scoped (`lease_ref` / `act_digest`) —
there is no master key.

## Plane tables (4)

### active_leases (`ActiveLease`)

Right to hold/work one trajectory capsule for a bounded window. Immutable
once issued. No trajectory content, no principal, no seat.

| Column | Type | Description |
|--------|------|-------------|
| lease_id | u64 (PK) | Opaque lease id (never reused) |
| capsule_digest | String | sha256 hex of capsule content — binds without storing |
| policy_version | u32 | Monotonic policy version |
| epoch | u64 | Fenced security epoch |
| issued_at | Timestamp | Issue time |
| expires_at | Timestamp | Mandatory expiry — no perpetual leases |

Consume/expiry state is **not** a mutable field here (TODO: no-rewind
consume ledger keyed by `lease_id`).

### signaling_grants (`SignalingGrant`)

Authorization for exactly one external signaling act. Digest-family only.

| Column | Type | Description |
|--------|------|-------------|
| act_digest | String (PK) | sha256 hex over normalized act commitment |
| lease_ref | u64 | Lease this act serves |
| policy_version | u32 | Policy version |
| epoch | u64 | Security epoch |
| issued_at | Timestamp | Issue time |
| expires_at | Timestamp | Single-use horizon |

No provider identity, prompt text, token counts, costs, or `request_id`.

### closure_receipts (`ClosureReceipt`)

Append-only, bounded-retention record that an act closed with a code.

| Column | Type | Description |
|--------|------|-------------|
| receipt_id | u64 (PK) | Opaque receipt id |
| act_digest | String | What closed — bound, not described |
| code | ClosureCode | Accepted / Rerouted / Abstained / Rejected |
| epoch | u64 | Security epoch |
| closed_at | Timestamp | Close time |
| retention_until | Timestamp | Bounded retention deadline |

No state IDs, raw text, provider, or `trajectory_id`.

### notary_manifests (`NotaryManifest`)

Manifest over a closed window of receipts (analytics/export crossing).
Compaction history folds in as manifest lines + tombstones (TODO), not a
`compaction_records` row keyed by `trajectory_id`.

| Column | Type | Description |
|--------|------|-------------|
| manifest_id | u64 (PK) | Fresh id per window (never reused) |
| window_start | Timestamp | Window start |
| window_len_secs | u64 | Window length (R3 default 86400) |
| cohort_count | u32 | Suppressed below floor at emission |
| suppression_floor | u32 | R3 floor (100) |
| suppression_spec | String | Rare-category / timing / joinability notes |
| emitted_at | Timestamp | Emission time |
| ttl_expires | Timestamp | Deletion-complete deadline (R3 external TTL 604800) |

## Enums

### ClosureCode
`Accepted` | `Rerouted` | `Abstained` | `Rejected`

(Unified former `ValidationOutcome` / `GenerationStatus`.)

## Deleted biography set (disposition)

| Former table | Disposition |
|---|---|
| `gradient_state_events` | **moved out** → capsule under `ActiveLease` (TODO capsule crate) |
| `state_transitions` | **moved out** → capsule |
| `trajectories` | **replaced** by `active_leases` |
| `validation_receipts` | **replaced** by `closure_receipts` |
| `latest_trajectory_state` | **deleted** (B1) |
| `trajectory_summaries` | **replaced** by `notary_manifests` |
| `jepa_predictions` | **moved out** → research capsule (TODO) |
| `routing_decisions` | **replaced** by `signaling_grants` + `closure_receipts` |
| `generation_requests` | **replaced** by ephemeral grant (no durable work queue) |
| `generation_results` | **replaced** by `closure_receipts` (`generated_text` → capsule or drop) |
| `compaction_records` | **replaced** by manifest lines + tombstones under `notary_manifests` |

`trajectory_id` is **gone** from schema and reducers.

## TODOs (blockers for full A2 / B2)

1. **Capsule crate / runtime** — holds state events, transitions, JEPA,
   raw egress under a lease; plane only stores `capsule_digest`.
2. **Codec newtypes** — `LeaseId` / `ActDigest` / `PolicyVersion` /
   `SecurityEpoch` / `ReceiptId` / `ManifestId` / `Window` /
   `SuppressionSpec` in `gradient-codec` (space-time currently uses
   storage stand-ins).
3. **Epoch root fencing** — reject writes whose `epoch` ≠ current root.
4. **Lease consume ledger** — no-rewind consume/expiry rows keyed by
   `lease_id` (not a mutable field on `ActiveLease`).
5. **Manifest tombstones / deletion-complete events** — R3 deletion
   recorded, not silent drop.
6. **B2 dual-write** — legacy frozen → readers cut over → drop (precondition:
   pins + single canonical module tree; tree drift already resolved).
7. **A5 CI** — deny-list scan for `trajectory_id`, `trace_id`,
   `prompt_hash`, `generated_text` on plane tables.

## Personality ledgers (live on gray-fog maincloud)

Agent personality for jelle seats is **not** stored in this module.
It lives on SpacetimeDB **maincloud** database `gray-fog` (sun-dance memory):

- **Federated** — keyed by `seat_id`
- **Timestamped** — `agent_memory_5d` append history
- **Lossy mutable** — `agent_memory_4d` replace/merge spine + `agent_memory_6d` fading cache

Jelle-serve pulls/pushes `personality:<seat>` there. Codec-valid Vector15D
geometry remains the codec crate's concern; this module no longer persists
trajectory biographies.
