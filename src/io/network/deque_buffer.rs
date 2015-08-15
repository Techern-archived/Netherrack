//! A Netherrack buffer backed by a VecDeque\<u8\>

use std::collections::VecDeque;

extern crate varint;

extern crate bit_utils;
use self::bit_utils::BitInformation;

/// A buffer that is backed by a VecDeque\<u8\>
pub struct DequeBuffer {
    
    pub data: VecDeque<u8>
    
}

impl DequeBuffer {

    /// Creates a DequeBuffer from a VecDeque
    ///
    /// After this method, we own the VecDeque
    pub fn from(deque: VecDeque<u8>) -> DequeBuffer {
        DequeBuffer { data: deque }
    }
    
    /// Creates a new empty DequeBuffer
    pub fn new() -> DequeBuffer {
        DequeBuffer { data: VecDeque::<u8>::new() }
    }
    
    /// Creates a new empty DequeBuffer with the specified initial capacity
    pub fn with_capacity(capacity: usize) -> DequeBuffer {
        DequeBuffer { data: VecDeque::<u8>::with_capacity(capacity) }
    }

    /// Gets the remaining bytes in this DequeBuffer
    pub fn remaining(&self) -> usize {
        self.data.len()
    }
    
    /// Reads a UTF-8 string from the buffer
    pub fn read_utf8_string(&mut self) -> String {
        let size_result = self.read_unsigned_varint_32();
        
        if size_result.is_err() {
            return String::from("!!!BAD_SIZE!!!");
        } else {
            let mut size: u32 = size_result.unwrap();
            
            let mut bytes: Vec<u8> = Vec::<u8>::with_capacity(size as usize);
            
            while size > 0 {
                bytes.push(self.read_unsigned_byte());
                size -= 1;
            }
            
            let result = String::from_utf8(bytes);
            
            if result.is_ok() {
                return result.unwrap();
            } else {
                return String::from("!!!INVALID_UTF8!!!");
            }
            
        }
    }
    
    /// Writes a UTF-8 string into the buffer
    pub fn write_utf8_string(&mut self, value: String) {
    
        let mut bytes = value.as_bytes();
                
        self.write_unsigned_varint_32(bytes.len() as u32);
        
        info!("Length is {}", bytes.len() as u32);
        
        for &byte in bytes {
            self.write_unsigned_byte(byte);
        }
    
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
    
    /// Reads an unsigned short from the buffer
    pub fn read_unsigned_short(&mut self) -> u16 {
        
        let high: u8 = self.read_unsigned_byte();
        let low: u8 = self.read_unsigned_byte();
        
        ((high as u16) << 8) | low as u16
        
    }
    
    /// Writes an unsigned byte to the buffer
    pub fn write_unsigned_byte(&mut self, value: u8) {
        self.data.push_back(value);
    }
    
    /// Writes a signed byte to the buffer
    pub fn write_signed_byte(&mut self, value: i8) {
        self.data.push_back(value as u8);
    }
    
    /// Writes a signed varint 32 to the buffer
    pub fn write_signed_varint_32(&mut self, value: i32) {
        
        self.write_unsigned_varint_32(self::varint::zigzag_signed_int(value));
        
    }
    
    /// Writes an unsigned 32-bit Varint from the buffer
    pub fn write_unsigned_varint_32(&mut self, value: u32) {
    
        let mut _value: u32 = value;
    
        if value == 0 {
            self.write_unsigned_byte(0);
        } else {
            
            while _value >= 0b10000000 {
            
                debug!("Value is {}, writing {}", (_value & 0xff) as u8, _value as u8);
                
                let next_byte: u8 = ((_value & 0b01111111) as u8) | 0b10000000;
                
                _value = _value >> 7;
                
                self.write_unsigned_byte(next_byte);
                
            }
            
            debug!("Writing last byte of value: {} is {}", _value, _value & 0b01111111);
            self.write_unsigned_byte((_value & 0b01111111) as u8);
            
        }
    
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_contains_no_elements() {
        let new = DequeBuffer::new();
        
        assert_eq!(0, new.remaining());
    }

    #[test]
    fn new_with_capacity_contains_no_elements() {
        let new = DequeBuffer::with_capacity(25000); //WOW, MUCH BIG PACKET
        
        assert_eq!(0, new.remaining());
    }
    
    #[test]
    fn unsigned_byte() {
        let mut buffer = DequeBuffer::with_capacity(2);
        
        buffer.write_unsigned_byte(1);
        buffer.write_unsigned_byte(15);
        
        assert_eq!(2, buffer.remaining());
        
        assert_eq!(1, buffer.read_unsigned_byte());
        assert_eq!(15, buffer.read_unsigned_byte());
    }
    
    #[test]
    fn signed_byte() {
        let mut buffer = DequeBuffer::with_capacity(2);
        
        buffer.write_signed_byte(-50);
        buffer.write_signed_byte(0);
        
        assert_eq!(2, buffer.remaining());
        
        assert_eq!(-50, buffer.read_signed_byte());
        assert_eq!(-0, buffer.read_signed_byte());
    }
    
    #[test]
    fn unsigned_varint_32() {
        
        let mut buffer = DequeBuffer::new();
        
        let var1: u32 = 0;
        
        let var2: u32 = 4003254234;
        
        let var3: u32 = 200;
        
        buffer.write_unsigned_varint_32(var1);
        buffer.write_unsigned_varint_32(var2);
        buffer.write_unsigned_varint_32(var3);
        
        assert_eq!(8, buffer.remaining());
        
        // No need to check for okay here, we know it will be. If you're getting it from the client, always check!
        assert_eq!(var1, buffer.read_unsigned_varint_32().unwrap());
        assert_eq!(var2, buffer.read_unsigned_varint_32().unwrap());
        assert_eq!(var3, buffer.read_unsigned_varint_32().unwrap());
        
    }

}