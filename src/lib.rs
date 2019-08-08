use crate::errors::Result;
use libcommon_rs::peer::{PeerId, PeerList};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::mpsc::Sender;

// Transport configurtatiion trait
pub trait TransportConfiguration<Data> {
    // register a sending-half of std::sync::mpsc::channel which is used to push
    // all received messages to.
    // Several channels can be registered, they will be pushed in
    // the order of registration.
    fn register_channel(&mut self, sender: Sender<Data>) -> Result<()>;
}

// Transport trait for various implementations of message
// sending/receiving services
//
// peer_address - network address of the peer; e.g. "IP:port".
//
// Id - peer ID type
// Data - Transmitting data type;
// Error - error type returned by methods of Pl: PeerList
// it can be a truct containing message type and payload data
pub trait Transport<Id, Data, Error, Pl>: Drop
where
    Data: Serialize + DeserializeOwned,
    Id: PeerId,
    Pl: PeerList<Id, Error>,
{
    // transport configuration type
    type Configuration: TransportConfiguration<Data>;

    // Create new Transport instance
    fn new(cfg: Self::Configuration) -> Self;

    // send specified message to the specified peer
    fn send(&mut self, peer_address: String, data: Data) -> Result<()>;

    // broadcast specified message to all peers
    fn broadcast(&mut self, peers: &mut Pl, data: Data) -> Result<()>;

    // register a sending-half of std::sync::mpsc::channel which is used to push
    // all received messages to.
    // Several channels can be registered, they will be pushed in
    // the order of registration.
    fn register_channel(&mut self, sender: Sender<Data>) -> Result<()>;
}

pub mod errors;

#[cfg(test)]
mod tests {
    use super::errors::{Error, Error::AtMaxVecCapacity, Result};
    use super::Transport;
    use libcommon_rs::peer::{Peer, PeerId, PeerList};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
    pub struct Data(pub u32);
    #[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
    pub struct Id(pub u32);

    struct TestPeer<Id> {
        pub id: Id,
        pub net_addr: String,
    }

    impl Peer<Id> for TestPeer<Id> {
        fn get_id(&self) -> Id {
            self.id.clone()
        }
        fn get_net_addr(&self) -> String {
            self.net_addr.clone()
        }
    }

    struct TestPeerList<Id> {
        peers: Vec<TestPeer<Id>>,
    }

    struct IterTestPeerList<'a, Id: 'a> {
        inner: &'a TestPeerList<Id>,
        pos: usize,
    }

    impl<'a, Id> Iterator for IterTestPeerList<'a, Id> {
        type Item = &'a TestPeer<Id>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.pos >= self.inner.peers.len() {
                None
            } else {
                self.pos += 1;
                self.inner.peers.get(self.pos - 1)
            }
        }
    }

    //impl<Id> Iterator for TestPeerList<Id> {
    //    fn iter<'a>(&'a self) -> IterTestPeerList<'a, TestPeerList<Id>> {
    //        IterTestPeerList::<Id> {
    //            inner: self,
    //            pos: 0,
    //        }
    //    }
    //}

    impl<'a, Error> PeerList<Id, Error> for TestPeerList<Id> {
        fn add(&mut self, p: TestPeer<Id>) -> std::result::Result<(), Error> {
            if self.peers.len() == std::usize::MAX {
                return Err(AtMaxVecCapacity);
            }
            self.peers.push(p);
            Ok(())
        }
        fn get_peers_from_file(json_peer_path: String) -> std::result::Result<(), Error> {
            // Stub not used in tests to satisfy PeerList trait
            Ok(())
        }
        type IterType = IterTestPeerList<'a, TestPeerList<Id>>;
        fn iter(&'a self) -> IterTestPeerList<'a, TestPeerList<Id>> {
            IterTestPeerList::<Id> {
                inner: self,
                pos: 0,
            }
        }
    }

    fn CommonTest<T: Transport<Id, Data, Error, TestPeerList<Id>>>() {}

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
