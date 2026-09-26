//! Codec gate for commit paths: deserialize → Vector15D::validate → hash.
//!
//! Persistence does not confer validity. Reducers must call
//! [`prepare_validated_payload`] (or [`validate_vector15d`] when already
//! deserialized) before inserting a state event. Failures abort the
//! SpacetimeDB transaction via `Result::Err`.

use sha2::{Digest, Sha256};
use vecGradient::{Vector15D, Vector15DError};

/// Validate an already-deserialized Vector15D via the codec authority.
pub fn validate_vector15d(state: &Vector15D) -> Result<(), String> {
    state
        .validate()
        .map_err(|e: Vector15DError| format!("Vector15D::validate failed: {e}"))
}

/// SHA-256 hex digest of the raw payload bytes (schema: content_hash).
pub fn content_hash_of(payload: &str) -> String {
    let digest = Sha256::digest(payload.as_bytes());
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// Deserialize payload JSON → Vector15D::validate() → content hash.
///
/// Order is intentional: never hash/commit an invalid Vector15D.
/// On Ok, returns the validated state and the sha256 hex of `payload`.
pub fn prepare_validated_payload(payload: &str) -> Result<(Vector15D, String), String> {
    let state: Vector15D = serde_json::from_str(payload)
        .map_err(|e| format!("invalid Vector15D payload: {e}"))?;
    validate_vector15d(&state)?;
    let content_hash = content_hash_of(payload);
    Ok((state, content_hash))
}

#[cfg(test)]
mod tests {
    use super::*;
    use vecGradient::Vector15D;

    fn default_payload() -> String {
        serde_json::to_string(&Vector15D::default()).expect("serialize default")
    }

    #[test]
    fn valid_state_prepares_and_hashes() {
        let payload = default_payload();
        let (state, hash) = prepare_validated_payload(&payload).expect("valid must pass");
        assert!(state.validate().is_ok());
        assert_eq!(hash, content_hash_of(&payload));
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn invalid_json_is_rejected() {
        let err = prepare_validated_payload("not-json").expect_err("garbage must fail");
        assert!(
            err.contains("invalid Vector15D payload"),
            "unexpected err: {err}"
        );
    }

    #[test]
    fn non_finite_vector_is_rejected() {
        let mut bad = Vector15D::default();
        bad.amplitude = f64::NAN;
        let err = validate_vector15d(&bad).expect_err("NaN must fail validate");
        assert!(
            err.contains("Vector15D::validate failed"),
            "unexpected err: {err}"
        );
        assert!(err.contains("amplitude") || err.contains("NaN"));
    }

    #[test]
    fn poles_non_finite_rejected() {
        let mut bad = Vector15D::default();
        bad.magnetic_south = f64::INFINITY;
        assert!(validate_vector15d(&bad).is_err());
    }

    #[test]
    fn content_hash_mismatch_detectable() {
        let payload = default_payload();
        let (_state, computed) = prepare_validated_payload(&payload).unwrap();
        assert_ne!(computed, "deadbeef");
    }
}
