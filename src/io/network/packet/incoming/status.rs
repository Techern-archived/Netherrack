//! Handles all status packets
/*
use super::super::super::deque_buffer::DequeBuffer;
use super::super::super::game_connection::GameConnection; //It's super super super effective!

use super::super::PacketHeader;

use super::super::outgoing::OutgoingPacket;
use super::super::outgoing::status::{StatusResponsePacket, ListPongPacket};

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
    #[allow(unused_variables)]
    pub fn decode(header: PacketHeader, connection: &mut GameConnection, buffer: &mut DequeBuffer) -> StatusRequestPacket {

        // Nothing to do here

        StatusRequestPacket { header: header /*etc*/ }

    }

    /// Handles this StatusRequestPacket
    pub fn handle(&self, connection: &mut GameConnection) {
        StatusResponsePacket::new().send(connection);
    }

}

/// A list ping packet sent from the client to the server
pub struct ListPingPacket {

    /// The header of this ListPingPacket
    pub header: PacketHeader,

    /// The ping ID (usually the system time in milliseconds)
    pub payload: i64

}

impl ListPingPacket {

    /// Decodes this Packet
    ///
    /// **DO NOT DO ANYTHING EXCEPT DECODING!**
    ///
    /// Actual functionality will be handled in the handle function
    #[allow(unused_variables)]
    pub fn decode(header: PacketHeader, connection: &mut GameConnection, buffer: &mut DequeBuffer) -> ListPingPacket {

        let id: i64 = buffer.read_signed_long();

        ListPingPacket { header: header, payload: id }

    }

    /// Handles this ListPingPacket
    pub fn handle(&self, connection: &mut GameConnection) {
        ListPongPacket::new(self.payload).send(connection);
    }

}*/
