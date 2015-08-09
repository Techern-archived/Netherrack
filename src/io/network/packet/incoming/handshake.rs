//! A handshake request packet

use super::super::super::deque_buffer::DequeBuffer;
use super::super::super::game_connection::GameConnection; //It's super super super effective!
use super::super::PacketHeader;

/// A packet sent from the client to initiate a handshake
pub struct HandshakePacket {

    pub header: PacketHeader,
    
    pub protocol_version: u32,
    
    pub server_address: String,
    
    pub server_port: u16,
    
    pub next_state: u32

}

impl HandshakePacket {

    /// Decodes this HandshakePacket
    pub fn decode(header: PacketHeader, connection: &mut GameConnection, buffer: &mut DequeBuffer) -> HandshakePacket {
        
        let header = header;
        
        let protocol_result = buffer.read_unsigned_varint_32();
        
        if protocol_result.is_err() {
            error!("Error reading protocol version");
            connection.disconnect();
        }
        
        let hostname: String = buffer.read_utf8_string();
        
        let port: u16 = buffer.read_unsigned_short();
        
        let next_state_result = buffer.read_unsigned_varint_32();
        
        if next_state_result.is_err() {
            error!("Invalid next state: {:?}", next_state_result);
            connection.disconnect();
        }
        
        debug!("Protocol version {}, hostname: {}, port: {}, next_state: {}", protocol_result.unwrap(), hostname, port, next_state_result.unwrap());
        
        HandshakePacket { header: header, protocol_version: protocol_result.unwrap(), server_address: hostname, server_port: port, next_state: next_state_result.unwrap() }
        
    }
    
    pub fn handle(&self, connection: &mut GameConnection) {
        info!("Handling handshake!");
    }

}