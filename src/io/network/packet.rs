//! Functions relating to network packets

/// A struct defining the header of a Packet
#[derive(Clone, Debug)]
pub struct PacketHeader {
    
    /// The number of bytes contained in the ID and the packet data
    pub length: u32,
    
    /// The ID of this packet
    pub id: u32
    
}