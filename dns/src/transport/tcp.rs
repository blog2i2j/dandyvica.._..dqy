use std::{
    io::Write,
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

use log::debug;

use crate::error::DNSResult;

use super::{mode::TransportMode, Transporter};

pub struct TcpTransport {
    stream: TcpStream,
}

impl TcpTransport {
    pub fn new<A: ToSocketAddrs>(addr: A, timeout: Duration) -> DNSResult<Self> {
        let stream = TcpStream::connect(addr)?;

        stream.set_read_timeout(Some(timeout))?;
        stream.set_write_timeout(Some(timeout))?;

        debug!("created TCP socket to {}", stream.peer_addr()?);
        Ok(Self { stream: stream })
    }
}

impl Transporter for TcpTransport {
    fn send(&mut self, buffer: &[u8]) -> DNSResult<usize> {
        let sent = self.stream.write(buffer)?;
        self.stream.flush()?;
        Ok(sent)
    }

    fn recv(&mut self, buffer: &mut [u8]) -> DNSResult<usize> {
        // let mut bytes = 0usize;
        // let mut reader = BufReader::new(&self.stream);
        // while bytes < 2059 {
        //     let x = reader.read(buffer)?;
        //     println!("============> read {x}");
        //     bytes += x;

        // }

        // Ok(bytes)
        // //Ok(reader.read(buffer)?)
        <TcpTransport as Transporter>::tcp_read(&mut self.stream, buffer)
    }

    fn uses_leading_length(&self) -> bool {
        true
    }

    fn mode(&self) -> TransportMode {
        TransportMode::Tcp
    }
}
