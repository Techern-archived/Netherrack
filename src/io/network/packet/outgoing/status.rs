//! Implements all outgoing status packets
/*
use super::super::super::deque_buffer::DequeBuffer;
use super::super::super::game_connection::GameConnection; //It's super super super effective!
use super::super::{ ID_STATUS_STC_RESPONSE, ID_STATUS_STC_PONG };

use super::OutgoingPacket;

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
        pub description: Description,

        /// The optional / Forge-only mod information
        pub modinfo: Option<ModInfo>

    }

    impl Response {

        /// Constructs a new Response
        pub fn construct() -> Response {

            return Response { players: Players::construct(), version: Version::construct(), description: Description::construct(), modinfo: Some(ModInfo::construct())};

        }

    }

    /// A struct containing mod (forge) information
    #[derive(Debug, RustcDecodable, RustcEncodable)]
    #[allow(non_snake_case)]
    pub struct ModInfo {

        /// The type of mod loader compatibility being used
        pub _type: String,

        /// The list of mods currently in use on the server
        pub modList: Vec<ModDescriptor>

    }

    impl ModInfo {

        /// Constructs a new ModInfo struct
        pub fn construct() -> ModInfo {

            ModInfo { _type: "FML".to_string(), modList: ModDescriptor::construct() }

        }

    }

    /// A struct containing mod information
    #[derive(Debug, RustcDecodable, RustcEncodable)]
    pub struct ModDescriptor {

        /// The mod's ID
        pub modid: String,

        /// The mod's version
        pub version: String

    }

    impl ModDescriptor {

        /// Constructs a new list of ModDescriptors
        pub fn construct() -> Vec<ModDescriptor> {

            //TODO: One day, replace this with names of mods added
            vec!(
                ModDescriptor { modid: "ExampleMod".to_string(), version: "0.0.1-pre".to_string() }
            )

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

    /// Creates a new Packet
    pub fn new() -> StatusResponsePacket {

        StatusResponsePacket

    }

}

impl OutgoingPacket for StatusResponsePacket {

    /// Gets this StatusResponsePacket's ID
    ///
    /// One day, this will be replaced with an associated constant
    fn get_id(&self) -> u32 {
        ID_STATUS_STC_RESPONSE
    }

    /// Encodes this StatusResponsePacket's data
    fn encode_data(&self, buffer: &mut DequeBuffer) {
        buffer.write_utf8_string(
            json::encode(&response::Response::construct()).unwrap().replace("_type", "type")
        );
    }

}

/// A packet sent from the server to the client responding to a ping
pub struct ListPongPacket {

    pub payload: i64

}

impl ListPongPacket {

    /// Creates a new ListPongPacket
    pub fn new(payload: i64) -> ListPongPacket {

        ListPongPacket { payload: payload }

    }
}

impl OutgoingPacket for ListPongPacket {

    /// Gets this ListPongPacket's ID
    fn get_id(&self) -> u32 {

        ID_STATUS_STC_PONG

    }

    /// Encodes this Packet's data
    fn encode_data(&self, buffer: &mut DequeBuffer) {
        buffer.write_signed_long(self.payload);
    }

    /// Called after the OutgoingPacket is sent
    fn post_send(&self, connection: &mut GameConnection) {
        // All ListPongPackets are at the end of any packet stream, so we can disconnect the client here
        connection.disconnect();
    }

}*/
