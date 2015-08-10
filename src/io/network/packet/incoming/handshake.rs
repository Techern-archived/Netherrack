//! Defines all handshake packets
//!
//! Thankfully, there's only one packet so far :)

use super::super::super::deque_buffer::DequeBuffer;
use super::super::super::game_connection::{ConnectionState, GameConnection}; //It's super super super effective!
use super::super::PacketHeader;

/// A packet sent from the client to initiate a handshake
pub struct HandshakePacket {

    /// The header of this HandshakePacket
    pub header: PacketHeader,
    
    /// The protocol version that the client is using
    pub protocol_version: u32,
    
    /// The address the client is using to connect to the server
    pub server_address: String,
    
    /// The port the client is connecting to the server with
    pub server_port: u16,
    
    /// The next state requested by the client
    pub next_state: u32

}

impl HandshakePacket {

    /// Decodes this HandshakePacket
    pub fn decode(header: PacketHeader, connection: &mut GameConnection, buffer: &mut DequeBuffer) -> HandshakePacket {
                
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
    
    /// Handles this HandshakePacket
    pub fn handle(&self, connection: &mut GameConnection) {
        info!("Got a handshake packet!");
        
        match self.next_state {
        
            1 => {
                connection.state = ConnectionState::STATUS;
            },
            2 => {
                connection.state = ConnectionState::LOGIN;
            }
            _ => {
                error!("Invalid handshake next state of {}, disconnecting", self.next_state);
                connection.disconnect();
            }
        
        }
    }

}