extern crate libc;
use std::io::prelude::*;
use std::env;
use std::net::{TcpListener, TcpStream};

fn main() {
    let processes: u32 = match env::args().nth(1) {
        Some(n) => {
            match n.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("[33mParse failed. Use default processes[0m");
                    4
                }
            }
        }
        None => 4,
    };
    println!("processes: {}", processes);


    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => unsafe {
                let pid = libc::fork();
                if pid == 0 {
                    handler(&mut stream);
                    break;
                } else {
                    let mut wstatus: i32 = 0;
                    let child_pid = libc::waitpid(-1, &mut wstatus, libc::WNOHANG);
                    println!("pid: {}", child_pid);
                }
            },
            Err(_) => {}
        }
    }
}

fn handler(stream: &mut TcpStream) {
    let mut data = [0; 8];
    let _ = stream.read(&mut data);
    let s = String::from_utf8_lossy(&data[..]);
    unsafe {
        let pid = libc::getpid();
        println!("{}: {}", pid, s);
    }
    let _ = stream.write(&data);
}
