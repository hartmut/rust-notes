extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
use std::net::ToSocketAddrs;
use futures::Future;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = "localhost:17001".to_socket_addrs().unwrap().next().unwrap();
    let tcp = TcpStream::connect(&addr, &handle);

    let request = tcp.and_then ( |socket| {
        tokio_io::io::write_all(socket, "
        help \r\n
        quit \r\n
        ".as_bytes())
    });

    let response = request.and_then(|(socket, _request)| {
        tokio_io::io::read_to_end(socket, Vec::new())
    });

    let (_socket, data) = core.run(response).unwrap();
    println!("{}", String::from_utf8_lossy(&data));
}
