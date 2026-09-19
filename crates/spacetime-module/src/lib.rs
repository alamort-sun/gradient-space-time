//! gradient-space-time: SpacetimeDB module for durable Vector15D trajectory persistence.
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
pub mod reducers;
