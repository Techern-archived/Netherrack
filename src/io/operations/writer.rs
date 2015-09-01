//! Writing I/O operations

use std::io::Write;
use std::io::Result;

pub trait Writer : Write {

    /// Writes an unsigned byte to this Writer
    fn write_unsigned_byte(&mut self, value: u8) -> Result<()> {

        let raw_buffer = vec![value];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)

    }

    /// Writes a signed byte to this Writer
    fn write_signed_byte(&mut self, value: i8) -> Result<()> {

        let raw_buffer = vec![value as u8];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)

    }

    /// Writes an unsigned little-endian short to this Writer
    fn write_unsigned_le_short(&mut self, value: u16) -> Result<()> {
        let raw_buffer = vec![
            value as u8,
            (value >> 8) as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

    /// Writes an unsigned big-endian short to this Writer
    fn write_unsigned_be_short(&mut self, value: u16) -> Result<()> {
        let raw_buffer = vec![
            (value >> 8) as u8,
            value as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

    /// Writes a signed little-endian short to this Writer
    fn write_signed_le_short(&mut self, value: i16) -> Result<()> {
        let raw_buffer = vec![
            value as u8,
            (value >> 8) as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

    /// Writes a signed big-endian short to this Writer
    fn write_signed_be_short(&mut self, value: i16) -> Result<()> {
        let raw_buffer = vec![
            (value >> 8) as u8,
            value as u8
        ];

        // Reassign to a buffer of raw u8s
        let raw_buffer: &[u8] = &raw_buffer[..];

        self.write_all(raw_buffer)
    }

}

impl Writer for ::std::io::Cursor<Vec<u8>> { }
impl Writer for ::std::net::TcpStream { }