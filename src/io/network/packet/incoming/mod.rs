//! The module containing all incoming packet implementations
//!
//! Any unimplemented packets currently print out the ID and length for future reference (see game_connection.rs)

pub mod handshake;
pub mod status;

/*
//! A packet

use super::super::super::deque_buffer::DequeBuffer;
use super::super::super::game_connection::{ConnectionState, GameConnection}; //It's super super super effective!
use super::super::PacketHeader;

/// A packet sent from the client to 
pub struct Packet {

    /// The header of this Packet
    pub header: PacketHeader,
    
    // Your fields here

}

impl Packet {

    /// Decodes this Packet
    ///
    /// **DO NOT DO ANYTHING EXCEPT DECODING!**
    ///
    /// Actual functionality will be handled in the handle function
    pub fn decode(header: PacketHeader, connection: &mut GameConnection, buffer: &mut DequeBuffer) -> Packet {
                
        // Decode here
        
        Packet { header: header /*etc*/ }
        
    }
    
    /// Handles this Packet
    pub fn handle(&self, connection: &mut GameConnection) {
        info!("Got a packet!");
        
        // Ooooh, handle the packet
    }

}
*/
