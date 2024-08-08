// Copyright 2024 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.
#![allow(clippy::mutable_key_type)] // for Bytes in NetworkAddress

use crate::{storage::RecordType, NetworkAddress};
use serde::{Deserialize, Serialize};
// TODO: remove this dependency and define these types herein.
pub use sn_transfers::{Hash, PaymentQuote};

/// Data and CashNote cmds - recording spends or creating, updating, and removing data.
///
/// See the [`protocol`] module documentation for more details of the types supported by the Safe
/// Network, and their semantics.
///
/// [`protocol`]: crate
#[allow(clippy::large_enum_variant)]
#[derive(Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Cmd {
    /// Write operation to notify nodes a list of PaymentQuote collected.
    QuoteVerification {
        target: NetworkAddress,
        quotes: Vec<(NetworkAddress, PaymentQuote)>,
    },
    /// Notify the peer it is now being considered as BAD due to the included behaviour
    PeerConsideredAsBad {
        detected_by: NetworkAddress,
        bad_peer: NetworkAddress,
        bad_behaviour: String,
    },
}

impl std::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cmd::QuoteVerification { target, quotes } => f
                .debug_struct("Cmd::QuoteVerification")
                .field("target", target)
                .field("quotes_len", &quotes.len())
                .finish(),
            Cmd::PeerConsideredAsBad {
                detected_by,
                bad_peer,
                bad_behaviour,
            } => f
                .debug_struct("Cmd::PeerConsideredAsBad")
                .field("detected_by", detected_by)
                .field("bad_peer", bad_peer)
                .field("bad_behaviour", bad_behaviour)
                .finish(),
        }
    }
}

impl Cmd {
    /// Used to send a cmd to the close group of the address.
    pub fn dst(&self) -> NetworkAddress {
        match self {
            Cmd::QuoteVerification { target, .. } => target.clone(),
            Cmd::PeerConsideredAsBad { bad_peer, .. } => bad_peer.clone(),
        }
    }
}

impl std::fmt::Display for Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cmd::QuoteVerification { target, quotes } => {
                write!(
                    f,
                    "Cmd::QuoteVerification(sent to {target:?} has {} quotes)",
                    quotes.len()
                )
            }
            Cmd::PeerConsideredAsBad {
                detected_by,
                bad_peer,
                bad_behaviour,
            } => {
                write!(
                    f,
                    "Cmd::PeerConsideredAsBad({detected_by:?} consider peer {bad_peer:?} as bad, due to {bad_behaviour:?})")
            }
        }
    }
}
