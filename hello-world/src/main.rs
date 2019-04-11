extern crate nx;

use nx::{hid,console};

fn main()
{	
	unsafe {
        console::init();
		
        println!("Hello world!");
		println!("Press PLUS to exit application.");
        loop
        {
			let k_down = hid::input_down(hid::Controller::Handheld);
			if k_down == 1024 as u64{
				break;
			}

            console::flush();
        }
		console::exit();
	}
}