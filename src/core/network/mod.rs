//! Functions relating to network operations.
//!
//! This will eventually be moved

use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

use std::io::Read;

fn start_listening(port: u16) -> Result<TcpListener, &'static str> {

    let listener = TcpListener::bind(("127.0.0.1", port));
    
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
                    handle_new_client(stream)
                });
            }
            Err(e) => { 
                error!("Could not accept a new stream: {}", e);
            }
        }
    
    }
    
    drop(listener);

}

pub fn handle_new_client(mut stream: TcpStream) {
    info!("Got a new client from {}! Oh my!", stream.peer_addr().unwrap());
    
    loop {
    
        let mut buffer = [0u8; 512];
        
        match stream.read(&mut buffer) {
        
            Err(error) => {
                // Connection dropped out or something broke
                // In the future, we use the customized NetherrackStream to see if we're in GAME mode, and if so, get the attached player and start saving their information and get rid of them
                // For now, we just exit the loop
                error!("Got an error! {}",  error);
                
                stream.shutdown(Shutdown::Both);
                break; //Close the connection
            }
            Ok(count) => {
                if (count == 0) {
                    ::std::thread::sleep_ms(2);
                } else {
                    info!("Read {} bytes, information is {}: {}: {}: {}", count, buffer[0], buffer[1], buffer[2], buffer[3]);
                }
            }
        
        }
    
    }
    
    info!("Connection closed");
    
}