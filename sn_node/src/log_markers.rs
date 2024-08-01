// Copyright 2024 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use crate::Error;
use libp2p::{kad::RecordKey, PeerId};
use sn_protocol::PrettyPrintRecordKey;
use strum::Display;

/// Public Markers for generating log output,
/// These generate appropriate log level output and consistent strings.
/// Changing these log markers is a breaking change.
#[derive(Debug, Clone, Display, Copy)]
pub enum Marker<'a> {
    /// The node has started
    NodeConnectedToNetwork,

    /// Peer was added to the routing table
    PeerAddedToRoutingTable(&'a PeerId),

    /// Peer was removed from the routing table
    PeerRemovedFromRoutingTable(&'a PeerId),

    /// The number of peers in the routing table
    PeersInRoutingTable(usize),

    /// Interval based replication
    IntervalReplicationTriggered,

    /// Keys of Records we are fetching to replicate locally
    FetchingKeysForReplication {
        /// fetching_keys_len: number of keys we are fetching from network
        fetching_keys_len: usize,
    },

    /// Valid non-existing Chunk record PUT from the network received and stored
    ValidChunkRecordPutFromNetwork(&'a PrettyPrintRecordKey<'a>),
    /// Valid non-existing Register record PUT from the network received and stored
    ValidRegisterRecordPutFromNetwork(&'a PrettyPrintRecordKey<'a>),
    /// Valid non-existing Spend record PUT from the network received and stored
    ValidSpendRecordPutFromNetwork(&'a PrettyPrintRecordKey<'a>),

    /// Valid paid to us and royalty paid chunk stored
    ValidPaidChunkPutFromClient(&'a PrettyPrintRecordKey<'a>),
    /// Valid paid to us and royalty paid register stored
    ValidPaidRegisterPutFromClient(&'a PrettyPrintRecordKey<'a>),
    /// Valid spend stored
    ValidSpendPutFromClient(&'a PrettyPrintRecordKey<'a>),

    /// The peer has been considered as bad
    PeerConsideredAsBad(&'a PeerId),

    /// Record rejected
    RecordRejected(&'a PrettyPrintRecordKey<'a>, &'a Error),

    /// Interval based bad_nodes check
    IntervalBadNodesCheckTriggered,
}

impl<'a> Marker<'a> {
    /// Returns the string representation of the LogMarker.
    pub fn log(&self) {
        // Down the line, if some logs are noisier than others, we can
        // match the type and log a different level.
        info!("{self:?}");
    }

    /// Helper to log the FetchingKeysForReplication variant
    pub fn fetching_keys_for_replication(keys: &[(PeerId, RecordKey)]) -> Self {
        Marker::FetchingKeysForReplication {
            fetching_keys_len: keys.len(),
        }
    }
}
