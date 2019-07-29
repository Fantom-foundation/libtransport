trait Transport: Send + Receive + Connection {
    
}

trait Send: Ping {
    fn send(connection: Connection, data: Vec<u8>) -> Result<u16 /* bytes sent */, Error>;
}


trait Receive: Ping {
    fn receive(connection: Connection, data: Vec<u8>) -> Result<u16 /* bytes received */, Error>;
}

trait Ping {
    fn ping(connection: Connection) -> Result<(), Error>;
}

trait Connection {
    type Address;

    fn connect(address: Address) -> Result<(), Error>;
    fn disconnect(address: Address) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
