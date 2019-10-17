libtransport
===========
[![Build Status](https://travis-ci.org/Fantom-foundation/libtransport.svg?branch=master)](https://travis-ci.org/Fantom-foundation/libtransport)

libtransport in Rust.

## RFCs

https://github.com/Fantom-foundation/fantom-rfcs

# Developer guide

Install the latest version of [Rust](https://www.rust-lang.org). We tend to use nightly versions. [CLI tool for installing Rust](https://rustup.rs).

We use [rust-clippy](https://github.com/rust-lang-nursery/rust-clippy) linters to improve code quality.

There are plenty of [IDEs](https://areweideyet.com) and other [Rust development tools to consider](https://github.com/rust-unofficial/awesome-rust#development-tools).

### Step-by-step guide
```bash
# Install Rust (nightly)
$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
# Install cargo-make (cross-platform feature-rich reimplementation of Make)
$ cargo install --force cargo-make
# Install rustfmt (Rust formatter)
$ rustup component add rustfmt
# Clone this repo
$ git clone https://github.com/Fantom-foundation/libtransport && cd libtransport
# Run tests
$ cargo test
# Format, build and test
$ cargo make
```

### Example ###

#### Prelude
```rust
use libtransport::TransportReceiver;
use libtransport::TransportSender;
use libtransport_tcp::receiver::TCPreceiver;
use libtransport_tcp::sender::TCPsender;
```


#### Step-by-step

**Prepare configuration**
```rust
let (transport_type, reply_bind_address) = {
    let cfg = config.read().unwrap();
    (cfg.transport_type.clone(), cfg.reply_addr.clone())
};
```

**setup `TransportSender` for `Sync` `Request`.**
```rust
let mut sync_req_sender = {
    match transport_type {
        libtransport::TransportType::TCP => {
            TCPsender::<P, SyncReq<P>, errors::Error, peer::DAGPeerList<P, PK>>::new().unwrap()
        }
        libtransport::TransportType::HTTP => {
            HTTPsender::<P, SyncReq<P>, errors::Error, peer::DAGPeerList<P, PK>>::new().unwrap()
        }
        libtransport::TransportType::Unknown => panic!("unknown transport"),
    }
};
let mut sync_reply_receiver = {
    match transport_type {
        libtransport::TransportType::TCP => {
            let x: TCPreceiver<P, SyncReply<D, P, PK, Sig>, Error, DAGPeerList<P, PK>> =
                TCPreceiver::new(reply_bind_address).unwrap();
            x
        }
        libtransport::TransportType::HTTP => {
            let x: HTTPreceiver<P, SyncReply<D, P, PK, Sig>, Error, DAGPeerList<P, PK>> =
                TCPreceiver::new(reply_bind_address).unwrap();
            x
        }
        libtransport::TransportType::Unknown => panic!("unknown transport"),
    }
};
```

**An example how to send data**
```rust
match sync_req_sender.send(peer.request_addr, request) {
    Ok(()) => {}
    Err(e) => error!("error sending sync request: {:?}", e),
}
```

**An example how to receive data**
```rust
block_on(async {
  if let Some(sync_reply) = sync_reply_receiver.next().await {
      debug!(
          "{} Sync Reply from {}",
          sync_reply.to.clone(),
          sync_reply.from.clone()
      );
      // do processing here for sync_reply received
  }
});

```
