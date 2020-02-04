use std::thread;
use std::net::UdpSocket;
use std::io;
use std::sync::mpsc;
use std::process::Command;
use std::io::Write;

mod listener;
mod sender;

// const MY_SOCKET: &str = "10.0.0.73:8888";
// const REMOTE_SOCKET: &str = "10.0.0.61:8888";

fn main() {
    print!("Enter remote ip: ");
    match io::stdout().flush() {
        Ok(_) => (),
        Err(e) => panic!("Printing to the screen failed lmao what: {}", e),
    };
    let (my_ip, remote_ip) = &get_ips();

    let (tx, rx) = mpsc::channel();
    let socket = UdpSocket::bind(&my_ip).expect("Failed to create Socket");
    socket.connect(&remote_ip).expect("couldnt connect to remote");
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
                    tx.send(input).unwrap();
                    break;
                }
                else {
                    tx.send(input).unwrap();
                }
            },
            Err(e) => println!("idk bruh: {:?}", e),
        }
    });

    loop {
        let res = my_listener.listen();
        match res {
            Ok(_) => my_listener.print_msg(),
            Err(_) => (),
        }
        let msg = rx.try_recv();
        match msg {
            Ok(_) => {
                let msg = &msg.unwrap();
                if msg == &"quit()\n".to_string() {
                    break;
                }
                else {
                    if msg.len() > 1 {
                        my_sender.send(msg, &remote_ip);
                    }
                }
                
            }
            Err(_) => (),
        }
    }
    input_thread.join().expect("Input thread paniced");
}

fn get_ips() -> (String, String) {
    let port = String::from(":8888");

    let output = match Command::new("hostname").args(&["-I"]).output() {
        Ok(ok) => ok,
        Err(_) => return ("".to_string(), "".to_string()),
    };

    let local_ip = match String::from_utf8(output.stdout) {
        Ok(ok) => ok,
        Err(_) => return ("".to_string(), "".to_string()),
    };

    let local_ip = local_ip[..local_ip.len() - 2].to_string();
    let local_ip = (local_ip + &port).to_string();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input[..input.len() - 1].to_string();
            let remote_ip = (input + &port).to_string();
            return (local_ip, remote_ip);
        },
        Err(_) => return ("".to_string(), "".to_string()),
    }
}