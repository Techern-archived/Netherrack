//! Netherrack-specific reading I/O operations

use io_operations::reader::Reader;

pub trait NetherrackReader : Reader {

}

impl NetherrackReader for ::std::io::Cursor<Vec<u8>> { }
impl NetherrackReader for ::std::net::TcpStream { }
