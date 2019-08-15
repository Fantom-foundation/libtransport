use crate::errors::{Error, Error::AtMaxVecCapacity};
use crate::{Transport, TransportConfiguration};
use core::slice::{Iter, IterMut};
use libcommon_rs::peer::{Peer, PeerList};
//use os_pipe::{pipe, PipeReader};
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{thread, time};

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

pub struct TestPeer<Id> {
    pub id: Id,
    pub net_addr: String,
    pub last_data: Option<Data>,
}

impl TestPeer<Id> {
    fn set_last_data(&mut self, data: Data) -> bool {
        self.last_data = Some(data);
        true
    }
}

impl Peer<Id> for TestPeer<Id> {
    fn new(id: Id, addr: String) -> TestPeer<Id> {
        TestPeer {
            id,
            net_addr: addr,
            last_data: None,
        }
    }
    fn get_id(&self) -> Id {
        self.id.clone()
    }
    fn get_net_addr(&self) -> String {
        self.net_addr.clone()
    }
}

pub struct TestPeerList<Id> {
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
    fn get_peers_from_file(&mut self, _json_peer_path: String) -> std::result::Result<(), Error> {
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

pub fn common_test<
    C: TransportConfiguration<Data>,
    T: Transport<Id, Data, Error, TestPeerList<Id>, Configuration = C>,
>(
    net_addrs: Vec<String>,
) {
    let n_peers = net_addrs.len();
    let mut pl: TestPeerList<Id> = TestPeerList::new();
    let mut ch_r: Vec<Receiver<Data>> = Vec::with_capacity(n_peers);
    //let mut pi_r: Vec<PipeReader> = Vec::with_capacity(n_peers);
    let mut trns: Vec<T> = Vec::with_capacity(n_peers);
    for i in 0..n_peers {
        let mut config = C::new(net_addrs[i].clone());
        let (tx, rx) = mpsc::channel::<Data>();
        ch_r.insert(i, rx);
        config.register_channel(tx).unwrap();
        //let (reader, writer) = pipe().unwrap();
        //pi_r.insert(i, reader);
        //config.register_os_pipe(writer).unwrap();
        let mut peer = TestPeer::new(i.into(), net_addrs[i].clone());
        config
            .register_callback(|d: Data| peer.set_last_data(d))
            .unwrap();
        pl.add(peer).unwrap();
        trns.push(T::new(config));
    }
    thread::sleep(time::Duration::from_secs(3));

    // Test broadcast
    println!("Broadcast test");
    let d: Data = Data(55);
    trns[0].broadcast(&mut pl, d.clone()).unwrap();
    for i in 0..n_peers {
        println!("receiving from peer {}", i);
        let t = ch_r[i].recv().unwrap();
        assert_eq!(d, t);
        println!("checking closure delivary on peer {}", i);
        let peer_data = {
            match &pl[i].last_data {
                None => panic!("none on the peer {}", i),
                Some(x) => x,
            }
        };
        assert_eq!(d, *peer_data);
    }

    // Test direct sending
    println!("Unicast test");
    let u: Data = Data(0xaa);
    trns[1].send(pl[0].net_addr.clone(), u.clone()).unwrap();
    let t = ch_r[0].recv().unwrap();
    assert_eq!(u, t);
}
