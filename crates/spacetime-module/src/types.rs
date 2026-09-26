//! Typed column definitions and codec-faithful categorical bridges (spacetimedb 1.12.0).
//!
//! The canonical categorical types `DomainWall` / `GaugeCoupling` live in the codec
//! authority (`vecGradient`, POLYFORM-NONCOMMERCIAL, the law). That crate cannot take a
//! `spacetimedb` dependency, so these storage-side wrappers encode the codec enum at the
//! boundary: a faithful, canonical variant-name string. The codec enum remains the single
//! source of truth; this is purely a storage serialization seam.
//!
//! The `#[sats(...)]` derive is left without `crate =`, so it uses the crate default
//! fallback `spacetimedb::spacetimedb_lib`, which re-exports `sats`, `de`, `ser`,
//! `SpacetimeType`, `Private`, `FilterableValue`, and `DirectIndexKey` (the complete set
//! the proc-macro emits). `spacetimedb` and `spacetimedb_sats` each re-export only a
//! partial set, which is why an explicit `crate =` pointed at either fails.

use spacetimedb::SpacetimeType;

/// Re-export the canonical codec types (the authority).
pub use vecGradient::{DomainWall, GaugeCoupling};

/// Storage column for codec `DomainWall`: the canonical variant name as a string.
///
/// Encodes/decodes losslessly against the codec enum. Unknown strings fail to decode
/// (`to_codec` returns `None`) rather than silently mapping to a default, so the codec
/// authority is never quietly overridden by the storage layer.
#[derive(Debug, Clone, PartialEq, Eq, SpacetimeType)]
#[sats(name = "DomainWall")]
pub struct DomainWallColumn {
    pub value: String,
}

impl From<DomainWall> for DomainWallColumn {
    fn from(d: DomainWall) -> Self {
        let s = match d {
            DomainWall::Linked => "Linked",
            DomainWall::Broken => "Broken",
            DomainWall::Gradient => "Gradient",
        };
        DomainWallColumn {
            value: s.to_string(),
        }
    }
}

impl DomainWallColumn {
    /// Decode back to the canonical codec enum. `None` for any string the codec does not
    /// define — the storage layer never invents a category.
    pub fn to_codec(&self) -> Option<DomainWall> {
        match self.value.as_str() {
            "Linked" => Some(DomainWall::Linked),
            "Broken" => Some(DomainWall::Broken),
            "Gradient" => Some(DomainWall::Gradient),
            _ => None,
        }
    }
}

/// Storage column for codec `GaugeCoupling`: the canonical variant name as a string.
#[derive(Debug, Clone, PartialEq, Eq, SpacetimeType)]
#[sats(name = "GaugeCoupling")]
pub struct GaugeCouplingColumn {
    pub value: String,
}

impl From<GaugeCoupling> for GaugeCouplingColumn {
    fn from(g: GaugeCoupling) -> Self {
        let s = match g {
            GaugeCoupling::Static => "Static",
            GaugeCoupling::Spinning => "Spinning",
            GaugeCoupling::Oscillating => "Oscillating",
        };
        GaugeCouplingColumn {
            value: s.to_string(),
        }
    }
}

impl GaugeCouplingColumn {
    pub fn to_codec(&self) -> Option<GaugeCoupling> {
        match self.value.as_str() {
            "Static" => Some(GaugeCoupling::Static),
            "Spinning" => Some(GaugeCoupling::Spinning),
            "Oscillating" => Some(GaugeCoupling::Oscillating),
            _ => None,
        }
    }
}

/// Closure disposition for a validated state or generation result.
///
/// Single enum for both `gradient_state_events.validation_outcome` and
/// `generation_results.validation_status` (formerly ValidationOutcome /
/// GenerationStatus).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, SpacetimeType)]
#[sats(name = "ClosureCode")]
pub enum ClosureCode {
    #[default]
    Accepted,
    Rerouted,
    Abstained,
    Rejected,
}

/// Source type / provenance of a state event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, SpacetimeType)]
#[sats(name = "SourceType")]
pub enum SourceType {
    #[default]
    Observed,
    Generated,
    Predicted,
    Synthetic,
    Replayed,
}

/// Codec schema version for persisted Vector15D payloads.
pub const CODEC_SCHEMA_VERSION: &str = "v15d";
