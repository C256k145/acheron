use std::thread;
use std::net::UdpSocket;
use std::io;
use std::sync::mpsc;
mod listener;
mod sender;

const MY_SOCKET: &str = "127.0.0.1:3000";
const REMOTE_SOCKET: &str = "127.0.0.1:3001";

fn main() {
    let mut go = true;
    let (tx, rx) = mpsc::channel();
    let socket = UdpSocket::bind(MY_SOCKET).expect("Failed to create Socket");
    socket.connect(REMOTE_SOCKET).expect("couldnt connect to remote");
    let res = socket.set_nonblocking(true);
    match res {
        Ok(_) => (),
        Err(e) => panic!("Couldn't unblock socket: {}", e),
    }
    let my_socket = socket;

    let mut my_listener = listener::Listener::new(&my_socket, 1000);
    let my_sender = sender::Sender::new(&my_socket);

    let input_thread = thread::spawn(move || loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input == "quit()\n" {
                    println!("quitting");
                    go = false;
                    break;
                }
                else {
                    tx.send(input).unwrap();
                }
            },
            Err(e) => println!("idk bruh: {:?}", e),
        }
    });

    while go == true {
        let res = my_listener.listen();
        match res {
            Ok(_) => my_listener.print_msg(),
            Err(_) => (),
        }
        let msg = rx.try_recv();
        match msg {
            Ok(_) => {
                let msg = &msg;
                if msg.as_ref().unwrap().len() > 1 {
                    let msg = &msg;
                    my_sender.send(msg.as_ref().unwrap(), REMOTE_SOCKET);
                }
                
            }
            Err(_) => (),
        }
    }
    input_thread.join().expect("Input thread paniced");
}
