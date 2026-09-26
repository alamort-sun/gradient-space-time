//! gradient-space-time: SpacetimeDB module for plane-safe lease / grant /
//! receipt / manifest persistence (Saraswati A2).
//!
//! Trajectory biography tables and the `trajectory_id` master key are gone.
//! Capsule content is out of scope here (see SCHEMA.md TODOs).
//!
//! Reducers are the transaction boundary and the ONLY mechanism for table mutation.
//! Keep reducers deterministic, bounded, side-effect-free.
//!
//! NEVER inside reducers:
//! - LLM API calls
//! - Network I/O
//! - Hugging Face downloads
//! - GPU jobs
//! - JEPA training
//! - Long-running inference
//! - Filesystem work
//! - Unbounded historical scans
//! - Non-replayable hidden side effects

pub mod types;
pub mod tables;
pub mod validation;
pub mod reducers;
