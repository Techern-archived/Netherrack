//! Functions relating to network packets

extern crate varint;

use self::varint::Varint;

/// A struct defining the header of a Packet
#[derive(Clone, Debug)]
pub struct PacketHeader {
    
    /// The number of bytes contained in the ID and the packet data
    pub length: Varint,
    
    /// The ID of this packet
    pub id: Varint
    
}