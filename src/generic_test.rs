/// # Fantom Libtransport/generic_tests
///
/// This file contains a set of generic tests which can be used to test the functionality of the
/// Transport trait. This file contains a bunch of dummy data which can be quickly used for any
/// Transport struct.
///
/// The common_test method allows us to quickly test the new(), send(), and broadcast() methods and
/// (hopefully) verifies that they work.
use crate::errors::{Error, Error::AtMaxVecCapacity, Result};
use crate::Transport;
use core::fmt::Display;
use core::slice::{Iter, IterMut};
use futures::executor::block_on;
use futures::stream::StreamExt;
use libcommon_rs::peer::{Peer, PeerList};
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};
use std::{thread, time};

// Dummy data struct. Simply uses a u32 for instantiation.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Data(pub u32);
// Dummy ID, also uses a u32 for instantiation.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Id(pub u32);

// Allows a usize to be used for Id struct creation.
impl From<usize> for Id {
    fn from(x: usize) -> Id {
        Id(x as u32)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Id {
    fn default() -> Id {
        Id { 0: 0 }
    }
}

// Allows a usize to be used for Data struct creation.
impl From<usize> for Data {
    fn from(x: usize) -> Data {
        Data(x as u32)
    }
}

// A simple test struct for holding peer information. This includes both an id and an address.
// NOTE: This specific implementation is only for testing purposes.
pub struct TestPeer<Id> {
    pub id: Id,
    pub base_addr: String,
    pub net_addr: Vec<String>,
}

// Implement the Peer trait for TestPeer.
impl Peer<Id, Error> for TestPeer<Id> {
    // Create a new peer
    fn new(id: Id, addr: String) -> TestPeer<Id> {
        TestPeer {
            id,
            base_addr: addr,
            net_addr: Vec::with_capacity(1),
        }
    }
    // Getter for the Id
    fn get_id(&self) -> Id {
        self.id.clone()
    }
    // Getter for the base network address
    fn get_base_addr(&self) -> String {
        self.base_addr.clone()
    }
    fn get_net_addr(&self, n: usize) -> String {
        self.net_addr[n].clone()
    }
    fn set_net_addr(&mut self, n: usize, addr: String) -> std::result::Result<(), Error> {
        if self.net_addr.len() == std::usize::MAX {
            return Err(AtMaxVecCapacity);
        }
        // FIXME: insert panics in n > net_addr.len()
        self.net_addr.insert(n, addr);
        Ok(())
    }
}

// Creation of our own PeerList type (used for testing purposes)
pub struct TestPeerList<Id> {
    peers: Vec<TestPeer<Id>>,
}

// Allows the use of indexing to access data within the peer list.
impl<Id> Index<usize> for TestPeerList<Id> {
    type Output = TestPeer<Id>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.peers[index]
    }
}

// Allows the use of indexing to access mutable data within the peer list.
impl<Id> IndexMut<usize> for TestPeerList<Id> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.peers[index]
    }
}

// Implementation of PeerList for our TestPeerList struct.
impl PeerList<Id, Error> for TestPeerList<Id> {
    type P = TestPeer<Id>;

    // Constructor
    fn new() -> Self {
        TestPeerList {
            peers: Vec::with_capacity(1),
        }
    }
    // Function which allows adding new peers to our peer list.
    fn add(&mut self, p: TestPeer<Id>) -> std::result::Result<(), Error> {
        // Check if we're at max capacity
        if self.peers.len() == std::usize::MAX {
            return Err(AtMaxVecCapacity);
        }
        // Push value into vec
        self.peers.push(p);

        Ok(())
    }
    // Loads peers in from a json file. Not relevant to this test.
    fn get_peers_from_file(&mut self, _json_peer_path: String) -> std::result::Result<(), Error> {
        // Stub not used in tests to satisfy PeerList trait
        Ok(())
    }
    // Allows iteration over the peer list.
    fn iter(&self) -> Iter<'_, Self::P> {
        self.peers.iter()
    }
    // Allows a mutable iteration over the peer list.
    fn iter_mut(&mut self) -> IterMut<'_, Self::P> {
        self.peers.iter_mut()
    }
}

/*
    The function used to actually test the Transport. It takes in a Transport Configuration and a
    Transport trait implementor.

    THis method simply takes in a list of peers, instantiates them, and tests whether they can
    send/receive data to one another.
*/
pub fn common_test<
    //    C: TransportConfiguration<Data>,
    T: Transport<Id, Data, Error, TestPeerList<Id>>,
>(
    net_addrs: Vec<String>,
) -> Result<()> {
    let n_peers = net_addrs.len();
    // Create a new TestPeerList
    let mut pl: TestPeerList<Id> = TestPeerList::new();

    let mut trns: Vec<T> = Vec::with_capacity(n_peers);
    // Iterate over all peers, create a config for each one and create a Transport to handle
    // messaging.
    for (i, net_addr) in net_addrs.iter().enumerate() {
        pl.add(TestPeer::new(i.into(), net_addr.clone()))?;
        trns.push(T::new(net_addr.clone())?);
    }

    // Wait three seconds.
    thread::sleep(time::Duration::from_secs(3));

    // Test broadcast
    println!("Broadcast test");

    // Create Data to send.
    let d: Data = Data(55);
    // Broadcast data.
    trns[0].broadcast(&mut pl, d.clone())?;
    for (i, trn) in trns.iter_mut().enumerate() {
        // Asynchronously check if all peers have received the message.
        block_on(async {
            println!("receiving from peer {}", i);
            let n = trn.next().await;
            match n {
                Some(t) => assert_eq!(d, t),
                None => panic!("unexpected None"),
            }
        });
    }

    // Test direct sending
    println!("Unicast test");
    let u: Data = Data(0xaa);
    // Send directed data between two peers.
    trns[1].send(pl[0].base_addr.clone(), u.clone())?;
    // Asynchronously check whether the receiver got the sent message.
    block_on(async {
        let n = trns[0].next().await;
        match n {
            Some(t) => assert_eq!(u, t),
            None => panic!("unexpected None"),
        }
    });

    Ok(())
}
