//! Netherrack-specific I/O operations for Rust

pub mod reader;

pub mod writer;

#[cfg(test)]
mod test {

    use super::reader::NetherrackReader;
    use super::writer::NetherrackWriter;

    use io_operations::writer::Writer;
    use io_operations::reader::Reader;


    use std::io::{ Read, Cursor };

    #[test]
    fn test_read_write_cursor_changes() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_utf8_string("Hello, cruel world!".to_string()).is_ok());

        assert!(vector.write_unsigned_byte(13).is_ok());

        let bytes_written = vector.position() as u32;

        let mut new_vector = Cursor::new(vec![0u8; 0]);

        assert!(new_vector.write_unsigned_varint_32(bytes_written).is_ok());

        vector.set_position(0);

        vector.read_to_end(new_vector.get_mut());

        new_vector.set_position(0);

        assert_eq!(bytes_written, new_vector.read_unsigned_varint_32().unwrap());

        assert_eq!("Hello, cruel world!", new_vector.read_utf8_string().unwrap());

        assert_eq!(13, new_vector.read_unsigned_byte().unwrap());

    }

    #[test]
    fn test_read_write_utf8_string() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_utf8_string("Hello, world!".to_string()).is_ok());
        assert!(vector.write_utf8_string("".to_string()).is_ok());
        assert!(vector.write_utf8_string("null".to_string()).is_ok());
        assert!(vector.write_utf8_string("0".to_string()).is_ok());
        assert!(vector.write_utf8_string("And that is why this really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, long string is the Prince of Bel-Air".to_string()).is_ok());

        vector.set_position(0);

        assert_eq!("Hello, world!", vector.read_utf8_string().unwrap());
        assert_eq!("", vector.read_utf8_string().unwrap());
        assert_eq!("null", vector.read_utf8_string().unwrap());
        assert_eq!("0", vector.read_utf8_string().unwrap());
        assert_eq!("And that is why this really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, really, long string is the Prince of Bel-Air", vector.read_utf8_string().unwrap());

    }

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

    #[test]
    fn test_read_write_signed_varint_32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_signed_varint_32(-4).is_ok());
        assert!(vector.write_signed_varint_32(0).is_ok());
        assert!(vector.write_signed_varint_32(2111111111).is_ok());
        assert!(vector.write_signed_varint_32(-2111111111).is_ok());
        assert!(vector.write_signed_varint_32(-3463465).is_ok());

        vector.set_position(0);

        assert_eq!(-4, vector.read_signed_varint_32().unwrap());
        assert_eq!(-0, vector.read_signed_varint_32().unwrap());
        assert_eq!(2111111111, vector.read_signed_varint_32().unwrap());
        assert_eq!(-2111111111, vector.read_signed_varint_32().unwrap());
        assert_eq!(-3463465, vector.read_signed_varint_32().unwrap());


    }

    #[test]
    fn test_read_write_lots_of_unsigned_varint_32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        let mut index = 0;

        while index < 123456 {
            index += 1;

            assert!(vector.write_unsigned_varint_32(index).is_ok());
        }

        vector.set_position(0);
        index = 0;

        while index < 123456 {
            index += 1;

            assert_eq!(index, vector.read_unsigned_varint_32().unwrap());
        }

    }

}
