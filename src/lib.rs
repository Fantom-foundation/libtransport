/// # Fantom libtransport
///
/// This library defines a trait for the management and definition of specific transport prototcols
/// (such as TCP/IP or UDP). This repository solely defines the trait and provides no further
/// functionality. If you want a specific implementation, please check the transport-tcp repo for a
/// complete implementation: https://github.com/Fantom-foundation/libtransport-tcp.
///
/// Currently only one trait is defined: Transport.
///
/// # Transport
///
/// The Transport trait defines the functionality for connecting with other devices in the same
/// network. This trait will handle the barebones methods of:
///
/// a) Recieving data from a peer
/// b) Sending data to a peer
/// c) Broadcasting messages to all peers in the peerslist
///
/// The Transport trait requires the following parameter types to work properly:
///
/// Id: The unqiue ID type of the peer in question. This can be as simple as:
/// ```
/// // Another arbitrary type for holding ID data
/// pub struct Id(pub u32);
/// ```
/// <b>Data:</b> The data being transmitted between peers - same as the data struct defined above.
/// <b>Error:</b> An error type that can be returned by implementations of the 'PeerList' trait
/// <b>Pl:</b> A struct which implementes the PeerList trait (defined in the libcommon repo:
/// https://github.com/Fantom-foundation/libcommon-rs )
///
/// Finally, the Transport trait implements three methods: 'new', 'send', and 'broadcast'. The trait
/// also requires an implementation of the 'Stream' trait from the async/.await framework (only
/// available on nightly).
///
/// For further examples on how you can use the Transport trait, please look at the 'generic_test.rs'
/// file for a simple implementation.
///
use crate::errors::Result;
use futures::stream::Stream;
use libcommon_rs::peer::{PeerId, PeerList};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::Unpin;

/// An enum for identifying various Transport types. So far only the TCP variant has been identified
/// and implemented.

#[derive(Clone)]
pub enum TransportType {
    Unknown,
    TCP,
}

/// Transport trait allows us to create multiple message sending/receiving services which share
/// similar functionality.
///
/// The Transport trait requires 4 parameter types to work:
/// Id: The peer's ID type.
/// Data: The data being transmitted.
/// Error: An error returned by the PeerList trait
/// Pl: A list of Peers (PeerList trait struct)
///
/// NOTE: Transport must implement Stream trait from async/.await framework.
///
/// For an example of how this trait can be implemented, please look at the libtransport-tcp
/// repository: https://github.com/Fantom-foundation/libtransport-tcp

pub trait Transport<Id, Data, Error, Pl>: Stream<Item = Data> + Drop + Unpin
where
    Id: PeerId,
    Pl: PeerList<Id, Error>,
    Data: Serialize + DeserializeOwned,
{
    /// Creates a new Transport type using a preset configuration type.
    fn new(set_bind_net_addr: String) -> Result<Self>
    where
        Self: Sized;

    /// Sends a message of type 'Data' to the specified peer (as specified by an address)
    fn send(&mut self, peer_address: String, data: Data) -> Result<()>;

    /// Broadcasts a message of type 'Data' to all peers on the network. Requires a struct which
    /// implements PeerList.
    fn broadcast(&mut self, peers: &mut Pl, data: Data) -> Result<()>;
}

/// Transport sender trait allows us to create multiple `Data` sending only services.
/// `TransportSender` trait requires the same 4 parameter types as `Transport` trait above.
pub trait TransportSender<Id, Data, Error, Pl>
where
    Id: PeerId,
    Pl: PeerList<Id, Error>,
    Data: Serialize,
{
    /// Creates a new Transport type using a preset configuration type.
    fn new() -> Result<Self>
    where
        Self: Sized;

    /// Sends a message of type 'Data' to the specified peer (as specified by an address)
    fn send(&mut self, peer_address: String, data: Data) -> Result<()>;

    /// Broadcasts a message of type 'Data' to all peers on the network. Requires a struct which
    /// implements PeerList.
    fn broadcast(&mut self, peers: &mut Pl, data: Data) -> Result<()>;
}

/// Transport receiver trait allows us to create multiple `Data` receiving only services.
/// `TransportReceiver` trait requires the same 4 parameter types as `Transport` trait above.
///
/// NOTE: `TransportReceiver` must implement Stream trait from async/.await framework.
pub trait TransportReceiver<Id, Data, Error, Pl>: Stream<Item = Data> + Drop + Unpin
where
    Id: PeerId,
    Pl: PeerList<Id, Error>,
    Data: DeserializeOwned,
{
    /// Creates a new Transport type using a preset configuration type.
    fn new(set_bind_net_addr: String) -> Result<Self>
    where
        Self: Sized;
}

// Imports
pub mod errors;
pub mod generic_test;
