//! Functions relating to network packets

pub mod incoming;

pub use super::game_connection::GameConnection;

/// A struct defining the header of a Packet
#[derive(Clone, Debug)]
pub struct PacketHeader {
    
    /// The number of bytes contained in the ID and the packet data
    pub length: u32,
    
    /// The ID of this packet
    pub id: u32
    
}

/// A trait for all types that can represent packets
pub trait Packet {

    /// Gets this (Packet)'s header
    fn get_header(&self) -> PacketHeader;
    
    /// Gets the connection to the client
    fn get_connection(&self) -> GameConnection;

}

/// A trait for all incoming packets
pub trait IncomingPacket {

    /// Sets the header of this (IncomingPacket)
    fn set_header(&self, header: PacketHeader);
    
    /// Decodes this (IncomingPacket)
    fn decode(&self);

}

/// The ID of a handshake packet
pub const ID_HANDSHAKE_CTS_HANDSHAKING: u8 = 0x00;

/// The ID of a legacy server list ping packet
pub const ID_HANDSHAKE_CTS_LEGACY_PING: u8 = 0xFE;

/// The ID of a keep-alive packet
///
/// One of these must be sent periodically, and if the client does not receive one at least every 30 seconds, it will disconnect
pub const ID_PLAY_STC_KEEP_ALIVE: u8 = 0x00;

/// The ID of a "join game" packet packet
pub const ID_PLAY_STC_JOIN_GAME: u8 = 0x01;

/// The ID of a chat message packet
pub const ID_PLAY_STC_CHAT_MESSAGE: u8 = 0x02;

/// The ID of a time update packet
pub const ID_PLAY_STC_TIME_UPDATE: u8 = 0x03;

/// The ID of a entity equipment packet
pub const ID_PLAY_STC_ENTITY_EQUIPMENT: u8 = 0x04;

/// The ID of a spawn position packet
pub const ID_PLAY_STC_SPAWN_POSITION: u8 = 0x05;

/// The ID of a health update packet
pub const ID_PLAY_STC_UPDATE_HEALTH: u8 = 0x06;

/// The ID of a respawn packet
pub const ID_PLAY_STC_RESPAWN: u8 = 0x07;

/// The ID of a player position **and** look packet
pub const ID_PLAY_STC_PLAYER_POSITION_AND_LOOK: u8 = 0x08;

/// The ID of a change held item packet
pub const ID_PLAY_STC_CHANGE_HELD_ITEM: u8 = 0x09;

/// The ID of a use bed packet
pub const ID_PLAY_STC_USE_BED: u8 = 0x0A;

/// The ID of an animation packet
pub const ID_PLAY_STC_ANIMATION: u8 = 0x0B;

/// The ID of a spawn player packet
pub const ID_PLAY_STC_SPAWN_PLAYER: u8 = 0x0C;

/// The ID of a collect item packet
pub const ID_PLAY_STC_COLLECT_ITEM: u8 = 0x0D;

/// The ID of a spawn object packet
pub const ID_PLAY_STC_SPAWN_OBJECT: u8 = 0x0E;

/// The ID of a spawn mob packet
pub const ID_PLAY_STC_SPAWN_MOB: u8 = 0x0F;

/// The ID of a spawn painting packet
pub const ID_PLAY_STC_SPAWN_PAINTING: u8 = 0x10;

/// The ID of a spawn experience orb packet
pub const ID_PLAY_STC_SPAWN_EXPERIENCE_ORB: u8 = 0x11;

/// The ID of an entity velocity packet
pub const ID_PLAY_STC_ENTITY_VELOCITY: u8 = 0x12;

/// The ID of a destroy entities packet
pub const ID_PLAY_STC_DESTROY_ENTITIES: u8 = 0x13;

/// The ID of an entity packet
pub const ID_PLAY_STC_ENTITY: u8 = 0x14;

/// The ID of an entity relative move packet
pub const ID_PLAY_STC_ENTITY_RELATIVE_MOVE: u8 = 0x15;

/// The ID of an entity look packet
pub const ID_PLAY_STC_ENTITY_LOOK: u8 = 0x16;

/// The ID of an entity look and relative move packet
pub const ID_PLAY_STC_ENTITY_LOOK_AND_RELATIVE_MOVE: u8 = 0x17;

/// The ID of an entity teleport packet
pub const ID_PLAY_STC_ENTITY_TELEPORT: u8 = 0x18;

/// The ID of an entity head look packet
pub const ID_PLAY_STC_HEAD_LOOK: u8 = 0x19;

/// The ID of an entity status packet
pub const ID_PLAY_STC_ENTITY_STATUS: u8 = 0x1A;

/// The ID of an attach entity packet
pub const ID_PLAY_STC_ATTACH_ENTITY: u8 = 0x1B;

/// The ID of an entity metadata packet
pub const ID_PLAY_STC_ENTITY_METADATA: u8 = 0x1C;

/// The ID of an entity effect packet
pub const ID_PLAY_STC_ENTITY_EFFECT: u8 = 0x1D;

/// The ID of a remove entity effect packet
pub const ID_PLAY_STC_REMOVE_ENTITY_EFFECT: u8 = 0x1E;

/// The ID of a set experience packet
pub const ID_PLAY_STC_SET_EXPERIENCE: u8 = 0x1F;

/// The ID of an entity properties packet
pub const ID_PLAY_STC_ENTITY_PROPERTIES: u8 = 0x20;

/// The ID of a chunk data packet
pub const ID_PLAY_STC_CHUNK_DATA: u8 = 0x21;

/// The ID of a multi block change packet
pub const ID_PLAY_STC_MULTI_BLOCK_CHANGE: u8 = 0x22;

/// The ID of a block change packet
pub const ID_PLAY_STC_BLOCK_CHANGE: u8 = 0x23;

/// The ID of a block action packet
pub const ID_PLAY_STC_BLOCK_ACTION: u8 = 0x24;

/// The ID of a block break animation packet
pub const ID_PLAY_STC_BLOCK_BREAK_ANIMATION: u8 = 0x25;

/// The ID of a map chunk bulk packet
pub const ID_PLAY_STC_MAP_CHUNK_BULK: u8 = 0x26;

/// The ID of an explosion packet
pub const ID_PLAY_STC_EXPLOSION: u8 = 0x27;

/// The ID of an effect packet
pub const ID_PLAY_STC_EFFECT: u8 = 0x28;

/// The ID of a sound effect packet
pub const ID_PLAY_STC_SOUND_EFFECT: u8 = 0x29;

/// The ID of a particle packet
pub const ID_PLAY_STC_PARTICLE: u8 = 0x2A;

/// The ID of a change game state packet
pub const ID_PLAY_STC_CHANGE_GAME_STATE: u8 = 0x2B;

/// The ID of a spawn global entity packet
pub const ID_PLAY_STC_SPAWN_GLOBAL_ENTITY: u8 = 0x2C;

/// The ID of an open window packet
pub const ID_PLAY_STC_OPEN_WINDOW: u8 = 0x2D;

/// The ID of a close window packet
pub const ID_PLAY_STC_CLOSE_WINDOW: u8 = 0x2E;

/// The ID of a set slot packet
pub const ID_PLAY_STC_SET_SLOT: u8 = 0x2F;

/// The ID of a window items packet
pub const ID_PLAY_STC_WINDOW_ITEMS: u8 = 0x30;

/// The ID of a window property packet
pub const ID_PLAY_STC_WINDOW_PROPERTY: u8 = 0x31;

/// The ID of a confirm transaction packet
pub const ID_PLAY_STC_CONFIRM_TRANSACTION: u8 = 0x32;

/// The ID of a sign update packet
pub const ID_PLAY_STC_SIGN_UPDATE: u8 = 0x33;

/// The ID of a maps packet
pub const ID_PLAY_STC_MAPS: u8 = 0x34;

/// The ID of a block entity update packet
pub const ID_PLAY_STC_BLOCK_ENTITY_UPDATE: u8 = 0x35;

/// The ID of an open sign editor packet
pub const ID_PLAY_STC_OPEN_SIGN_EDITOR: u8 = 0x36;

/// The ID of a statistics packet
pub const ID_PLAY_STC_STATISTICS: u8 = 0x37;

/// The ID of a player list item packet
pub const ID_PLAY_STC_PLAYER_LIST_ITEM: u8 = 0x38;

/// The ID of a player abilities packet
pub const ID_PLAY_STC_PLAYER_ABILITIES: u8 = 0x39;

/// The ID of a tab complete packet
pub const ID_PLAY_STC_TAB_COMPLETE: u8 = 0x3A;

/// The ID of a scoreboard objective packet
pub const ID_PLAY_STC_SCOREBOARD_OBJECTIVE: u8 = 0x3B;

/// The ID of an update score packet
pub const ID_PLAY_STC_UPDATE_SCORE: u8 = 0x3C;

/// The ID of a display scoreboard packet
pub const ID_PLAY_STC_DISPLAY_SCOREBOARD: u8 = 0x3D;

/// The ID of a teams packet
pub const ID_PLAY_STC_TEAMS: u8 = 0x3E;

/// The ID of a plugin message packet
pub const ID_PLAY_STC_PLUGIN_MESSAGE: u8 = 0x3F;

/// The ID of a disconnect packet
pub const ID_PLAY_STC_DISCONNECT: u8 = 0x40;

/// The ID of a server difficulty packet
pub const ID_PLAY_STC_SERVER_DIFFICULTY: u8 = 0x41;

/// The ID of a combat event packet
pub const ID_PLAY_STC_COMBAT_EVENT: u8 = 0x42;

/// The ID of a camera packet
pub const ID_PLAY_STC_CAMERA: u8 = 0x43;

/// The ID of a world border packet
pub const ID_PLAY_STC_WORLD_BORDER: u8 = 0x44;

/// The ID of a title packet
pub const ID_PLAY_STC_TITLE: u8 = 0x45;

/// The ID of a set compression packet
pub const ID_PLAY_STC_SET_COMPRESSION: u8 = 0x46;

/// The ID of a player list header/footer packet
pub const ID_PLAY_STC_PLAYER_LIST_HEADER_FOOTER: u8 = 0x47;

/// The ID of a send resource pack packet
pub const ID_PLAY_STC_SEND_RESOURCE_PACK: u8 = 0x48;

/// The ID of an update entity NBT packet
pub const ID_PLAY_STC_UPDATE_ENTITY_NBT: u8 = 0x49;

/// The ID of a keep alive packet
pub const ID_PLAY_CTS_KEEP_ALIVE: u8 = 0x00;

/// The ID of a chat message packet
pub const ID_PLAY_CTS_CHAT_MESSAGE: u8 = 0x01;

/// The ID of a use entity packet
pub const ID_PLAY_CTS_USE_ENTITY: u8 = 0x02;

/// The ID of a player packet
pub const ID_PLAY_CTS_PLAYER: u8 = 0x03;

/// The ID of a player position packet
pub const ID_PLAY_CTS_PLAYER_POSITION: u8 = 0x04;

/// The ID of a player look packet
pub const ID_PLAY_CTS_PLAYER_LOOK: u8 = 0x05;

/// The ID of a player position and look packet
pub const ID_PLAY_CTS_PLAYER_POSITION_AND_LOOK: u8 = 0x06;

/// The ID of a player digging packet
pub const ID_PLAY_CTS_PLAYER_DIGGING: u8 = 0x07;

/// The ID of a player block placement packet
pub const ID_PLAY_CTS_PLAYER_BLOCK_PLACEMENT: u8 = 0x08;

/// The ID of a change held item packet
pub const ID_PLAY_CTS_CHANGE_HELD_ITEM: u8 = 0x09;

/// The ID of an animation packet
pub const ID_PLAY_CTS_ANIMATION: u8 = 0x0A;

/// The ID of an entity action packet
pub const ID_PLAY_CTS_ENTITY_ACTION: u8 = 0x0B;

/// The ID of a steer vehicle packet
pub const ID_PLAY_CTS_STEER_VEHICLE: u8 = 0x0C;

/// The ID of a close window packet
pub const ID_PLAY_CTS_CLOSE_WINDOW: u8 = 0x0D;

/// The ID of a click window packet
pub const ID_PLAY_CTS_CLICK_WINDOW: u8 = 0x0E;

/// The ID of a confirm transaction packet
pub const ID_PLAY_CTS_CONFIRM_TRANSACTION: u8 = 0x0F;

/// The ID of a creative inventory action packet
pub const ID_PLAY_CTS_CREATIVE_INVENTORY_ACTION: u8 = 0x10;

/// The ID of an enchant item packet
pub const ID_PLAY_CTS_ENCHANT_ITEM: u8 = 0x11;

/// The ID of a sign update packet
pub const ID_PLAY_CTS_SIGN_UPDATE: u8 = 0x12;

/// The ID of a player abilities packet
pub const ID_PLAY_CTS_PLAYER_ABILITIES: u8 = 0x13;

/// The ID of a tab complete packet
pub const ID_PLAY_CTS_TAB_COMPLETE: u8 = 0x14;

/// The ID of a client settings packet
pub const ID_PLAY_CTS_CLIENT_SETTINGS: u8 = 0x15;

/// The ID of a client status packet
pub const ID_PLAY_CTS_CLIENT_STATUS: u8 = 0x16;

/// The ID of a plugin message packet
pub const ID_PLAY_CTS_PLUGIN_MESSAGE: u8 = 0x17;

/// The ID of a spectate packet
pub const ID_PLAY_CTS_SPECTATE: u8 = 0x18;

/// The ID of a resource pack status packet
pub const ID_PLAY_CTS_RESOURCE_PACK_STATUS: u8 = 0x19;

/// The ID of a response packet
pub const ID_STATUS_STC_RESPONSE: u8 = 0x00;

/// The ID of a pong packet
pub const ID_STATUS_STC_PONG: u8 = 0x01;

/// The ID of a request packet
pub const ID_STATUS_CTS_REQUEST: u8 = 0x00;

/// The ID of a ping packet
pub const ID_STATUS_CTS_PING: u8 = 0x01;

/// The ID of a disconnect packet
pub const ID_LOGIN_STC_DISCONNECT: u8 = 0x00;

/// The ID of an encryption request packet
pub const ID_LOGIN_STC_ENCRYPTION_REQUEST: u8 = 0x01;

/// The ID of a login success packet
pub const ID_LOGIN_STC_LOGIN_SUCCESS: u8 = 0x02;

/// The ID of a set compression packet
pub const ID_LOGIN_STC_SET_COMPRESSION: u8 = 0x03;

/// The ID of a start login packet
pub const ID_LOGIN_CTS_START_LOGIN: u8 = 0x00;

/// The ID of an encryption response packet
pub const ID_LOGIN_CTS_ENCRYPTION_RESPONSE: u8 = 0x01;
