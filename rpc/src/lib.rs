// LNP Node: node running lightning network protocol and generalized lightning
// channels.
// Written in 2020-2024 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#![recursion_limit = "256"]
// Coding conventions
#![allow(clippy::large_enum_variant)]

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_encoding;
#[macro_use]
extern crate internet2;
#[macro_use]
extern crate log;

#[cfg(feature = "serde")]
extern crate serde_crate as serde;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_with;

#[cfg(not(any(feature = "bolt", feature = "bifrost")))]
compile_error!("either 'bolt' or 'bifrost' feature must be used");

mod client;
mod error;
mod listen_addr;
mod messages;
mod service_id;

pub use client::Client;
pub use error::{Error, FailureCode};
pub use listen_addr::ListenAddr;
pub use messages::*;
pub use service_id::ServiceId;

pub const LNP_NODE_RPC_ENDPOINT: &str = "0.0.0.0:62962";
