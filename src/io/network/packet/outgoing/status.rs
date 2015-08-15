/// Implements all outgoing status packets

use std::io::Write;

use super::super::super::deque_buffer::DequeBuffer;
use super::super::super::game_connection::GameConnection; //It's super super super effective!
use super::super::{ ID_STATUS_STC_RESPONSE, ID_STATUS_STC_PONG };

use rustc_serialize::json;

mod response {

    use super::super::super::super::super::super::{get_version, MINECRAFT_PROTOCOL_VERSION};
    use super::super::super::super::super::super::universe::{MAX_PLAYERS};

    /// A response to be sent by the server list response packet
    #[derive(Debug, RustcDecodable, RustcEncodable)]
    pub struct Response {
    
        /// The player data contained in this response
        pub players: Players,
        
        /// The version data contained in this response
        pub version: Version,
        
        /// The description contained in this response
        pub description: Description
    
    }
    
    impl Response {
    
        /// Constructs a new Response
        pub fn construct() -> Response {
        
            Response { players: Players::construct(), version: Version::construct(), description: Description::construct() }
            
        }
    
    }
    
    /// A struct defining the players portion of the StatusResponsePacket
    #[derive(Debug, RustcDecodable, RustcEncodable)]
    pub struct Players {
        
        /// The maximum number of players allowed on this server
        pub max: i32, //Yes, I know. But Java only supports signed integers
        
        /// The current number of online players
        pub online: i32
        
    }
    
    impl Players {
    
        /// Constructs a new Players struct
        ///
        /// TODO: Make this use real data for online
        pub fn construct() -> Players {
        
            Players { max: MAX_PLAYERS, online: 0 }
        
        }
    
    }
    
    /// The version part of the response packet
    #[derive(Debug, RustcDecodable, RustcEncodable)]
    pub struct Version {
    
        /// The name of the version (This may or may not be ignored)
        pub name: String,
        
        /// The Minecraft protocol number being used
        pub protocol: u32
    
    }
    
    impl Version {
    
        /// Constructs a new Version struct
        ///
        /// TODO: Make name use a defined value
        pub fn construct() -> Version {
        
            Version { name: "1.8.8".to_string(), protocol: MINECRAFT_PROTOCOL_VERSION }
        
        }
    
    }
    
    /// The description part of the response packet
    #[derive(Debug, RustcDecodable, RustcEncodable)]
    pub struct Description {
        
        /// The text to display
        pub text: String
    
    }
    
    impl Description {
    
        /// Constructs a new Description.
        ///
        /// In the future, this will get the description from a (cached) file
        pub fn construct() -> Description {
            Description { text: format!("A Netherrack {} server", get_version()) }
        }
    
    }

}

/// A packet sent from the server to 
pub struct StatusResponsePacket;

impl StatusResponsePacket {

    /// Gets this StatusResponsePacket's ID
    pub fn get_id(&self) -> u32 {
    
        ID_STATUS_STC_RESPONSE
    
    }
    
    /// Gets the JSON response
    fn get_json_response(&self) -> String {
        json::encode(&response::Response::construct()).unwrap()
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
        
        let mut length_buffer = DequeBuffer::new();
        
        length_buffer.write_unsigned_varint_32(data_buffer.remaining() as u32); //TODO: Try to get this as write_unsigned_varint_32_front
        
        let buffer: Vec<u8> = length_buffer.data.into_iter().chain(data_buffer.data.into_iter()).collect();
        
        let _buffer: &[u8] = &buffer[..];
        
        if connection.stream.write(_buffer).is_ok() {
            let _ = connection.stream.flush();
        }
        
    }

}

/// A packet sent from the server to the client responding to a ping
pub struct ListPongPacket {

    pub payload: i64

}

impl ListPongPacket {

    /// Gets this ListPongPacket's ID
    pub fn get_id(&self) -> u32 {
    
        ID_STATUS_STC_PONG
    
    }

    /// Creates a new ListPongPacket
    pub fn new(payload: i64) -> ListPongPacket {
    
        ListPongPacket { payload: payload }
        
    }
    
    /// Encodes and sends this ListPongPacket
    pub fn send(&self, connection: &mut GameConnection) {
        
        let mut data_buffer = DequeBuffer::new();
        
        data_buffer.write_unsigned_varint_32(self.get_id());
        
        data_buffer.write_signed_long(self.payload);
        
        let mut length_buffer = DequeBuffer::new();
        
        length_buffer.write_unsigned_varint_32(data_buffer.remaining() as u32); //TODO: Try to get this as write_unsigned_varint_32_front
        
        let buffer: Vec<u8> = length_buffer.data.into_iter().chain(data_buffer.data.into_iter()).collect();
        
        let _buffer: &[u8] = &buffer[..];
        
        if connection.stream.write(_buffer).is_ok() {
            let _ = connection.stream.flush();
        }
        
    }

}
