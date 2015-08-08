//! Functions relating to network operations.
//!
//! This will eventually be moved

pub mod deque_buffer;

pub mod packet;

use std::collections::VecDeque;

use std::net::{Shutdown, TcpListener, TcpStream};

use std::thread;

use std::io::Read;

use self::deque_buffer::DequeBuffer;

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
    
        let mut raw_buffer = vec![0u8; 512];
        
        match stream.read(&mut raw_buffer) {
        
            Err(error) => {
                // Connection dropped out or something broke
                // In the future, we use the customized NetherrackStream to see if we're in GAME mode, and if so, get the attached player and start saving their information and get rid of them
                // For now, we just exit the loop
                error!("Got an error! {}", error);
                
                let _ = stream.shutdown(Shutdown::Both);
                break; //Close the connection
            }
            Ok(count) => {
                if count == 0 {
                    //Nothing in stream, let's sleep for a few milliseconds and try again
                    ::std::thread::sleep_ms(2);
                } else {
                
                    // Now truncate the raw buffer - Don't worry, it will reallocate to 512 on the next networking loop
                    raw_buffer.truncate(count);
                    
                    let mut backing: VecDeque<u8> = VecDeque::<u8>::with_capacity(count);
                    
                    for b in raw_buffer {
                        
                        backing.push_back(b);
                    }
                
                    let mut buffer = DequeBuffer::from(backing);
                    
                    //drop(raw_buffer);
                    
                    let length_result = buffer.read_unsigned_varint_32();
                    let id_result = buffer.read_unsigned_varint_32();
                    
                    if length_result.is_err() || id_result.is_err() {
                        error!("Error while reading packet ID or length! Oh no, dropping connection");
                        
                        debug!("Shutdown result: {:?}", stream.shutdown(Shutdown::Both));
                        break;
                    }
                
                    info!("Read {} bytes, length is {}, id is {}", count, length_result.unwrap(), id_result.unwrap());
                    
                    info!("Remaining buffer size is {}", buffer.remaining());
                    
                    // This is a memory leak. We need to get the time of last packet and check to see if it's more than 20 seconds ago
                }
            }
        
        }
    
    }
    
    info!("Connection closed");
    
}