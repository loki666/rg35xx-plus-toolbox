mod display;

use std::fs::File;
use std::os::fd::{AsRawFd};
static DISP_DEV: &str = "/dev/disp";

fn main() {
    let dev = File::options()
        .read(true)
        .write(true)
        .open(DISP_DEV).unwrap();
    
    let result = display::get_brightness(dev.as_raw_fd());

    match result {
        Ok(val) => println!("Ok {val}"),
        Err(err) => println!("{:?}", err),
    }
}