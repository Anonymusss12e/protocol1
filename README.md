# protocol

[![Build Status](https://travis-ci.org/dylanmckay/protocol.svg?branch=master)](https://travis-ci.org/dylanmckay/protocol)
[![Crates.io](https://img.shields.io/crates/v/protocol.svg)](https://crates.io/crates/protocol)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

[Documentation](https://docs.rs/protocol)

Easy protocol definitions in Rust.

This crate adds a custom derive that can be added to types, allowing
structured data to be sent and received from any IO stream.

Networking is built-in, with special support for TCP and UDP.

The protocol you define can be used outside of networking too - see the `Parcel::from_raw_bytes` and `Parcel::raw_bytes` methods.

This crate also provides:

* [TCP](https://docs.rs/protocol/latest/protocol/wire/stream/index.html) and [UDP](https://docs.rs/protocol/latest/protocol/wire/dgram/index.html) modules for easy sending and receiving of `Parcel`s
* A generic [middleware](https://docs.rs/protocol/latest/protocol/wire/middleware/index.html) library for automatic transformation of sent/received data
  * Middleware has already been written to support [compression](https://docs.rs/protocol/latest/protocol/wire/middleware/compression/index.html)
  * Custom middleware can be implemented via a trait with two methods

Checkout the [examples](./examples) folder for usage.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
protocol = { version = "3.4", features = ["derive"] }
```

And then define a type with the `#[derive(protocol::Protocol)]` attribute:

```rust
#[derive(protocol::Protocol)]
struct Hello {
    pub a: String,
    pub b: u32,
}
```

## Under the hood

The most interesting part here is the [`protocol::Parcel`](https://docs.rs/protocol/latest/protocol/trait.Parcel.html) trait. Any type that implements this trait can then be serialized to and from a byte stream. All primitive types, standard collections, tuples, and arrays implement this trait.

This crate becomes particularly useful when you define your own `Parcel` types. You can use `#[derive(protocol::Protocol)]` to do this. Note that in order for a type to implement `Parcel`, it must also implement `Clone`, `Debug`, and `PartialEq`.

```rust
#[derive(Parcel, Clone, Debug, PartialEq)]
pub struct Health(f32);

#[derive(Parcel, Clone, Debug, PartialEq)]
pub struct SetPlayerPosition {
    pub position: (f32, f32),
    pub health: Health,
    pub values: Vec<String>,
}
```

### Custom derive

Any user-defined type can have the `Parcel` trait automatically derived.

## Example

```rust
#[derive(protocol::Protocol, Clone, Debug, PartialEq)]
pub struct Handshake;

#[derive(protocol::Protocol, Clone, Debug, PartialEq)]
pub struct Hello {
    id: i64,
    data: Vec<u8>,
}

#[derive(protocol::Protocol, Clone, Debug, PartialEq)]
pub struct Goodbye {
    id: i64,
    reason: String,
}

#[derive(protocol::Protocol, Clone, Debug, PartialEq)]
pub struct Node {
    name: String,
    enabled: bool
}

#[protocol(discriminant = "integer")]
#[derive(protocol::Protocol, Clone, Debug, PartialEq)]
pub enum PacketKind {
    #[protocol(discriminator(0x00))]
    Handshake(Handshake),
    #[protocol(discriminator(0xaa))]
    Hello(Hello),
    #[protocol(discriminator(0xaf))]
    Goodbye(Goodbye),
}

fn main() {
    use std::net::TcpStream;

    let stream = TcpStream::connect("127.0.0.1:34254").unwrap();
    let mut connection = protocol::wire::stream::Connection::new(stream, protocol::wire::middleware::pipeline::default());

    connection.send_packet(&Packet::Handshake(Handshake)).unwrap();
    connection.send_packet(&Packet::Hello(Hello { id: 0, data: vec![ 55 ]})).unwrap();
    connection.send_packet(&Packet::Goodbye(Goodbye { id: 0, reason: "leaving".to_string() })).unwrap();

    loop {
        if let Some(response) = connection.receive_packet().unwrap() {
            println!("{:?}", response);
            break;
        }
    }
}
```

## Enums

### Discriminators

Enum values can be transmitted either by their 1-based variant index, or by transmitting the string name of each variant.

**NOTE:** The default behaviour is to use *the variant name as a string* (`string`).

This behaviour can be changed by the `#[protocol(discriminant = "<type>")]` attribute.

Supported discriminant types:

* `string` (default)
    * This transmits the enum variant name as the over-the-wire discriminant
    * This uses more bytes per message, but it very flexible
* `integer`
    * This transmits the 1-based variant number as the over-the-wire discriminant
    * If enum variants have explicit discriminators, the
    * Enum variants cannot be reordered in the source without breaking the protocol


```rust
#[derive(protocol::Protocol, Clone, Debug, PartialEq)]
#[protocol(discriminant = "string")]
pub enum PlayerState {
  Stationary,
  Flying { velocity: (f32,f32,f32) },
  // Discriminators can be explicitly specified.
  #[protocol(discriminator("ArbitraryOverTheWireName"))]
  Jumping { height: f32 },
}
```

### Misc

You can rename the variant for their serialisation.

```rust
#[derive(protocol::Protocol, Clone, Debug, PartialEq)]
#[protocol(discriminant = "string")]
pub enum Foo {
  Bar,
  #[protocol(name = "Biz")] // the Bing variant will be send/received as 'Biz'.
  Bing,
  Baz,
}
```
