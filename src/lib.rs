//! # Fantom transport library
//!
//! This library defines a trait for management and definition of specific transport protocols (such
//! as TCP/IP or UDP). This repository solely defines the trait [`Transport`] and provides no
//! further functionality. If you are looking a concrete implementation, please check the
//! [`libtransport-tcp` repo](https://github.com/Fantom-foundation/libtransport-tcp).
use crate::errors::Result;
use futures::stream::Stream;
use libcommon_rs::peer::{PeerId, PeerList};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::Unpin;

/// An enum for identifying various Transport types. So far only the TCP variant has been identified
/// and implemented.
pub enum TransportType {
    Unknown,
    TCP,
}

/// Transport configuration trait.
pub trait TransportConfiguration<Data> {
    /// Creates a new transport configuration with the specified network address for incoming
    /// message listener.
    fn new(set_bind_net_addr: String) -> Result<Self>
    where
        Self: Sized;

    /// Sets the bind network address of the incoming message listener.
    fn set_bind_net_addr(&mut self, address: String) -> Result<()>;
}

/// The `Transport` trait defines the functionality for connecting with other devices in the same
/// network. This trait will handle the barebones methods of:
/// * Recieving data from a peer.
/// * Sending data to a peer.
/// * Broadcasting messages to all peers in the peer list.
///
/// The `Transport` trait requires the following parameter types to work properly:
/// * `Id`: The unqiue ID type of the peer in question. An instance of `Id` can be a simple newtype.
/// * `Data`: The data being transmitted between peers - same as the data struct defined above.
/// * `Error`: An error type that can be returned by implementations of the `PeerList` trait.
/// * `Pl`: A struct which implementes the `PeerList` trait (defined in the [`libcommon`
/// repo](https://github.com/Fantom-foundation/libcommon-rs).
///
/// The trait can be instantiated by a struct containing message type and payload data.
///
/// Receive functionality of a `Transport` must be implemented with the [`futures::stream::Stream`]
/// trait.  For an example of how this trait can be implemented, please look at the
/// [`libtransport-tcp` repository](https://github.com/Fantom-foundation/libtransport-tcp).
///
/// For further examples on how you can use the `Transport` trait, see the
/// [`generic_test`](./generic_test.rs) module for a simple implementation.
pub trait Transport<Id, Data, Error, Pl>: Stream<Item = Data> + Drop + Unpin
where
    Id: PeerId,
    Pl: PeerList<Id, Error>,
    Data: Serialize + DeserializeOwned,
{
    /// Transport configuration type.
    type Configuration: TransportConfiguration<Data>;

    /// Creates a new `Transport` instance.
    fn new(cfg: Self::Configuration) -> Self;

    /// Sends the specified message to the specified peer.
    fn send(&mut self, peer_address: String, data: Data) -> Result<()>;

    /// Broadcasts a message of type `Data` to all peers.
    fn broadcast(&mut self, peers: &mut Pl, data: Data) -> Result<()>;
}

pub mod errors;
pub mod generic_test;
