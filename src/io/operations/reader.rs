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
}

impl Reader for ::std::io::Cursor<Vec<u8>> { }
