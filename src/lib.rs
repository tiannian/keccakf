//! An implementation of the `keccak-F[1600,800,400,200]`.
//!
//! All `Keccak-F[]` permutation is fully unrolled; it's nearly as fast
//! as the Keccak team's optimized permutation. Use `macro` to generate
//! functions `Keccak-F[]` with different parameters.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! keccakf = "0.1.2"
//! ```
//!
//! then you can use this code:
//!
//! ```rust
//! use keccakf::{Keccak1600State, Permutation};
//!
//! let mut state = Keccak1600State::default();
//! state.permute();
//! ```
//!
//! Original implemntation in Rust:
//! [tiny-keccak](https://github.com/debris/tiny-keccak)
//!
//! Test vectors:
//! [XKCP](https://github.com/XKCP/XKCP)

#![no_std]

mod prelude;
pub use prelude::*;

mod macros;

mod keccakfs;
pub use keccakfs::*;

mod state;
pub use state::*;
