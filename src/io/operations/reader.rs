//! Netherrack-specific reading I/O operations

use io_operations::reader::Reader;

extern crate varint;

extern crate bit_utils;
use self::bit_utils::BitInformation;

pub trait NetherrackReader : Reader {

    /// Reads an unsigned 32-bit Varint from this NetherrackReader
    fn read_unsigned_varint_32(&mut self) -> Result<u32, &'static str> {

        // The number of bits to shift by. <<0, <<7, <<14, etc
        let mut shift_amount: u32 = 0;

        // The decoded value
        let mut decoded_value: u32 = 0;

        loop {

            let byte_value = self.read_unsigned_byte();

            if byte_value.is_err() {
                return Err("Byte in read_unsigned_varint_32 returned an error");
            } else {
                let byte_value: u8 = byte_value.unwrap();

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

impl NetherrackReader for ::std::io::Cursor<Vec<u8>> { }
impl NetherrackReader for ::std::net::TcpStream { }
