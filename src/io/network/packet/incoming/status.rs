//! Handles all status packets

use super::super::super::deque_buffer::DequeBuffer;
use super::super::super::game_connection::{ConnectionState, GameConnection}; //It's super super super effective!
use super::super::PacketHeader;

/// A packet sent from the client to make the server send a status response packet
pub struct StatusRequestPacket {

    /// The header of this StatusRequestPacket
    pub header: PacketHeader

}

impl StatusRequestPacket {

    /// Decodes this StatusRequestPacket
    ///
    /// **DO NOT DO ANYTHING EXCEPT DECODING!**
    ///
    /// Actual functionality will be handled in the handle function
    pub fn decode(header: PacketHeader, connection: &mut GameConnection, buffer: &mut DequeBuffer) -> StatusRequestPacket {
                
        // Nothing to do here
        
        StatusRequestPacket { header: header /*etc*/ }
        
    }
    
    /// Handles this StatusRequestPacket
    pub fn handle(&self, connection: &mut GameConnection) {
        info!("Got a status request!");
        
        
    }

}