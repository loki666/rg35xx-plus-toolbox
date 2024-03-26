use std::io;
use std::os::fd::{RawFd};
use std::convert::TryInto;

fn from_nix_result<T>(res: ::nix::Result<T>) -> io::Result<T> {
    match res {
        Ok(r) => Ok(r),
        Err(err) => Err(err.into()),
    }
}

mod ioctl {
    use nix::{ioctl_write_ptr_bad};

    const  DISP_LCD_SET_BRIGHTNESS: u32          = 0x102;
    const  DISP_LCD_GET_BRIGHTNESS: u32          = 0x103;

    ioctl_write_ptr_bad!(set_brightness, DISP_LCD_SET_BRIGHTNESS, [u32]); 
    ioctl_write_ptr_bad!(get_brightness, DISP_LCD_GET_BRIGHTNESS, [u32]);   
}

pub fn set_brightness(fd: RawFd, val: u32) -> io::Result<u32> {
    let data: [u32; 4] = [0, val, 0, 0];
    from_nix_result(unsafe { ioctl::set_brightness(fd, &data) })?;
    Ok(val)
}

pub fn get_brightness(fd: RawFd) -> io::Result<u32> {
    let data: [u32; 4] = [0, 0, 0, 0];
    let value = from_nix_result(unsafe { ioctl::get_brightness(fd, &data) })?;
    Ok(value.try_into().unwrap())
}