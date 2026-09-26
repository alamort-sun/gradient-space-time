# Dependencies — gradient-space-time

## Rust Dependencies

| Crate | Version | License | Purpose |
|-------|---------|---------|---------|
| spacetimedb | Cargo.toml req `"1.0"` (semver); Cargo.lock resolves **1.12.0** (do not invent a tighter Cargo.toml pin until spacetime wasm host/CLI is verified) | Apache-2.0 (crates.io crate) | Transactional persistence and synchronization |
| serde | 1.0 | MIT OR Apache-2.0 | Serialization/deserialization |
| serde_json | 1.0 | MIT OR Apache-2.0 | JSON serialization |
| thiserror | 1.0 | MIT OR Apache-2.0 | Error derive macros |
| tracing | 0.1 | MIT | Structured logging |
| vecGradient (in-ecosystem) | path = `../gradient-codec/vecGradient` (documented codec pin **`359d00c`**; path tracks local tree) | PolyForm Noncommercial | Authority for `Vector15D`, `DomainWall`, `GaugeCoupling`, `validate()`/`try_new_15()` |

## External Dependencies

| Dependency | Version | License | Source |
|------------|---------|---------|--------|
| SpacetimeDB (product / host) | TODO: pin host/CLI independently of the Rust crate req | AGPL-3.0 (product terms — verify against upstream; not relabeled) | https://spacetimedb.com |
| gradient-codec / vecGradient | **`359d00c`** (Seal plane split…; local path-dep expected to match `origin/main` of `alamort-sun/gradient-codec`) | PolyForm Noncommercial | https://github.com/alamort-sun/gradient-codec |

## Path-dep vs documented SHA

`[workspace.dependencies] vecGradient = { path = "../gradient-codec/vecGradient" }` (and `crates/spacetime-module` via `workspace = true`) tracks the **local checkout**, not a git/crates.io pin. The SHA `359d00c` is the honesty pin for which remote commit that local tree should match after the plane-split seal. Confirm with `git -C ../gradient-codec rev-parse --short HEAD` before treating a newer local tree as authoritative.

## Rules

1. **Prefer an exact SpacetimeDB pin when safe.** Today Cargo.toml keeps `spacetimedb = "1.0"` so the wasm toolchain is not broken by an unverified exact bump. Document the lock resolve (1.12.0) here; tighten Cargo.toml only after host/CLI verification.
2. **Do not fork or modify SpacetimeDB internals.** Use SpacetimeDB Rust modules and generated Rust client bindings as-is.
3. **Never relabel third-party dependencies as PolyForm or CC0.** Record their original terms, provenance, version, and source.
4. **SpacetimeDB license note.** SpacetimeDB product/host terms are separate from this repository's PolyForm Noncommercial license (applies only to code written here) and from the crates.io `spacetimedb` crate's declared Apache-2.0.
5. **gradient-codec dependency.** Path-depend on crate `vecGradient` for Vector15D validation (`Vector15D`, alias `Vector13D`). Document the codec commit SHA here whenever `origin/main` advances. See MODEL-PROVENANCE.md (in gradient-jelle) for the codec commit used during training.

## TODO

- [ ] Tighten `spacetimedb` in Cargo.toml to an exact version once spacetime CLI / wasm host build is verified locally (lock currently: 1.12.0 under req `"1.0"`)
- [✓] gradient-codec / vecGradient documented pin **`359d00c`** via path dependency in workspace + `crates/spacetime-module/Cargo.toml`. DomainWall/GaugeCoupling re-exported from codec — no local copy remains.
- [ ] Verify SpacetimeDB SDK API surface matches the reducer signatures
- [ ] Record all transitive dependency licenses (unchecked — do not invent an audit)
