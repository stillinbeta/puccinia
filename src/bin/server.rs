extern crate cursive;
extern crate futures;
extern crate termiontelnet;
extern crate tokio;

use crossbeam_channel::unbounded;
use cursive::backend::termiontelnet::{Connection, TelnetEvent};
use futures::Stream;
use std::io::{Error, ErrorKind};
use std::thread;
use termiontelnet::{ClientEvents::*, ServerEvents, TelnetOption};
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() {
    let listen_addr = std::env::var("BIND_ADDRESS").unwrap_or("127.0.0.1".into());
    let listen_port = std::env::var("PORT").unwrap_or("12345".into());

    let addr = format!("{}:{}", listen_addr, listen_port).parse().unwrap();
    let listener = TcpListener::bind(&addr).expect("couldn't listen");

    println!("Listening on {}", addr);

    let server = listener
        .incoming()
        .map_err(|e| eprintln!("An error occured: {:?}", e))
        .for_each(|sock| {
            let framed = termiontelnet::TelnetCodec::framed(sock);

            let (sink, stream) = framed.split();

            let task = sink
                .send_all(futures::stream::iter_ok::<_, std::io::Error>(vec![
                    // Set up the telnet terminal for Cursive
                    ServerEvents::Do(TelnetOption::WindowSize),
                    ServerEvents::Will(TelnetOption::Echo),
                    ServerEvents::Do(TelnetOption::LineMode),
                    ServerEvents::EnableMouse,
                ]))
                .and_then(move |(sink, _err)| {
                    let (s, r) = unbounded();

                    let c = Connection {
                        events: r,
                        sink: Box::new(sink),
                    };

                    let _handler = thread::spawn(|| {
                        // TODO: handle error gracefully
                        let mut c = cursive::Cursive::termion_telnet(c).unwrap();
                        puccinia::setup_cursive(&mut c);
                        c.run()
                    });

                    stream.for_each(move |msg| {
                        let evt = match msg {
                            ResizeEvent(w, h) => TelnetEvent::ResizeEvent(h, w),
                            TermionEvent(evt) => TelnetEvent::TEvent(evt),
                            _ => return future::ok(()),
                        };

                        future::result(s.send(evt).map_err(|err| Error::new(ErrorKind::Other, err)))
                    })
                })
                .map_err(|e| eprintln!("An error occured: {:?}", e));

            tokio::spawn(task)
        });

    tokio::run(server);
}
