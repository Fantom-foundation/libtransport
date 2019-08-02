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
pub trait Transport<Id, Peer, Data, Error>
where
    Data: AsRef<u8> + Serialize,
    Id: PeerId,
{
    // transport configuration type
    type Configuration;

    // Create new Transport instance
    fn new(cfg: Self::Configuration) -> Self;

    // send specified message to the specified peer
    fn send(&mut self, peer_address: String, data: Data) -> Result<()>;

    // broadcast specified message to all peers
    // NB: broadcast effectivelly possible via this call only if underlying
    // implementation allowing it, e.g. broadcasting within IP network.
    // Otherwise create a macro that calls send() above for every member of peer list.
    fn broadcast(&mut self, peers: &dyn PeerList<Id, Error, Item = Peer>, data: Data)
        -> Result<()>;

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
