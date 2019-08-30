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
/// a) Creating a set of peers
/// b) Sending data between multiple peers
/// c) Broadcasting messages to all peers within the same network.
///
/// The Transport trait requires the definition of a number of types to work properly:
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
/// Finally, the Transport trait implements three methods: 'new', 'send', and 'broadcast'
///
/// For further examples on how you can use the Transport trait, please look at the 'generic_test.rs'
/// file for a simple implementation.
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
    /// For an example of an implementation of this function, check the libtransport.tcp repository.
    fn new(set_bind_net_addr: String) -> Result<Self>
    where
        Self: Sized;

    /// Binds the network address tor the incoming messages listener
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
pub trait Transport<Id, Data, Error, Pl, Configuration>:
    Stream<Item = Data> + Drop + Unpin
where
    Id: PeerId,
    Pl: PeerList<Id, Error>,
    Data: Serialize + DeserializeOwned,
    // transport configuration type
    Configuration: TransportConfiguration<Data>,
{
    // Create new Transport instance
    fn new(cfg: Configuration) -> Self
    where
        Self: Sized;

    // send specified message to the specified peer
    fn send(&mut self, peer_address: String, data: Data) -> Result<()>;

    // broadcast specified message to all peers
    fn broadcast(&mut self, peers: &mut Pl, data: Data) -> Result<()>;
}

pub mod errors;
pub mod generic_test;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
