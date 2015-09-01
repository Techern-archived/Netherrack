//! I/O operations for Rust
//!
//! To be completed

pub mod reader;

pub mod writer;

#[cfg(test)]
mod test {

    use super::reader::Reader;
    use super::writer::Writer;

    use std::io::Cursor;

    #[test]
    fn test_read_write_u8() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_unsigned_byte(15).is_ok());
        assert!(vector.write_unsigned_byte(62).is_ok());

        vector.set_position(0);

        assert_eq!(15, vector.read_unsigned_byte().unwrap());
        assert_eq!(62, vector.read_unsigned_byte().unwrap());

    }

    #[test]
    fn test_read_write_i8() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_signed_byte(-43).is_ok());
        assert!(vector.write_signed_byte(33).is_ok());

        vector.set_position(0);

        assert_eq!(-43, vector.read_signed_byte().unwrap());
        assert_eq!(33, vector.read_signed_byte().unwrap());

    }

}
