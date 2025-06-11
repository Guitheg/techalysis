#![warn(missing_docs)]
//! This crate provides a collection of technical indicators and utilities for financial
//! analysis.

/// This module contains the error types used throughout the library.
pub mod errors;

/// This module contains the implementations of the technical indicators.
/// Each indicator is implemented as a separate module. Each module contains the
/// implementation of the indicator, a state struct, and a result struct.
/// The result struct contains the calculated values and the state of the indicator,
/// which can be used to calculate the next values incrementally.
pub mod indicators;

/// This module contains the types used throughout the library.
pub mod types;

/// This module contains the traits used throughout the library.
pub mod traits;
