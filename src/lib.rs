/// # Fantom libtransport
///
/// This library defines a trait for the management and definition of specific transport prototcols
/// (such as TCP/IP or UDP). This repository solely defines the trait and provides no further
/// functionality. If you want a specific implementation, please check the transport-tcp repo for a
/// complete implementation: https://github.com/Fantom-foundation/libtransport-tcp.
///
/// Currently, two traits are defined: TransportConfiguration and Transport.
///
/// # TransportConfiguration
///
/// Currently the TransportConfiguration trait requires the definition of a type: Data. Data can
/// be of any type which needs to be transmitted across the network. An example of a 'Data'
/// definition can be:
///
/// ```
/// // An arbritrary data type which should be transmitted across the network
/// pub struct Data(pub u32);
///
/// ```
/// This trait also comes with two functions which need to be implemented: 'new' and
/// 'set_bind_net_addr'. These will be addressed below.
///
/// A transport configuration is essential for the Transport trait to function properly.
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
/// <b>Configuration:</b> The TransportConfiguration type required to make the function work.
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

pub enum TransportType {
    Unknown,
    TCP,
}

/// The TransportConfiguration trait - used to configure the transmission and protocol type.
///
/// # Examples
/// ```
/// use std::net::TcpListener;
/// use libtransport::TransportConfiguration;
/// use crate::libtransport::errors::Error;
/// use serde::export::PhantomData;
/// use core::mem;
///
/// pub struct Data(pub u32);
///
/// pub struct ExampleTransportConfig<Data> {
///
///     bind_net_addr: String,
///     listener : TcpListener,
///     data : PhantomData<Data>
/// }
///
/// impl<Data> TransportConfiguration<Data> for ExampleTransportConfig<Data> {
///
///     // Creates a new configuration type and binds the given address to a listener
///     fn new(set_bind_net_addr: String) -> Result<Self, Error> where
///         Self: Sized {
///
///         let listener = TcpListener::bind(set_bind_net_addr.clone())?;
///
///         Ok(ExampleTransportConfig {
///             bind_net_addr: set_bind_net_addr,
///             listener,
///             data: PhantomData
///         })
///     }
///
///     // Used to change the listener address. Binds input address to a new listener
///     fn set_bind_net_addr(&mut self,address: String) -> Result<(), Error> {
///
///         self.bind_net_addr = address;
///
///         let listener = TcpListener::bind(self.bind_net_addr.clone()).unwrap();
///
///         drop(mem::replace(&mut self.listener, listener));
///         Ok(())
///     }
/// }
/// ```
pub trait TransportConfiguration<Data> {
    /// Creates a new configuration with a specified network, taking the address of the incoming
    /// messages listener.
    /// Requires a network address as a String.
    /// For an example of an implementation of this function, check the libtransport-tcp repository.
    fn new(set_bind_net_addr: String) -> Result<Self>
    where
        Self: Sized;

    /// Binds the network address tor the incoming messages listener
    fn set_bind_net_addr(&mut self, address: String) -> Result<()>;
}

/// Transport trait allows us to create multiple message sending/receiving services which share
/// similar functionality.
///
/// The Transport trait requires 5 types to work:
/// Id: The peer's ID type.
/// Data: The data being transmitted.
/// Error: An error returned by the PeerList trait
/// Pl: A list of Peers (PeerList trait struct)
/// Configuration: A struct implementing TransportConfiguration
///
/// NOTE: TCP Transport should implement Stream in order for this to be accepted.
///
/// For an example of how this trait can be implemented, please look at the libtransport-tcp
/// repository: https://github.com/Fantom-foundation/libtransport-tcp

pub trait Transport<Id, Data, Error, Pl, Configuration>:
    Stream<Item = Data> + Drop + Unpin
where
    Id: PeerId,
    Pl: PeerList<Id, Error>,
    Data: Serialize + DeserializeOwned,
    Configuration: TransportConfiguration<Data>,
{
    /// Creates a new Transport type using a preset configuration type.
    fn new(cfg: Configuration) -> Self
    where
        Self: Sized;

    /// Sends a message of type 'Data' to the specified peer (as specified by an address)
    fn send(&mut self, peer_address: String, data: Data) -> Result<()>;

    /// Broadcasts a message of type 'Data' to all peers on the network. Requires a struct which
    /// implements PeerList.
    fn broadcast(&mut self, peers: &mut Pl, data: Data) -> Result<()>;
}

// Imports
pub mod errors;
pub mod generic_test;
