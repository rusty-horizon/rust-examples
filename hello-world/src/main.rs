extern crate nx;
use nx::console;

fn main() {
    let console = console::Handle::new().unwrap();

    println!("Hello rusty world!");
    console.flush(); // Like in libnx, flush/update after console write

    std::thread::sleep(std::time::Duration::from_millis(5000));
}
