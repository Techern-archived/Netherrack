//! Netherrack-specific I/O operations for Rust

pub mod reader;

pub mod writer;

#[cfg(test)]
mod test {

    use super::reader::NetherrackReader;
    use super::writer::NetherrackWriter;

    use std::io::Cursor;

    #[test]
    fn test_read_write_unsigned_varint_32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_unsigned_varint_32(15).is_ok());
        assert!(vector.write_unsigned_varint_32(0).is_ok());
        assert!(vector.write_unsigned_varint_32(2111111111).is_ok());
        assert!(vector.write_unsigned_varint_32(3463465).is_ok());

        vector.set_position(0);

        assert_eq!(15, vector.read_unsigned_varint_32().unwrap());
        assert_eq!(0, vector.read_unsigned_varint_32().unwrap());
        assert_eq!(2111111111, vector.read_unsigned_varint_32().unwrap());
        assert_eq!(3463465, vector.read_unsigned_varint_32().unwrap());


    }

}
