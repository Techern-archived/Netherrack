//! Reading I/O operations

use std::io::Read;

pub trait Reader : Read {

    /// Reads an unsigned byte from this Reader
    fn read_unsigned_byte(&mut self) -> Result<u8, &'static str> {

        let mut raw_buffer = vec![0u8; 1];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 1 {
                    return Ok(raw_buffer[0]);
                } else {
                    return Err("Could not read one byte")
                }
            },
            Err(error) => {
                error!("Got an error while reading a byte! {}", error);
                return Err("Error while reading one byte, check log output");
            }

        }

    }

    /// Reads a signed byte from this Reader
    fn read_signed_byte(&mut self) -> Result<i8, &'static str> {

        let mut raw_buffer = vec![0u8; 1];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 1 {
                    return Ok(raw_buffer[0] as i8);
                } else {
                    return Err("Could not read one byte")
                }
            },
            Err(error) => {
                error!("Got an error while reading a byte! {}", error);
                return Err("Error while reading one byte, check log output");
            }

        }

    }

    /// Reads an unsigned big-endian short from this Reader
    fn read_unsigned_be_short(&mut self) -> Result<u16, &'static str> {

        let mut raw_buffer = vec![0u8; 2];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 2 {
                    return Ok(
                        ((raw_buffer[0] as u16) << 8) |
                        raw_buffer[1] as u16
                    );
                } else {
                    return Err("Could not read one byte")
                }
            },
            Err(error) => {
                error!("Got an error while reading a byte! {}", error);
                return Err("Error while reading one byte, check log output");
            }

        }

    }

    /// Reads an unsigned little-endian short from this Reader
    fn read_unsigned_le_short(&mut self) -> Result<u16, &'static str> {

        let mut raw_buffer = vec![0u8; 2];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 2 {
                    return Ok(
                        raw_buffer[0] as u16 |
                        ((raw_buffer[1] as u16) << 8)
                    );
                } else {
                    return Err("Could not read one byte")
                }
            },
            Err(error) => {
                error!("Got an error while reading a byte! {}", error);
                return Err("Error while reading one byte, check log output");
            }

        }

    }

    /// Reads a signed big-endian short from this Reader
    fn read_signed_be_short(&mut self) -> Result<i16, &'static str> {

        let mut raw_buffer = vec![0u8; 2];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 2 {
                    return Ok(
                        ((raw_buffer[0] as i16) << 8) |
                        raw_buffer[1] as i16
                    );
                } else {
                    return Err("Could not read one byte")
                }
            },
            Err(error) => {
                error!("Got an error while reading a byte! {}", error);
                return Err("Error while reading one byte, check log output");
            }

        }

    }

    /// Reads a signed little-endian short from this Reader
    fn read_signed_le_short(&mut self) -> Result<i16, &'static str> {

        let mut raw_buffer = vec![0u8; 2];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 2 {
                    return Ok(
                        raw_buffer[0] as i16 |
                        ((raw_buffer[1] as i16) << 8)
                    );
                } else {
                    return Err("Could not read one byte")
                }
            },
            Err(error) => {
                error!("Got an error while reading a byte! {}", error);
                return Err("Error while reading one byte, check log output");
            }

        }

    }
}

impl Reader for ::std::io::Cursor<Vec<u8>> { }
impl Reader for ::std::net::TcpStream { }
