//! gradient-space-time — SpacetimeDB module root
//!
//! Persistence does not confer validity. Every inserted, reconstructed,
//! replayed, or generated Vector13D state retains codec provenance and
//! must pass codec validation against the pinned gradient-codec commit.
//!
//! STATUS: skeleton. TODOs mark every point requiring source verification
//! against (1) the pinned gradient-codec commit and (2) the pinned
//! SpacetimeDB SDK version. Do not remove a TODO without citing the
//! exact source that resolves it.

pub mod tables;
pub mod reducers;

/// Mirror of gradient-codec `vector13d::DomainWall` (field 9, connection).
/// MUST stay in sync with the pinned codec commit — see DEPENDENCIES.md.
/// TODO: verify enum support in the pinned SpacetimeDB SDK; if enums are
/// not natively supported in table rows, use a typed u8 newtype with
/// explicit conversion functions at this boundary — never free-form strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainWall {
    Linked,
    Broken,
    Gradient,
}

/// Mirror of gradient-codec `vector13d::GaugeCoupling` (field 12, rotation).
/// Same sync rule as DomainWall.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GaugeCoupling {
    Static,
    Spinning,
    Oscillating,
}

/// Pinned codec commit this module was built against.
/// TODO: keep in lockstep with DEPENDENCIES.md and re-verify on every re-pin.
pub const CODEC_PIN: &str = "9f4b4d5";

/// Schema version for every persisted row.
pub const SCHEMA_VERSION: u32 = 1;
