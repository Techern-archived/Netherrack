/// Implements all outgoing status packets

use std::io::Write;
use std::mem::transmute;

use super::super::super::deque_buffer::DequeBuffer;
use super::super::super::game_connection::GameConnection; //It's super super super effective!
use super::super::{ PacketHeader, ID_STATUS_STC_RESPONSE };

/// A packet sent from the server to 
pub struct StatusResponsePacket;

impl StatusResponsePacket {

    /// Gets this StatusResponsePacket's ID
    pub fn get_id(&self) -> u32 {
    
        ID_STATUS_STC_RESPONSE
    
    }
    
    /// Gets the JSON response
    fn get_json_response(&self) -> String {
"{
    \"version\": {
        \"name\": \"1.8.8\",
        \"protocol\": 47
    },
    \"players\": {
        \"max\": 100,
        \"online\": 5
    },	
    \"description\": {
        \"text\": \"Hello world\"
    }
}".to_string()
    }

    /// Creates a new Packet
    pub fn new() -> StatusResponsePacket {
    
        StatusResponsePacket
        
    }
    
    /// Encodes and sends this StatusResponsePacket
    pub fn send(&self, connection: &mut GameConnection) {
        info!("Sending a server status response packet!");
        
        let mut data_buffer = DequeBuffer::new();
        
        data_buffer.write_unsigned_varint_32(self.get_id());
        
        data_buffer.write_utf8_string(self.get_json_response());
        
        let mut length_buffer = DequeBuffer::with_capacity(data_buffer.remaining());
        
        length_buffer.write_unsigned_varint_32(data_buffer.remaining() as u32); //TODO: Try to get this as write_unsigned_varint_32_front
        
        let buffer = &(length_buffer.data.iter().chain(data_buffer.data.iter()).collect::<Vec<&u8>>())[..];
              
        let _buffer: &[u8] = unsafe { transmute(buffer) };  
                
        info!("{} + {} = {}", length_buffer.remaining(), data_buffer.remaining(), _buffer.len());
        
        info!("Flush? {:?}", connection.stream.flush());
        
        let result = connection.stream.write(_buffer);
        
        if (result.is_ok()) {
            info!("Flush? {:?}", connection.stream.flush());
        }
        
        info!("Wrote a {:?}", result);
        
    }

}