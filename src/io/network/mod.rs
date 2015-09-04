//! Functions relating to network operations.
//!
//! This will eventually be moved

pub mod game_connection;
use self::game_connection::GameConnection;

pub mod packet;

use std::net::TcpListener;

use std::thread;


fn start_listening(port: u16) -> Result<TcpListener, &'static str> {

    let listener = TcpListener::bind(("0.0.0.0", port));

    if listener.is_ok() {
        return Ok(listener.unwrap());
    } else {
        return Err("Could not listen on requested port");
    }

}

pub fn start_network() {

    let listener = start_listening(25565);

    if listener.is_err() {
        error!("Could not start listening on port {}", 25565);
    }

    let listener = listener.unwrap();

    for stream in listener.incoming() {

        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    // New connection succeeded
                    //TODO: Add this connection to a list of connections
                    GameConnection::new(stream).start_listening()
                });
            }
            Err(e) => {
                error!("Could not accept a new stream: {}", e);
            }
        }

    }

    drop(listener);

}
