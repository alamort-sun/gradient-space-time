# Dependencies — gradient-space-time

## Rust Dependencies

| Crate | Version | License | Purpose |
|-------|---------|---------|---------|
| spacetimedb | TODO: pin exact version | Apache-2.0 | Transactional persistence and synchronization |
| serde | 1.0 | MIT OR Apache-2.0 | Serialization/deserialization |
| serde_json | 1.0 | MIT OR Apache-2.0 | JSON serialization |
| thiserror | 1.0 | MIT OR Apache-2.0 | Error derive macros |
| tracing | 0.1 | MIT | Structured logging |

## External Dependencies

| Dependency | Version | License | Source |
|------------|---------|---------|--------|
| SpacetimeDB | TODO: pin | AGPL-3.0 | https://spacetimedb.com |
| gradient-codec | TODO: pin commit | PolyForm Noncommercial | https://github.com/alamort-sun/gradient-codec |

## Rules

1. **Pin the exact SpacetimeDB version.** Do not use floating version ranges. The pinned version must be recorded here and in Cargo.toml.

2. **Do not fork or modify SpacetimeDB internals.** Use SpacetimeDB Rust modules and generated Rust client bindings as-is.

3. **Never relabel third-party dependencies as PolyForm or CC0.** Record their original terms, provenance, version, and source.

4. **SpacetimeDB license note.** SpacetimeDB is licensed under its own terms. This repository's PolyForm Noncommercial license applies only to the code written in this repository, not to SpacetimeDB itself or any other third-party dependency.

5. **gradient-codec dependency.** gradient-space-time depends on gradient-codec for Vector15D validation (crate still named `vector13d`; type `Vector15D`, alias `Vector13D`). The codec version must be pinned to a specific commit. See MODEL-PROVENANCE.md (in gradient-jelle) for the codec commit used during training.

## TODO

- [ ] Pin exact SpacetimeDB version in Cargo.toml
- [ ] Pin exact gradient-codec commit as a dependency
- [ ] Verify SpacetimeDB SDK API surface matches the reducer signatures
- [ ] Record all transitive dependency licenses

## Vector15D pin

Canonical codec: https://github.com/alamort-sun/gradient-codec @ `c8d0ee1` (magnetic_north / magnetic_south).
