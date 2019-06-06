extern crate nx;

use nx::{console, twili};

fn main() {
    let console = console::Handle::new().unwrap();

    let _twili = twili::Handle::new();

    println!("Hello world!");

    console.flush();

    std::thread::sleep(std::time::Duration::from_millis(5000));
}
