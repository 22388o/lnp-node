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
#![deny(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_mut,
    unused_imports,
    dead_code,
    missing_docs
)]

//! Main executable for channeld: lightning node channel operations microservice

#[macro_use]
extern crate log;

use clap::Parser;
use lnp::p2p::bolt::ActiveChannelId;
use lnp_node::channeld::{self, Opts};
use lnp_node::Config;

fn main() {
    println!("channeld: lightning channel microservice");

    let mut opts = Opts::parse();
    trace!("Command-line arguments: {:?}", &opts);
    opts.process();
    trace!("Processed arguments: {:?}", &opts);

    let config: Config = opts.clone().into();
    trace!("Daemon configuration: {:?}", &config);
    debug!("MSG RPC socket {}", &config.msg_endpoint);
    debug!("CTL RPC socket {}", &config.ctl_endpoint);

    /*
    use self::internal::ResultExt;
    let (config_from_file, _) =
        internal::Config::custom_args_and_optional_files(std::iter::empty::<
            &str,
        >())
        .unwrap_or_exit();
     */

    debug!("Starting runtime ...");
    let channel_id = if opts.reestablish {
        info!("Will try to re-establish channel {}", opts.channel_id);
        ActiveChannelId::Static(opts.channel_id)
    } else {
        ActiveChannelId::Temporary(opts.channel_id.into())
    };
    channeld::run(config, channel_id).expect("Error running channeld runtime");

    unreachable!()
}
