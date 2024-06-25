//! This demo project shows different ways of how to implement the typestate pattern.
//!
//! Each module shows a different implementation.
//! The basic state machine that all versions implement is this door example:
//!
#![doc = include_str!("../assets/door.svg")]
//!
//! The submodules are enumerated so you can follow along.
//! Notice, that with each example the requirements of the state machine slightly change and the
//! implementations are changing to respond to these changed requirements.
//!
//! # Disclaimer
//!
//! The demo in this repository is slightly changed and way more documentation is added compared to
//! the code presented in the talk.
//! This was done to make it easier to follow along without watching the talk in parallel.
//!

pub mod v00_runtime;
pub mod v01_state_as_types;
pub mod v02_state_markers;
pub mod v03_generic_state;
