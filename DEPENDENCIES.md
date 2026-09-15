# DEPENDENCIES

> Third-party terms are never relabeled. Original terms, provenance, version, and source are recorded here.

## gradient-codec

- Source: https://github.com/alamort-sun/gradient-codec (in-ecosystem, PolyForm Noncommercial policy family)
- Pin: `9f4b4d5` (observed 2026-09-15; re-pin before use — upstream moves same-day)
- Provides: Vector13D type, DomainWall/GaugeCoupling enums, observed baselines (n1, n2), validation authority
- Note: the `vector13d` crate (workspace member) is serde-only; enum-safe types at this boundary must stay in sync with the pinned commit. TODO: decide vendoring vs. git dependency vs. type-mirror + content-hash verification at insert time.

## SpacetimeDB

- Source: https://spacetimedb.com (Clockwork Labs)
- Version: **TODO — pin exact SDK/engine version before first build**
- Terms: TODO — record SpacetimeDB engine licensing and additional-use terms verbatim. Treat as a separate dependency concern; preserve notices.
- Usage: Rust module (tables, reducers, subscriptions) + generated Rust client bindings. No forks or internal modifications unless a documented required capability cannot be achieved through the module API.

## serde / serde_json (transitive via vector13d)

- MIT OR Apache-2.0 — standard dual license; notices preserved via cargo metadata.

## Policy

- Software source in this repository: PolyForm Noncommercial 1.0.0 (see LICENSE).
- Commercial use requires a separate written license (see COMMERCIAL.md).
- Intentionally unrestricted artifacts (synthetic fixtures, golden vectors, public schemas, protocol examples) may be marked CC0-1.0 individually.
- Never record a third-party item under a license it did not come with.
