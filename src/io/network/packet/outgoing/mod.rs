//! The module containing all outgoing packet implementations
//!
//! Enjoy

//pub mod handshake; No handshake packets (yet)!
pub mod status;

/*
use super::super::super::deque_buffer::DequeBuffer;
use super::super::super::game_connection::GameConnection; //It's super super super effective!
use super::super::{ PacketHeader, ID_STC_ };

/// A packet sent from the server to 
pub struct Packet {

    // Your fields here

}

impl Packet {

    /// Gets this Packet's ID
    pub fn get_id(&self) -> u32 {
    
        ID_STC_
    
    }

    /// Creates a new Packet
    pub fn new() -> Packet {
    
        Packet { /*etc*/ }
        
    }
    
    /// Encodes and sends this Packet
    pub fn send(&self, connection: &mut GameConnection) {
        
        let mut data_buffer = DequeBuffer::new();
        
        data_buffer.write_unsigned_varint_32(self.get_id());
        
        //Write your data here
        
        let mut length_buffer = DequeBuffer::new();
        
        length_buffer.write_unsigned_varint_32(data_buffer.remaining() as u32); //TODO: Try to get this as write_unsigned_varint_32_front
        
        let buffer: Vec<u8> = length_buffer.data.into_iter().chain(data_buffer.data.into_iter()).collect();
        
        let _buffer: &[u8] = &buffer[..];
        
        if connection.stream.write(_buffer).is_ok() {
            let _ = connection.stream.flush();
        }
        
    }

}
*/
