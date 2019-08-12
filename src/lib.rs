//#![feature(generic_associated_types)]
use crate::errors::Result;
use libcommon_rs::peer::{PeerId, PeerList};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::mpsc::Sender;

// Transport configurtatiion trait
pub trait TransportConfiguration<Data> {
    // creates new transport configuration with specified network
    // address for incoming messages listener
    fn new(set_bind_net_addr: String) -> Self;
    // register a sending-half of std::sync::mpsc::channel which is used to push
    // all received messages to.
    // Several channels can be registered, they will be pushed in
    // the order of registration.
    fn register_channel(&mut self, sender: Sender<Data>) -> Result<()>;
    // set bind network address for incoming messages listener
    fn set_bind_net_addr(&mut self, address: String) -> Result<()>;
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
    use super::{Transport, TransportConfiguration};
    use core::slice::{Iter, IterMut};
    use libcommon_rs::peer::{Peer, PeerId, PeerList};
    use serde::{Deserialize, Serialize};
    use std::ops::{Index, IndexMut};
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};

    #[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
    pub struct Data(pub u32);
    #[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
    pub struct Id(pub u32);

    impl From<usize> for Id {
        fn from(x: usize) -> Id {
            Id(x as u32)
        }
    }

    impl From<usize> for Data {
        fn from(x: usize) -> Data {
            Data(x as u32)
        }
    }

    struct TestPeer<Id> {
        pub id: Id,
        pub net_addr: String,
    }

    impl Peer<Id> for TestPeer<Id> {
        fn new(id: Id, addr: String) -> TestPeer<Id> {
            TestPeer {
                id: id,
                net_addr: addr,
            }
        }
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

    impl<Id> Index<usize> for TestPeerList<Id> {
        type Output = TestPeer<Id>;
        fn index(&self, index: usize) -> &Self::Output {
            &self.peers[index]
        }
    }

    impl<Id> IndexMut<usize> for TestPeerList<Id> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.peers[index]
        }
    }

    impl PeerList<Id, Error> for TestPeerList<Id> {
        type P = TestPeer<Id>;
        fn new() -> Self {
            TestPeerList {
                peers: Vec::with_capacity(1),
            }
        }
        fn add(&mut self, p: TestPeer<Id>) -> std::result::Result<(), Error> {
            if self.peers.len() == std::usize::MAX {
                return Err(AtMaxVecCapacity);
            }
            self.peers.push(p);
            Ok(())
        }
        fn get_peers_from_file(
            &mut self,
            _json_peer_path: String,
        ) -> std::result::Result<(), Error> {
            // Stub not used in tests to satisfy PeerList trait
            Ok(())
        }
        fn iter(&self) -> Iter<'_, Self::P> {
            self.peers.iter()
        }
        fn iter_mut(&mut self) -> IterMut<'_, Self::P> {
            self.peers.iter_mut()
        }
    }

    fn common_test<
        C: TransportConfiguration<Data> + Clone,
        T: Transport<Id, Data, Error, TestPeerList<Id>, Configuration = C>,
    >(
        net_addrs: Vec<String>,
    ) {
        let n_peers = net_addrs.len();
        let mut cfgs: Vec<C> = Vec::with_capacity(n_peers);
        let mut pl: TestPeerList<Id> = TestPeerList::new();
        let mut ch_r: Vec<Receiver<Data>> = Vec::with_capacity(n_peers);
        for i in 0..n_peers {
            let mut config = C::new(net_addrs[i].clone());
            let (tx, rx) = mpsc::channel::<Data>();
            ch_r[i] = rx;
            config.register_channel(tx).unwrap();
            cfgs.push(config);
            pl.add(TestPeer::new(i.into(), net_addrs[i].clone()))
                .unwrap();
        }
        let mut trns: Vec<T> = Vec::with_capacity(n_peers);
        for i in 0..n_peers {
            trns.push(T::new(cfgs[i].clone()));
        }

        // Test broadcast
        let d: Data = Data(55);
        trns[0].broadcast(&mut pl, d.clone()).unwrap();
        for i in 0..n_peers {
            let t = ch_r[i].recv().unwrap();
            assert_eq!(d, t);
        }

        // Test direct sending
        let u: Data = Data(0xaa);
        trns[1].send(pl[0].net_addr.clone(), u.clone()).unwrap();
        let t = ch_r[0].recv().unwrap();
        assert_eq!(u, t);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
