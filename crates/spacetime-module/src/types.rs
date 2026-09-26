//! Plane-safe column types for the A2 four-object set (Saraswati).
//!
//! Authority: workspace path dep `gradient-plane` (from `../gradient-codec/plane`)
//! with `spacetimedb` feature for column derive. No bare u64/hex String stand-ins
//! on the four plane tables — see `tables.rs`.
//!
//! Do not reintroduce biography master keys, request-chain keys, provider
//! identity, or raw egress text on these types.

pub use gradient_plane::{
    require_digest_hex, ActDigest, CapsuleDigest, ClosureCode, LeaseId, ManifestId,
    PolicyVersion, ReceiptId, SecurityEpoch, SuppressionSpec, Window, DIGEST_HEX_LEN,
};

/// Codec schema version label (geometry payloads still use v15d).
pub const CODEC_SCHEMA_VERSION: &str = "v15d";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_hex_accepts_sha256() {
        let d = "a".repeat(64);
        require_digest_hex("act_digest", &d).expect("hex ok");
        ActDigest::from_hex(d).expect("act");
    }

    #[test]
    fn digest_hex_rejects_short() {
        let err = require_digest_hex("act_digest", "deadbeef").expect_err("short");
        assert!(err.contains("expected 64"));
    }

    #[test]
    fn digest_hex_rejects_non_hex() {
        let d = "g".repeat(64);
        let err = require_digest_hex("capsule_digest", &d).expect_err("non-hex");
        assert!(err.contains("hexadecimal"));
    }

    #[test]
    fn biography_master_key_absent_as_field() {
        let key = ["trajectory", "id"].join("_");
        let col = format!("pub {key}:");
        let param = format!("{key}:");
        for (label, src) in [
            ("tables.rs", include_str!("tables.rs")),
            ("reducers.rs", include_str!("reducers.rs")),
        ] {
            assert!(
                !src.lines().any(|l| {
                    let t = l.trim_start();
                    t.starts_with(&col) || t.starts_with(&param)
                }),
                "{label} must not declare column/param {key}"
            );
        }
    }

    #[test]
    fn tables_use_plane_newtypes_not_bare_standins() {
        let tables = include_str!("tables.rs");
        // Banned bare stand-in column types on the A2 set.
        for needle in [
            "pub lease_id: u64",
            "pub lease_ref: u64",
            "pub receipt_id: u64",
            "pub manifest_id: u64",
            "pub epoch: u64",
            "pub policy_version: u32",
            "pub act_digest: String",
            "pub capsule_digest: String",
            "pub suppression_spec: String",
        ] {
            assert!(
                !tables.contains(needle),
                "tables.rs still has stand-in `{needle}` — use gradient-plane newtypes"
            );
        }
        for needle in [
            "LeaseId",
            "ActDigest",
            "CapsuleDigest",
            "ReceiptId",
            "ManifestId",
            "PolicyVersion",
            "SecurityEpoch",
            "SuppressionSpec",
            "Window",
        ] {
            assert!(
                tables.contains(needle),
                "tables.rs should reference plane newtype {needle}"
            );
        }
    }
}
