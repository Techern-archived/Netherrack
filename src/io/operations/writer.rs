//! Netherrack-specific writing I/O operations

use io_operations::writer::Writer;

use std::io::Result;

extern crate varint;

pub trait NetherrackWriter : Writer {

    /// Writes a signed varint 32 to this NetherrackWriter
    fn write_signed_varint_32(&mut self, value: i32) -> Result<()> {
        self.write_unsigned_varint_32(self::varint::zigzag_signed_int(value))
    }

    /// Writes an unsigned 32-bit Varint to this NetherrackWriter
    ///
    /// TODO: Currently, this returns the Result of the last byte written. We need to roll our own
    /// eventually
    fn write_unsigned_varint_32(&mut self, value: u32) -> Result<()> {

        let mut _value: u32 = value;

        if value == 0 {
            return self.write_unsigned_byte(0)
        } else {

            while _value >= 0b10000000 {

                let next_byte: u8 = ((_value & 0b01111111) as u8) | 0b10000000;

                _value = _value >> 7;

                let _ = self.write_unsigned_byte(next_byte);

            }

            return self.write_unsigned_byte((_value & 0b01111111) as u8);

        }

    }

}

impl NetherrackWriter for ::std::io::Cursor<Vec<u8>> { }
impl NetherrackWriter for ::std::net::TcpStream { }
