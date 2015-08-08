//! A Netherrack buffer backed by a VecDeque::<u8>

use std::collections::VecDeque;

/// Defines a DequeBuffer
///
/// At some stage in the future, this will be changed to inherit the trait NetherrackBuffer<(Something extending Read and Write), but for now, we'll do everything here
pub struct DequeBuffer {
    
    data: VecDeque<u8>
    
}

impl DequeBuffer {

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

}