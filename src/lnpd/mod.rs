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

pub mod automata;
pub(self) mod daemons;
pub mod funding;
#[cfg(feature = "server")]
mod opts;
mod runtime;

pub use daemons::{read_node_key_file, Daemon};
#[cfg(feature = "server")]
pub use opts::{Command, Opts};
pub use runtime::run;
