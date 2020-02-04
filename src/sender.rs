use std::net::UdpSocket;

pub struct Sender<'s> {
    local_socket: &'s UdpSocket
}

impl<'s> Sender<'s> {
    pub fn new(socket_addr: &'s UdpSocket) -> Self {
        Sender {
            local_socket: socket_addr
        }
    }

    pub fn send(&self, msg: &str, remote_socket: &str) {
        let msg = msg.as_bytes();
        let res = self.local_socket.send_to(&msg, &remote_socket);
        match res {
            Ok(_) => (),
            Err(e) => panic!("Sending Failed: {}", e),
        }
    }
}