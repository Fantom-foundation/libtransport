use crate::errors::Result;
use std::sync::mpsc::Sender;

// Transport trait for various implementations of message
// sending/receiving services
//
// Address - external address type; e.g. PeerId for Consensus.
// Translation into internal addresses is done in the implementation
// based on configuration supplied, or based on implementation's internal data
//
// Data - Transmitting data type;
// it can be a truct containing message type and payload data
pub trait Transport<Address, Data>
where
    Data: AsRef<u8>,
{
    // transport configuration type
    type Configuration;

    // Create new Transport instance
    fn new(cfg: Self::Configuration) -> Self;

    // send specified message to the specified peer
    fn send(&mut self, peer: &Address, data: Data) -> Result<()>;

    // broadcast specified message to all peers
    fn broadcast(&mut self, data: Data) -> Result<()>;

    // register a sending-half of std::sync::mpsc::channel which is used to push
    // all received messages to.
    // Several channels can be registered, they will be pushed in
    // the order of registration.
    fn register_channel(&mut self, sender: Sender<Data>) -> Result<()>;
}

mod errors;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
