//! Netherrack-specific writing I/O operations

use io_operations::writer::Writer;
//use std::io::Result;

pub trait NetherrackWriter : Writer {



}

impl NetherrackWriter for ::std::io::Cursor<Vec<u8>> { }
impl NetherrackWriter for ::std::net::TcpStream { }
