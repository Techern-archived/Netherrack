//! Netherrack-specific reading I/O operations

use io_operations::reader::Reader;

extern crate varint;

extern crate bit_utils;
use self::bit_utils::BitInformation;

pub trait NetherrackReader : Reader {

    /// Reads a UTF-8 string from the buffer
    #[allow(unused_variables)] //Only for error stuff
    fn read_utf8_string(&mut self) -> Result<String, &'static str> {

        match self.read_unsigned_varint_32() {

            Err(error) => {
                return Err("Bad size for utf-8 string");
            }

            Ok(mut size) => {

                let mut bytes: Vec<u8> = Vec::<u8>::with_capacity(size as usize);

                while size > 0 {
                    match self.read_unsigned_byte() {
                        Err(error) => {
                            return Err("Bad things while reading a UTF-8 string");
                        }

                        Ok(value) => {
                            bytes.push(value);
                            size -= 1;
                        }
                    }
                }

                match String::from_utf8(bytes) {

                    Err(errval) => {
                        return Err("Error parsing UTF-8 String");
                    }

                    Ok(string) => {
                        return Ok(string);
                    }

                }

            }

        }

    }

    /// Reads a signed 32-bit Varint from this NetherrackReader
    #[allow(unused_variables)] //For the error handling as we need to change the string
    fn read_signed_varint_32(&mut self) -> Result<i32, &'static str> {
        match self.read_unsigned_varint_32() {
            Ok(value) => {
                return Ok(self::varint::zigzag_unsigned_int(value));
            }

            Err(error) => {
                return Err("Could not read a signed varint32");
            }
        }
    }

    /// Reads an unsigned 32-bit Varint from this NetherrackReader
    fn read_unsigned_varint_32(&mut self) -> Result<u32, &'static str> {

        // The number of bits to shift by. <<0, <<7, <<14, etc
        let mut shift_amount: u32 = 0;

        // The decoded value
        let mut decoded_value: u32 = 0;

        loop {

            match self.read_unsigned_byte() {
                Err(error) => {
                    error!("Got an error while reading a byte for an unsigned varint32: {}", error);
                    return Err("Could not read an unsigned byte for an unsigned varint32");
                }

                Ok(byte_value) => {
                    decoded_value |= ((byte_value & 0b01111111) as u32) << shift_amount;

                    // See if we're supposed to keep reading
                    if byte_value.has_most_signifigant_bit() {
                        shift_amount += 7;
                    } else {
                        return Ok(decoded_value);
                    }
                }
            }

        }

    }

}

impl NetherrackReader for ::std::io::Cursor<Vec<u8>> { }
impl NetherrackReader for ::std::net::TcpStream { }
