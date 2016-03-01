//! Utilities to do with connections between a Minecraft or Forge client and Netherrack

extern crate chrono;
use self::chrono::duration::Duration;

/*use super::packet::{ PacketHeader,
                     ID_HANDSHAKE_CTS_HANDSHAKING,
                     ID_STATUS_CTS_REQUEST, ID_STATUS_CTS_PING
                   };*/

use std::net::{Shutdown, TcpStream};

use ::io_operations::reader::Reader;

/// An enum describing the current connection's state
#[derive(PartialEq, Clone, Copy, Debug)]
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
#[derive(Debug)]
pub struct GameConnection {

    /// The TCP stream bound to this GameConnection
    pub stream: TcpStream,

    /// The state of this GameConnection
    pub state: ConnectionState,

    /// Whether the client has the Forge Mod Loader injected into it
    ///
    /// This may change in 1.9 as FML is being destroyed and Forge is becoming the norm
    pub forge_enabled: bool,

    /// A boolean declaring if this GameConnection is connected
    pub connected: bool

}

impl GameConnection {

    /// Creates a new GameConnection
    pub fn new(stream: TcpStream) -> GameConnection {
        GameConnection {
            stream: stream,
            state: ConnectionState::HANDSHAKE,
            forge_enabled: false,
            connected: true //HURR DURR
        }
    }

    pub fn disconnect(&mut self) {
        //TODO: Check to see if in ConnectionState::PLAY, otherwise
        let result = self.stream.shutdown(Shutdown::Both);

        if result.is_ok() == false {
            error!("Error disconnecting: {:?}", result);
        }

        self.connected = false;

        ::std::thread::yield_now();

    }

    /// Starts listening on the connection
    pub fn start_listening(&mut self) {

        trace!("Got a new client from {}!", self.stream.peer_addr().unwrap());

        let mut timeout_duration = Duration::zero();

        loop {

            //First, let's start off the timeout timer.
            timeout_duration = timeout_duration + Duration::milliseconds(2);
            ::std::thread::sleep(::std::time::Duration::from_millis(2));

            if timeout_duration.num_seconds() >= 1 {
                info!("Client at {} timed out, dropping connection", self.stream.peer_addr().unwrap());
                self.disconnect();
                break;
            }

            if !self.connected {
                debug!("Client at {} was forcibly disconnected, maybe for a good reason", self.stream.peer_addr().unwrap());
                break;
            }

            match self.stream.read_unsigned_byte() {
                Err(error) => {

                    if error != "Could not read one byte" {
                        error!("Error: {}", error);
                        self.disconnect();
                    }
                }

                Ok(value) => {
                    timeout_duration = Duration::zero();
                    info!("Value is {}", value);
                }
            }

        /*
            let mut raw_buffer = vec![0u8; 512];

            match self.stream.read(&mut raw_buffer) {

                Err(error) => {
                    // Connection dropped out or something broke
                    // In the future, we use the customized NetherrackStream to see if we're in GAME mode, and if so, get the attached player and start saving their information and get rid of them
                    // For now, we just exit the loop
                    error!("Got an error! {}", error);

                    self.disconnect();
                    break;
                }
                Ok(count) => {
                    if count == 0 {
                        //Nothing in stream, let's sleep for a few milliseconds and try again
                        timeout_duration = timeout_duration + Duration::milliseconds(2);
                        ::std::thread::sleep_ms(2);
                    } else {

                        timeout_duration = Duration::zero();

                        // Now truncate the raw buffer - Don't worry, it will reallocate to 512 on the next networking loop
                        raw_buffer.truncate(count);

                        let mut backing: VecDeque<u8> = VecDeque::<u8>::with_capacity(count);

                        for b in raw_buffer {

                            backing.push_back(b);
                        }

                        let mut buffer = DequeBuffer::from(backing);

                        //drop(raw_buffer);

                        let length_result = buffer.read_unsigned_varint_32();
                        let id_result = buffer.read_unsigned_varint_32();

                        if length_result.is_err() || id_result.is_err() {
                            error!("Error while reading packet ID or length! Oh no, dropping connection");

                            self.disconnect();
                            break;
                        }

                        let packet_header = PacketHeader::new(length_result.unwrap(), id_result.unwrap());

                        if self.state == ConnectionState::HANDSHAKE {

                            match packet_header.id {

                                ID_HANDSHAKE_CTS_HANDSHAKING => {
                                    super::packet::incoming::handshake::HandshakePacket::decode(packet_header, self, &mut buffer).handle(self);
                                },
                                _ => debug!("Incoming handshake packet unhandled; ID: {}, Length: {}", packet_header.id, packet_header.length)

                            }

                        } else if self.state == ConnectionState::LOGIN {

                            match packet_header.id {

                                _ => debug!("Incoming login packet unhandled; ID: {}, Length: {}", packet_header.id, packet_header.length)

                            }

                        } else if self.state == ConnectionState::STATUS {

                            match packet_header.id {

                                ID_STATUS_CTS_REQUEST => {
                                    super::packet::incoming::status::StatusRequestPacket::decode(packet_header, self, &mut buffer).handle(self);
                                },
                                ID_STATUS_CTS_PING => {
                                    super::packet::incoming::status::ListPingPacket::decode(packet_header, self, &mut buffer).handle(self);
                                },
                                _ => debug!("Incoming play packet unhandled; ID: {}, Length: {}", packet_header.id, packet_header.length)

                            }

                        } else if self.state == ConnectionState::PLAY {

                            match packet_header.id {

                                _ => debug!("Incoming play packet unhandled; ID: {}, Length: {}", packet_header.id, packet_header.length)

                            }

                        }

                        // This is a memory leak. We need to get the time of last packet and check to see if it's more than 20 seconds ago
                    }
                }

            }*/

        }

    }

}
