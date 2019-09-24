extern crate nx;

use nx::{console, twili};
use std::net::{IpAddr, Ipv4Addr, TcpStream};
use std::fmt::Debug;
use std::io::{BufReader, BufRead, Write};

pub fn print<T, E: Debug>(res: Result<T, E>) -> T {
    match res {
        Ok(ok) => ok,
        Err(err) => {
            println!("{:?}", err);
            std::process::exit(0)
        }
    }
}

fn main() {
    let console = console::Handle::new().unwrap();

    let _twili = twili::Handle::new();

    //println!("adding panic hook");

    //nx::no_crash_panic();

    println!("socketInitialize");

    unsafe { nx::sys::socketInitialize(nx::sys::socketGetDefaultInitConfig()); };

    let addr = IpAddr::V4(Ipv4Addr::new(52, 20, 16, 20));

    println!("TcpStream");
    let stream = print(TcpStream::connect((addr, 30001)));
    println!("BufReader");
    let mut buf = BufReader::new(&stream);
    let mut write = &stream;

    println!("write_all");
    write.write_all(b"hi\n").expect("write_all");

    println!("String::new");
    let mut string = String::new();
    println!("read_line");
    buf.read_line(&mut string).expect("read_line");

    println!("{}", string);

    println!("flush");
    console.flush();

    println!("sleep");
    std::thread::sleep(std::time::Duration::from_millis(5000));

    println!("socketExit");
    unsafe { nx::sys::socketExit(); };
}
