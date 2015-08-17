//! The module containing all outgoing packet implementations

use super::super::deque_buffer::DequeBuffer;
use super::super::game_connection::GameConnection; //It's super super super effective!

use std::io::Write;

//pub mod handshake; No handshake packets (yet)!
pub mod status;

/// A trait for outgoing packets
pub trait OutgoingPacket {

    /// Gets this Packet's ID
    ///
    /// One day, this will be replaced with an associated constant
    fn get_id(&self) -> u32;

    /// Encodes this Packet's data
    fn encode_data(&self, buffer: &mut DequeBuffer);

    /// Encodes this OutgoingPacket
    fn encode(&self, buffer: &mut DequeBuffer) {
        
        buffer.write_unsigned_varint_32(self.get_id());
        
        self.encode_data(buffer);
        
    }
    
    /// Sends this OutgoingPacket to the client
    fn send(&self, connection: &mut GameConnection) {
    
        let mut data_buffer = DequeBuffer::new();
        
        self.encode(&mut data_buffer);
        
        let mut length_buffer = DequeBuffer::new();
        
        length_buffer.write_unsigned_varint_32(data_buffer.remaining() as u32); //TODO: Try to get this as data_buffer.prepend_varint_32 - One day
        
        let buffer: Vec<u8> = length_buffer.data.into_iter().chain(data_buffer.data.into_iter()).collect();
        
        let _buffer: &[u8] = &buffer[..];
        
        if connection.stream.write(_buffer).is_ok() {
            let _ = connection.stream.flush();
        }
    
    }

}

/*
use super::super::super::deque_buffer::DequeBuffer;
use super::super::super::game_connection::GameConnection; //It's super super super effective!
use super::super::{ PacketHeader, ID_STC_ };

/// A packet sent from the server to 
pub struct Packet {

    // Your fields here

}

impl OutgoingPacket for Packet {

    
    /// Gets this Packet's ID
    ///
    /// One day, this will be replaced with an associated constant
    fn get_id(&self) -> u32 {
        ID_STC_
    }

    /// Encodes this Packet's data
    fn encode_data(&self, buffer: &mut DequeBuffer) {
    
    }

    /// Creates a new Packet
    pub fn new() -> Packet {
    
        Packet { /*etc*/ }
        
    }

}
*/
