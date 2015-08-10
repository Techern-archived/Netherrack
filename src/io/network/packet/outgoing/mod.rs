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
    pub fn get_id() -> u32 {
    
        ID_STC_
    
    }

    /// Creates a new Packet
    pub fn new() -> Packet {
    
        Packet { /*etc*/ }
        
    }
    
    /// Encodes and sends this Packet
    pub fn send(&self, connection: &mut GameConnection) {
        info!("Sending a packet!");
        
        let mut data_buffer = DequeBuffer::new();
        
        data_buffer.write_unsigned_varint_32(self.get_id());
        
        // Write your data here
        
        let mut length_buffer = DequeBuffer::with_capacity(data_buffer.remaining());
        
        length_buffer.write_unsigned_varint_32(data_buffer.remaining() as u32); //TODO: Try to get this as write_unsigned_varint_32_front
        
        let buffer = &(length_buffer.data.iter().chain(data_buffer.data.iter()).collect::<Vec<&u8>>())[..];
              
        let _buffer: &[u8] = unsafe { transmute(buffer) };  
                
        info!("{} + {} = {}", length_buffer.remaining(), data_buffer.remaining(), _buffer.len());
        
        let result = connection.stream.write_all(_buffer);
        
        if (result.is_ok()) {
            connection.stream.flush();
        }
        
        info!("Wrote a {:?}", result);
        
    }

}
*/
