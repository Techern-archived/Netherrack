//! Utilities to do with connections between a Minecraft or Forge client and Netherrack

pub use std::net::TcpStream;

/// An enum describing the current connection's state
pub enum ConnectionState {
    
    /// Connection is in handshake mode
    HANDSHAKE,
    
    /// Connection is in login mode
    LOGIN,
    
    /// Connection is in status mode
    STATUS,
    
    /// Connection is in play mode
    PLAY
    
}

/// A struct defining a game connection
pub struct GameConnection {

    /// The TCP stream bound to this GameConnection
    pub stream: TcpStream,
    
    /// The state of this GameConnection
    pub state: ConnectionState

}

/// Creates a new GameConnection
pub fn new(stream: TcpStream) -> GameConnection {
    GameConnection {
        stream: stream,
        state: ConnectionState::HANDSHAKE
    }
}
