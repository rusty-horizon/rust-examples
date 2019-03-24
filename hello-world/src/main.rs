extern crate libnx;

fn main()
{
    unsafe
    {
        libnx::console::initialize();
        println!("Hello world!");
        loop
        {
            libnx::console::flush();
        }
        libnx::console::exit();
    }
}