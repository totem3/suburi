extern crate libc;
use std::io::prelude::*;
use std::env;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;

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
    let mut listener = Rc::new(listener);
    unsafe {
        for i in 0..processes {
            let pid = libc::fork();
            if pid == 0 {
                match Rc::get_mut(&mut listener) {
                    Some(listener) => {
                        accept_connection(listener);
                    }
                    None => {} // ignore
                }
            } else {
                if i == processes - 1 {
                    thread::park();
                }
            }
        }
    }
}

fn accept_connection(listener: &mut TcpListener) {
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || handler(&mut stream));
            }
            Err(_) => {
                println!("stream error");
            }
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
    stream.write(&data);
}
