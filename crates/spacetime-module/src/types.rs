//! Plane-safe column types for the A2 four-object set (Saraswati).
//!
//! SpacetimeDB storage seam: opaque u64 / hex-string digests stand in for
//! codec newtypes (`LeaseId`, `ActDigest`, …) until those land in
//! `gradient-codec`. Do not reintroduce biography master keys, request-chain
//! keys, provider identity, or raw egress text on these types.

use spacetimedb::SpacetimeType;

/// Closure disposition for an act (Saraswati A2 `ClosureCode`).
///
/// Former `ValidationOutcome` / `GenerationStatus` collapsed here (B1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, SpacetimeType)]
#[sats(name = "ClosureCode")]
pub enum ClosureCode {
    #[default]
    Accepted,
    Rerouted,
    Abstained,
    Rejected,
}

/// Codec schema version label (geometry payloads still use v15d).
pub const CODEC_SCHEMA_VERSION: &str = "v15d";

/// Hex-encoded sha256 digest length (capsule / act digests).
pub const DIGEST_HEX_LEN: usize = 64;

/// Reject digests that are not lowercase/uppercase hex sha256.
pub fn require_digest_hex(label: &str, digest: &str) -> Result<(), String> {
    if digest.len() != DIGEST_HEX_LEN {
        return Err(format!(
            "{label}: expected {DIGEST_HEX_LEN}-char hex digest, got len {}",
            digest.len()
        ));
    }
    if !digest.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!("{label}: digest must be hexadecimal"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_hex_accepts_sha256() {
        let d = "a".repeat(64);
        require_digest_hex("act_digest", &d).expect("hex ok");
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
}
