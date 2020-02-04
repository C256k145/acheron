use std::net::UdpSocket;
use std::io;

pub struct Listener<'a> {
    local_socket: &'a UdpSocket,
    len: usize, 
    buf: Vec<u8>,
}

impl<'a> Listener<'a> {
    pub fn new(socket_addr: &'a UdpSocket, buf_len: usize) -> Self {
        Listener {
            local_socket: socket_addr,
            len: buf_len,
            buf: vec![0; buf_len],
        }
    }

    pub fn print_msg(&self) {
        let msg = String::from_utf8_lossy(&self.buf);
        println!("{}", msg);
    }

    pub fn listen(&mut self) -> std::io::Result<bool> {
        self.buf = vec![0; self.len];
        let res = self.local_socket.recv_from(&mut self.buf);
        match  res {
            Ok(_) => return Ok(true),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                return Err(e);
            },
            Err(e) => panic!("encountered IO error: {}", e),
        }
    }
}