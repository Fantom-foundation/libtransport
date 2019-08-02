use crate::errors::Result;
use libcommon_rs::peer::{PeerId, PeerList};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;

// Transport trait for various implementations of message
// sending/receiving services
//
// peer_address - network address of the peer; e.g. "IP:port".
//
// Data - Transmitting data type;
// it can be a truct containing message type and payload data
pub trait Transport<Id, Peer, Data, Error, Pl>
where
    Data: AsRef<u8> + Serialize,
    Id: PeerId,
    Pl: PeerList<Id, Error, Item = Peer>,
{
    // transport configuration type
    type Configuration;

    // Create new Transport instance
    fn new(cfg: Self::Configuration) -> Self;

    // send specified message to the specified peer
    fn send(&mut self, peer_address: String, data: Data) -> Result<()>;

    // broadcast specified message to all peers
    fn broadcast(&mut self, peers: &Pl, data: Data) -> Result<()>;

    // register a sending-half of std::sync::mpsc::channel which is used to push
    // all received messages to.
    // Several channels can be registered, they will be pushed in
    // the order of registration.
    fn register_channel(&mut self, sender: Sender<Data>) -> Result<()>;
}

pub mod errors;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
