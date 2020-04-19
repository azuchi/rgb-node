// Kaleidoscope: RGB command-line wallet utility
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//     Alekos Filini <alekos.filini@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.


use std::io;
use tokio::task::JoinError;

use lnpbp::csv::serialize;


#[derive(Debug, Display, From)]
#[display_from(Debug)]
pub enum Error {
    TorNotYetSupported,

    #[derive_from]
    IoError(io::Error),

    #[derive_from]
    ArgParseError(String),

    SubscriptionError(zmq::Error),

    PublishingError(zmq::Error),

    #[derive_from]
    MultithreadError(JoinError),

    #[derive_from]
    SerializeError(serialize::Error),
}

impl std::error::Error for Error { }

impl From<Error> for String {
    fn from(err: Error) -> Self { format!("{}", err) }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::ArgParseError(err.to_string())
    }
}