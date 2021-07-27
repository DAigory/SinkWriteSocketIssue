pub mod models;
pub mod client;

use actix::*;
use actix::io::SinkWrite;
use awc::Client;

use futures::StreamExt;
use futures::stream::{SplitSink, SplitStream};

use std::{io, thread};
use std::io::prelude::*;

 fn main() {
    println!("hello!");
  //  ::std::env::set_var("RUST_LOG", "actix_web=info");
  //  env_logger::init();
    let sys = actix::System::new();
    let art = Arbiter::new();
    spawn(async move {
        let ws_conn = Client::new()
            .ws("ws://localhost:8080/echo/")
            .connect().await.map_err(|e| {
                println!("Error: {}", e);
            })
            .map(|(response, framed)| {
                println!("{:?}", response);
                let (sink, streamSplit) = framed.split();
               
             
                let addr = client::WsClient::create(|ctx| {
                    let a = streamSplit;
                    let b = ctx;
                 //   client::WsClient::add_stream( streamSplit, ctx);
                    client::WsClient(SinkWrite::new(sink, ctx))
                });

                // start console loop
                thread::spawn(move || loop {
                    let mut cmd = String::new();
                    if io::stdin().read_line(&mut cmd).is_err() {
                        println!("error");
                        return;
                    }
                    addr.do_send(client::ClientCommand(cmd.trim().to_string()));
                });
            });
    });
 
    let _ = sys.run();
}





