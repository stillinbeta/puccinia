extern crate futures;
extern crate termiontelnet;
extern crate tokio;

use futures::Stream;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    StringErr(String),
}

fn main() {
    let addr = "127.0.0.1:12345".parse().unwrap();
    let listener = TcpListener::bind(&addr).expect("couldn't listen");

    let server = listener
        .incoming()
        .for_each(|sock| {
            let framed = termiontelnet::TelnetCodec::framed(sock);

            let (sink, stream) = framed.split();

            sink.send(termiontelnet::ServerEvents::IACDoNAWS)
                .and_then(|_sink| {
                    stream
                        .for_each(|msg| {
                            println!("Received {:?}", msg);
                            future::ok(())
                        })
                        .map(|_| eprintln!("connection closed"))
                })
        })
        .map_err(|e| eprintln!("An error occured: {:?}", e));

    tokio::run(server);
}
