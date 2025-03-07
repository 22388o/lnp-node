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

mod ctl;
mod reports;

pub use ctl::*;
use lnp::p2p;
use lnp_rpc::RpcMsg;
use microservices::esb::BusId;
use microservices::rpc;
pub use reports::{IntoSuccessOrFalure, ToProgressOrFalure};

use crate::rpc::ServiceId;

/// Service buses used for inter-daemon communication
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Display)]
pub enum ServiceBus {
    /// RPC interface, from client to node
    #[display("RPC")]
    Rpc,

    /// LN P2P message bus
    #[display("MSG")]
    Msg,

    /// Control service bus
    #[display("CTL")]
    Ctl,

    /// Bridge between listening and sending parts of the peer connection
    #[display("BRIDGE")]
    Bridge,
}

impl BusId for ServiceBus {
    type Address = ServiceId;
}

/// Service bus messages wrapping all other message types
#[derive(Clone, Debug, Display, From, Api)]
#[api(encoding = "strict")]
#[non_exhaustive]
pub enum BusMsg {
    /// Wrapper for Bolt P2P messages to be transmitted over control bus
    #[cfg(feature = "bolt")]
    #[api(type = 1)]
    #[display(inner)]
    #[from]
    Bolt(p2p::bolt::Messages),

    /// Wrapper for Bifrost P2P messages to be transmitted over control bus
    #[cfg(feature = "bifrost")]
    #[api(type = 3)]
    #[display(inner)]
    #[from]
    Bifrost(p2p::bifrost::Messages),

    /// Wrapper for inner type of control messages to be transmitted over control bus
    #[api(type = 2)]
    #[display(inner)]
    #[from]
    Ctl(CtlMsg),

    /// Wrapper for RPC messages to be transmitted over control bus
    #[api(type = 4)]
    #[display(inner)]
    #[from]
    Rpc(RpcMsg),
}

impl rpc::Request for BusMsg {}
