use std::io;
use std::os::fd::{RawFd};
use std::convert::TryInto;

fn from_nix_result<T>(res: ::nix::Result<T>) -> io::Result<T> {
    match res {
        Ok(r) => { Ok(r) }
        Err(err) => { Err(err.into()) }
    }
}

mod ioctl {
    use nix::{ioctl_write_ptr_bad};

    const  DISP_LCD_SET_BRIGHTNESS: u32          = 0x102;
    const  DISP_LCD_GET_BRIGHTNESS: u32          = 0x103;

    ioctl_write_ptr_bad!(set_brightness, DISP_LCD_SET_BRIGHTNESS, [u32]); 
    ioctl_write_ptr_bad!(get_brightness, DISP_LCD_GET_BRIGHTNESS, [u32]);   
}

const BRIGHTNESS_MIN_LEVEL: u32 = 37;
const BRIGHTNESS_MAX_LEVEL: u32 = 255;

pub fn set_brightness(fd: RawFd, percent: u32) -> io::Result<()> {
    let raw_value: u32 = percent * (BRIGHTNESS_MAX_LEVEL - BRIGHTNESS_MIN_LEVEL) / 100 + BRIGHTNESS_MIN_LEVEL;
    let data: [u32; 4] = [0, raw_value, 0, 0];
    from_nix_result(unsafe { ioctl::set_brightness(fd, &data) })?;
    Ok(())
}

pub fn get_brightness(fd: RawFd) -> io::Result<u32> {
    let data: [u32; 4] = [0, 0, 0, 0];
    let raw_value: u32 = from_nix_result(unsafe { ioctl::get_brightness(fd, &data) })?.try_into().unwrap();
    let percent: u32 = (raw_value - BRIGHTNESS_MIN_LEVEL) * 100 / (BRIGHTNESS_MAX_LEVEL - BRIGHTNESS_MIN_LEVEL);
    Ok(percent)
}
