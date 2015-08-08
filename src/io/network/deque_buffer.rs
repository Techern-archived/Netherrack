//! A Netherrack buffer backed by a VecDeque::<u8>

use std::collections::VecDeque;

extern crate bit_utils;
use self::bit_utils::BitInformation;

/// Defines a DequeBuffer
///
/// At some stage in the future, this will be changed to inherit the trait NetherrackBuffer<(Something extending Read and Write), but for now, we'll do everything here
pub struct DequeBuffer {
    
    data: VecDeque<u8>
    
}

impl DequeBuffer {

    /// Creates a DequeBuffer from a VecDeque
    ///
    /// After this method, we own the VecDeque
    pub fn from(deque: VecDeque<u8>) -> DequeBuffer {
        DequeBuffer { data: deque }
    }

    /// Gets the remaining bytes in this DequeBuffer
    pub fn remaining(&mut self) -> usize {
        self.data.len()
    }

    /// Reads an unsigned byte from the buffer, returning 0 if no data is available
    pub fn read_unsigned_byte(&mut self) -> u8 {
        match self.data.pop_front() {
            Some(value) => value,
            None => 0
        }
    }
    
    /// Reads an unsigned byte from the buffer, returning -127 if no data is available
    pub fn read_signed_byte(&mut self) -> i8 {
        match self.data.pop_front() {
            Some(value) => value as i8,
            None => -127
        }
    }
    
    /// Writes an unsigned byte to the buffer
    pub fn write_unsigned_byte(&mut self, value: u8) {
        self.data.push_back(value);
    }
    
    /// Writes a signed byte to the buffer
    pub fn write_signed_byte(&mut self, value: i8) {
        self.data.push_back(value as u8);
    }
    
    /// Reads an unsigned Varint from the buffer
    pub fn read_unsigned_varint_32(&mut self) -> Result<u32, &'static str> {
        if self.remaining() == 0 {
            return Err("Cannot read a Varint with no data to read.");
        } else {
            // The number of bits to shift by. <<0, <<7, <<14, etc
            let mut shift_amount: u32 = 0;
            
            // The decoded value
            let mut decoded_value: u32 = 0;
            
            while self.remaining() >= 1 {
            
                let byte_value: u8 = self.read_unsigned_byte(); //Yay for this existing!
                
                decoded_value |= ((byte_value & 0b01111111) as u32) << shift_amount; // We're not shifting for the first byte ;)
                
                // See if we're still reading bytes
                if byte_value.has_most_signifigant_bit() {
                    shift_amount += 7;
                } else {
                    return Ok(decoded_value);
                }
            
            }
            
            return Err("Reached end of buffer while reading a varint32");
            
        }
    }

}