//! Enable compression.
//! The default middleware pipeline supports compression, but is disabled
//! by default.

#![feature(question_mark)]

#[macro_use]
extern crate protocol;

pub const ALGORITHM: protocol::wire::middleware::compression::Algorithm = protocol::wire::middleware::compression::Algorithm::Zlib;

define_packet!(Hello {
    id: i64,
    data: Vec<u8>
});

fn main() {
    use std::net::TcpStream;

    let stream = TcpStream::connect("127.0.0.1:34254").unwrap();
    let mut connection = protocol::wire::stream::Connection::new(stream, protocol::wire::middleware::pipeline::default());

    connection.middleware.compression = protocol::wire::middleware::Compression::Enabled(ALGORITHM);

    connection.send_packet(&Hello { id: 0, data: vec![ 55 ]}).unwrap();

    loop {
        if let Some(response) = connection.receive_packet().unwrap() {
            println!("{:?}", response);
            break;
        }
    }
}

