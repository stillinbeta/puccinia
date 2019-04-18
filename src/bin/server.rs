extern crate futures;
extern crate termiontelnet;
extern crate tokio;

use futures::Stream;
use termiontelnet::{ServerEvents, TelnetOption};
use tokio::net::TcpListener;
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
        .map_err(|e| eprintln!("An error occured: {:?}", e))
        .for_each(|sock| {
            let ip_addr: u16 = sock.peer_addr().map(|p| p.port()).unwrap_or(0);
            let framed = termiontelnet::TelnetCodec::framed(sock);

            let (sink, stream) = framed.split();

            let task = sink
                .send_all(futures::stream::iter_ok::<_, std::io::Error>(vec![
                    ServerEvents::Do(TelnetOption::WindowSize),
                    ServerEvents::Will(TelnetOption::Echo),
                    ServerEvents::Do(TelnetOption::LineMode),
                ]))
                .and_then(move |_sink| {
                    stream.for_each(move |msg| {
                        println!("Received {:?} from {}", msg, ip_addr);
                        future::ok(())
                    })
                })
                .map_err(|e| eprintln!("An error occured: {:?}", e));

            tokio::spawn(task)
        });

    tokio::run(server);
}
