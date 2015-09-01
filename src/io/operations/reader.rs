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
                    return Err("Could not read an unsigned BE short")
                }
            },
            Err(error) => {
                error!("Got an error while reading an unsigned BE short! {}", error);
                return Err("Error while reading an unsigned BE short, check log output");
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
                    return Err("Could not read an unsigned LE short")
                }
            },
            Err(error) => {
                error!("Got an error while reading an unsigned LE short! {}", error);
                return Err("Error while reading an unsigned LE short, check log output");
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
                    return Err("Could not read a signed BE short")
                }
            },
            Err(error) => {
                error!("Got an error while reading a signed BE short! {}", error);
                return Err("Error while reading a signed BE short, check log output");
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
                    return Err("Could not read a signed LE short")
                }
            },
            Err(error) => {
                error!("Got an error while reading a signed LE short! {}", error);
                return Err("Error while reading a signed LE short, check log output");
            }

        }

    }

    /// Reads an unsigned big-endian integer from this Reader
    fn read_unsigned_be_int(&mut self) -> Result<u32, &'static str> {

        let mut raw_buffer = vec![0u8; 4];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 4 {
                    return Ok(
                        ((raw_buffer[0] as u32) << 24) |
                        ((raw_buffer[1] as u32) << 16) |
                        ((raw_buffer[2] as u32) << 8) |
                        raw_buffer[3] as u32
                    );
                } else {
                    return Err("Could not read an unsigned BE int")
                }
            },
            Err(error) => {
                error!("Got an error while reading an unsigned BE int! {}", error);
                return Err("Error while reading an unsigned BE int, check log output");
            }

        }

    }

    /// Reads an unsigned little-endian integer from this Reader
    fn read_unsigned_le_int(&mut self) -> Result<u32, &'static str> {

        let mut raw_buffer = vec![0u8; 4];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 4 {
                    return Ok(
                        raw_buffer[0] as u32 |
                        ((raw_buffer[1] as u32) << 8) |
                        ((raw_buffer[2] as u32) << 16) |
                        ((raw_buffer[3] as u32) << 24)
                    );
                } else {
                    return Err("Could not read an unsigned LE int")
                }
            },
            Err(error) => {
                error!("Got an error while reading an unsigned LE int! {}", error);
                return Err("Error while reading an unsigned LE int, check log output");
            }

        }

    }

    /// Reads a signed big-endian integer from this Reader
    fn read_signed_be_int(&mut self) -> Result<i32, &'static str> {

        let mut raw_buffer = vec![0u8; 4];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 4 {
                    return Ok(
                        ((raw_buffer[0] as i32) << 24) |
                        ((raw_buffer[1] as i32) << 16) |
                        ((raw_buffer[2] as i32) << 8) |
                        raw_buffer[3] as i32
                    );
                } else {
                    return Err("Could not read a signed BE int")
                }
            },
            Err(error) => {
                error!("Got an error while reading a signed BE int! {}", error);
                return Err("Error while reading a signed BE int, check log output");
            }

        }

    }

    /// Reads a signed little-endian integer from this Reader
    fn read_signed_le_int(&mut self) -> Result<i32, &'static str> {

        let mut raw_buffer = vec![0u8; 4];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 4 {
                    return Ok(
                        raw_buffer[0] as i32 |
                        ((raw_buffer[1] as i32) << 8) |
                        ((raw_buffer[2] as i32) << 16) |
                        ((raw_buffer[3] as i32) << 24)
                    );
                } else {
                    return Err("Could not read a signed LE int")
                }
            },
            Err(error) => {
                error!("Got an error while reading a signed LE int! {}", error);
                return Err("Error while reading a signed LE int, check log output");
            }

        }

    }

    /// Reads an unsigned big-endian long from this Reader
    fn read_unsigned_be_long(&mut self) -> Result<u64, &'static str> {

        let mut raw_buffer = vec![0u8; 8];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 8 {
                    return Ok(
                        ((raw_buffer[0] as u64) << 56) |
                        ((raw_buffer[1] as u64) << 48) |
                        ((raw_buffer[2] as u64) << 40) |
                        ((raw_buffer[3] as u64) << 32) |
                        ((raw_buffer[4] as u64) << 24) |
                        ((raw_buffer[5] as u64) << 16) |
                        ((raw_buffer[6] as u64) << 8) |
                        raw_buffer[7] as u64
                    );
                } else {
                    return Err("Could not read an unsigned BE long")
                }
            },
            Err(error) => {
                error!("Got an error while reading an unsigned BE long! {}", error);
                return Err("Error while reading an unsigned BE long, check log output");
            }

        }

    }

    /// Reads a signed big-endian long from this Reader
    fn read_signed_be_long(&mut self) -> Result<i64, &'static str> {

        let mut raw_buffer = vec![0u8; 8];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 8 {
                    return Ok(
                        ((raw_buffer[0] as i64) << 56) |
                        ((raw_buffer[1] as i64) << 48) |
                        ((raw_buffer[2] as i64) << 40) |
                        ((raw_buffer[3] as i64) << 32) |
                        ((raw_buffer[4] as i64) << 24) |
                        ((raw_buffer[5] as i64) << 16) |
                        ((raw_buffer[6] as i64) << 8) |
                        raw_buffer[7] as i64
                    );
                } else {
                    return Err("Could not read a signed BE long")
                }
            },
            Err(error) => {
                error!("Got an error while reading a signed BE long! {}", error);
                return Err("Error while reading a signed BE long, check log output");
            }

        }

    }

    /// Reads an unsigned little-endian long from this Reader
    fn read_unsigned_le_long(&mut self) -> Result<u64, &'static str> {

        let mut raw_buffer = vec![0u8; 8];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 8 {
                    return Ok(
                        raw_buffer[0] as u64 |
                        ((raw_buffer[1] as u64) << 8) |
                        ((raw_buffer[2] as u64) << 16) |
                        ((raw_buffer[3] as u64) << 24) |
                        ((raw_buffer[4] as u64) << 32) |
                        ((raw_buffer[5] as u64) << 40) |
                        ((raw_buffer[6] as u64) << 48) |
                        ((raw_buffer[7] as u64) << 56)
                    );
                } else {
                    return Err("Could not read an unsigned LE long")
                }
            },
            Err(error) => {
                error!("Got an error while reading an unsigned LE long! {}", error);
                return Err("Error while reading an unsigned LE long, check log output");
            }

        }

    }

    /// Reads a signed little-endian long from this Reader
    fn read_signed_le_long(&mut self) -> Result<i64, &'static str> {

        let mut raw_buffer = vec![0u8; 8];

        match self.read(&mut raw_buffer) {

            Ok(count) => {
                if count == 8 {
                    return Ok(
                        raw_buffer[0] as i64 |
                        ((raw_buffer[1] as i64) << 8) |
                        ((raw_buffer[2] as i64) << 16) |
                        ((raw_buffer[3] as i64) << 24) |
                        ((raw_buffer[4] as i64) << 32) |
                        ((raw_buffer[5] as i64) << 40) |
                        ((raw_buffer[6] as i64) << 48) |
                        ((raw_buffer[7] as i64) << 56)
                    );
                } else {
                    return Err("Could not read a signed LE long")
                }
            },
            Err(error) => {
                error!("Got an error while reading a signed LE long! {}", error);
                return Err("Error while reading a signed LE long, check log output");
            }

        }

    }
}

impl Reader for ::std::io::Cursor<Vec<u8>> { }
impl Reader for ::std::net::TcpStream { }
