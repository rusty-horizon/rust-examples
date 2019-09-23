extern crate nx;

fn main() {
    
    nx::console::initialize();

    println!("Hello world!");

    // Get version and print it
    let ver = nx::os::get_version().unwrap();
    println!("System version: {}", ver);

    nx::console::flush();

    loop {
    }
}
