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

    #[test]
    fn test_read_write_be_u16() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_unsigned_be_short(64241).is_ok());
        assert!(vector.write_unsigned_be_short(4).is_ok());

        vector.set_position(0);

        assert_eq!(64241, vector.read_unsigned_be_short().unwrap());
        assert_eq!(4, vector.read_unsigned_be_short().unwrap());

    }

    #[test]
    fn test_read_write_le_u16() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_unsigned_le_short(64241).is_ok());
        assert!(vector.write_unsigned_le_short(4).is_ok());

        vector.set_position(0);

        assert_eq!(64241, vector.read_unsigned_le_short().unwrap());
        assert_eq!(4, vector.read_unsigned_le_short().unwrap());

    }

    #[test]
    fn test_read_write_be_i16() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_signed_be_short(-12234).is_ok());
        assert!(vector.write_signed_be_short(24524).is_ok());

        vector.set_position(0);

        assert_eq!(-12234, vector.read_signed_be_short().unwrap());
        assert_eq!(24524, vector.read_signed_be_short().unwrap());

    }

    #[test]
    fn test_read_write_le_i16() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_signed_le_short(-12234).is_ok());
        assert!(vector.write_signed_le_short(24524).is_ok());

        vector.set_position(0);

        assert_eq!(-12234, vector.read_signed_le_short().unwrap());
        assert_eq!(24524, vector.read_signed_le_short().unwrap());

    }

    #[test]
    fn test_read_write_be_u32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_unsigned_be_int(4000000000).is_ok());
        assert!(vector.write_unsigned_be_int(26436735).is_ok());

        vector.set_position(0);

        assert_eq!(4000000000, vector.read_unsigned_be_int().unwrap());
        assert_eq!(26436735, vector.read_unsigned_be_int().unwrap());

    }

    #[test]
    fn test_read_write_le_u32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_unsigned_le_int(4000000000).is_ok());
        assert!(vector.write_unsigned_le_int(26436735).is_ok());

        vector.set_position(0);

        assert_eq!(4000000000, vector.read_unsigned_le_int().unwrap());
        assert_eq!(26436735, vector.read_unsigned_le_int().unwrap());

    }

    #[test]
    fn test_read_write_be_i32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_signed_be_int(-2100000000).is_ok());
        assert!(vector.write_signed_be_int(26436735).is_ok());

        vector.set_position(0);

        assert_eq!(-2100000000, vector.read_signed_be_int().unwrap());
        assert_eq!(26436735, vector.read_signed_be_int().unwrap());

    }

    #[test]
    fn test_read_write_le_i32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_signed_le_int(-2100000000).is_ok());
        assert!(vector.write_signed_le_int(26436735).is_ok());

        vector.set_position(0);

        assert_eq!(-2100000000, vector.read_signed_le_int().unwrap());
        assert_eq!(26436735, vector.read_signed_le_int().unwrap());

    }

    #[test]
    fn test_read_write_be_u64() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_unsigned_be_long(35754238465284).is_ok());
        assert!(vector.write_unsigned_be_long(423).is_ok());

        vector.set_position(0);

        assert_eq!(35754238465284, vector.read_unsigned_be_long().unwrap());
        assert_eq!(423, vector.read_unsigned_be_long().unwrap());

    }

    #[test]
    fn test_read_write_le_u64() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_unsigned_le_long(35754238465284).is_ok());
        assert!(vector.write_unsigned_le_long(423).is_ok());

        vector.set_position(0);

        assert_eq!(35754238465284, vector.read_unsigned_le_long().unwrap());
        assert_eq!(423, vector.read_unsigned_le_long().unwrap());

    }

    #[test]
    fn test_read_write_be_i64() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_signed_be_long(-5754238465284).is_ok());
        assert!(vector.write_signed_be_long(423).is_ok());

        vector.set_position(0);

        assert_eq!(-5754238465284, vector.read_signed_be_long().unwrap());
        assert_eq!(423, vector.read_signed_be_long().unwrap());

    }

    #[test]
    fn test_read_write_le_i64() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_signed_le_long(-5754238465284).is_ok());
        assert!(vector.write_signed_le_long(423).is_ok());

        vector.set_position(0);

        assert_eq!(-5754238465284, vector.read_signed_le_long().unwrap());
        assert_eq!(423, vector.read_signed_le_long().unwrap());

    }

}
