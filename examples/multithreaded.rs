#![feature(slicing_syntax)]
extern crate irc;

use std::default::Default;
use std::sync::Arc;
use std::thread::Thread;
use irc::data::config::Config;
use irc::server::{IrcServer, Server};
use irc::server::utils::Wrapper;

fn main() {
    let config = Config {
        nickname: Some(format!("pickles")),
        server: Some(format!("irc.fyrechat.net")),
        channels: Some(vec![format!("#vana")]),
        .. Default::default()
    };
    let irc_server = Arc::new(IrcServer::from_config(config).unwrap());
    // The wrapper provides us with methods like send_privmsg(...) and identify(...)
    let server = Wrapper::new(&*irc_server);
    server.identify().unwrap();
    let server = irc_server.clone();
    // We won't use a wrapper here because we don't need the added functionality.
    let _ = Thread::spawn(move || { 
        for msg in server.iter() {
            print!("{}", msg.unwrap().into_string());
        }
    }).join(); // You might not want to join here for actual multi-threading.
}
