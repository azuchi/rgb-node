// RGB node providing smart contracts functionality for Bitcoin & Lightning.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use clap::{Parser, ValueHint};
use internet2::addr::ServiceAddr;
use rgb_node::opts::Opts as SharedOpts;
use storm_app::STORM_NODE_APP_ENDPOINT;

/// Command-line arguments
#[derive(Parser)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[clap(author, version, name = "rgbd", about = "RGB node managing service")]
pub struct Opts {
    /// These params can be read also from the configuration file, not just
    /// command-line args or environment variables
    #[clap(flatten)]
    pub shared: SharedOpts,

    /// ZMQ socket for connecting RGB node message bus.
    #[clap(
        long,
        env = "STORM_NODE_APP_ENDPOINT",
        default_value = STORM_NODE_APP_ENDPOINT,
        value_hint = ValueHint::FilePath
    )]
    pub storm_endpoint: ServiceAddr,
}
