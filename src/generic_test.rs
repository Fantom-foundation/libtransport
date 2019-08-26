//! # Fantom transport library generic tests
//!
//! This module contains a set of generic tests that can be used to test the functionality of the
//! [`Transport`] trait. Dummy data type helpers are also provided to be used with any
//! [`Transport`] instance.
//!
//! The `common_test` method, applied to an instance of [`Transport`] allows to test the methods
//! [`Transport::new`], [`Transport::send`], and [`Transport::broadcast`], and the instance of
//! [`futures::stream::Stream`].
use crate::errors::{Error, Error::AtMaxVecCapacity};
use crate::{Transport, TransportConfiguration};
use core::slice::{Iter, IterMut};
use futures::executor::block_on;
use futures::stream::StreamExt;
use libcommon_rs::peer::{Peer, PeerList};
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};
use std::{thread, time};

/// Dummy data struct wrapping a `u32`.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Data(pub u32);
/// Dummy ID wrapping a `u32`.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Id(pub u32);

/// Allows a usize to be used for `Id` struct creation.
impl From<usize> for Id {
    fn from(x: usize) -> Id {
        Id(x as u32)
    }
}

/// Allows a `usize` to be used for `Data` struct creation.
impl From<usize> for Data {
    fn from(x: usize) -> Data {
        Data(x as u32)
    }
}

/// A simple test struct that holds peer information. This includes both an ID and an address.  This
/// implementation is test-specific.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TestPeer<Id> {
    pub id: Id,
    pub net_addr: String,
}

/// Implement the Peer trait for TestPeer.
impl Peer<Id> for TestPeer<Id> {
    /// Create a new peer
    fn new(id: Id, addr: String) -> TestPeer<Id> {
        TestPeer { id, net_addr: addr }
    }

    /// Gets the `Id`.
    fn get_id(&self) -> Id {
        self.id.clone()
    }

    /// Gets the network address.
    fn get_net_addr(&self) -> String {
        self.net_addr.clone()
    }
}

/// A [`PeerList`] instance for testing purposes.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TestPeerList<Id> {
    peers: Vec<TestPeer<Id>>,
}

/// Allows the use of indexing to access data within a peer list.
impl<Id> Index<usize> for TestPeerList<Id> {
    type Output = TestPeer<Id>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.peers[index]
    }
}

/// Allows the use of indexing to access mutable data within a peer list.
impl<Id> IndexMut<usize> for TestPeerList<Id> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.peers[index]
    }
}

/// An implementation of `PeerList` for our `TestPeerList` struct.
impl PeerList<Id, Error> for TestPeerList<Id> {
    type P = TestPeer<Id>;

    /// Constructor.
    fn new() -> Self {
        TestPeerList {
            peers: Vec::with_capacity(1),
        }
    }

    /// Adds a new peer to the peer list.
    fn add(&mut self, p: TestPeer<Id>) -> std::result::Result<(), Error> {
        // Check if we're at max capacity
        if self.peers.len() == std::usize::MAX {
            return Err(AtMaxVecCapacity);
        }
        // Push value into vec
        self.peers.push(p);

        Ok(())
    }

    /// Loads peers in from a json file. Not relevant to this test.
    fn get_peers_from_file(&mut self, _json_peer_path: String) -> std::result::Result<(), Error> {
        // Stub not used in tests to satisfy PeerList trait
        Ok(())
    }

    /// Allows iteration over the peer list.
    fn iter(&self) -> Iter<'_, Self::P> {
        self.peers.iter()
    }

    /// Allows a mutable iteration over the peer list.
    fn iter_mut(&mut self) -> IterMut<'_, Self::P> {
        self.peers.iter_mut()
    }
}

/**
    The function used to actually test a `Transport`. It takes in a `TransportConfiguration` and a
    `Transport` trait implementor.

    This method simply takes in a list of peers, instantiates them, and tests whether they can
    send/receive data to/from one another.
**/
pub fn common_test<
    C: TransportConfiguration<Data>,
    T: Transport<Id, Data, Error, TestPeerList<Id>, Configuration = C>,
>(
    net_addrs: Vec<String>,
) {
    let n_peers = net_addrs.len();
    // Create a new TestPeerList
    let mut pl: TestPeerList<Id> = TestPeerList::new();

    let mut trns: Vec<T> = Vec::with_capacity(n_peers);
    // Iterate over all peers, create a config for each one and create a Transport to handle
    // messaging.
    for (i, addr) in net_addrs.iter().enumerate().take(n_peers) {
        let config = C::new(addr.clone()).expect("cannot create config");
        pl.add(TestPeer::new(i.into(), addr.clone()))
            .expect("cannot add peer");
        trns.push(T::new(config));
    }

    // Wait three seconds.
    thread::sleep(time::Duration::from_secs(3));

    // Test broadcast
    println!("Broadcast test");

    // Create Data to send.
    let d: Data = Data(55);
    // Broadcast data.
    trns[0].broadcast(&mut pl, d.clone()).unwrap();
    for (i, trn) in trns.iter_mut().enumerate().take(n_peers) {
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
    trns[1].send(pl[0].net_addr.clone(), u.clone()).unwrap();
    // Asynchronously check whether the receiver got the sent message.
    block_on(async {
        let n = trns[0].next().await;
        match n {
            Some(t) => assert_eq!(u, t),
            None => panic!("unexpected None"),
        }
    });
}
