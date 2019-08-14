//#![feature(generic_associated_types)]
use crate::errors::Result;
use libcommon_rs::peer::{PeerId, PeerList};
use os_pipe::PipeWriter;
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

    // Register a PipeWriter of os_pipe::pipe; which is used to push
    // all received data blocks to.
    // Several pipes can be registered, they will be pushed in
    // the order of registration.
    fn register_os_pipe(&mut self, sender: PipeWriter) -> Result<()>;

    // register a callback function which is called when data is received.
    // Several callback functions can be registered, they will be called in
    // the order of registration.
    // The callback function must return True when transaction is processed successfully
    // and False otherwise. The same callback function will be called with the same data
    // until callback function return True; a pause between  consecutive calls of the
    // callback function with the same block will be made for the value of milliseconds
    // set by set_callback_timeout() function of the TRansportConfiguration trait;
    // default value of the timeout is implementation defined.
    fn register_callback(&mut self, callback: fn(data: Data) -> bool) -> Result<()>;

    // Set timeout in milliseconds between consecutive calls of the callback
    // function with the same data received.
    fn set_callback_timeout(&mut self, timeout: u64);

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
