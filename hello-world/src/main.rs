extern crate nx;

fn main() {
    
    nx::console::initialize();

    println!("Hello world!");
    nx::console::flush(); //  Flush always needed in order to show the printed stuff

    // Get version and print it
    let ver = nx::os::get_version().unwrap();
    println!("System version: {}", ver);
    println!("System version (extended): {:?}", ver); // fmt::Debug prints the "SDK" title of the version (shown in DevMenu)
    nx::console::flush();

    println!("Press + to exit.");
    nx::console::flush();

    nx::console::flush();

    loop {
        // Press + to exit
        let ipt = nx::hid::input_down(nx::hid::Controller::Auto);
        if nx::input_any!(ipt, nx::hid::Key::Plus) {
            break;
        }
    }

    nx::console::exit();
}
