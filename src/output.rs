use std::io;
use std::os::fd::{RawFd};
use std::convert::TryInto;
use clap::{ValueEnum};

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum OutputType {
    LCD,
    HDMI
}

fn from_nix_result<T>(res: ::nix::Result<T>) -> io::Result<T> {
    match res {
        Ok(r) => { Ok(r) }
        Err(err) => { Err(err.into()) }
    }
}

mod ioctl {
    use nix::{ioctl_write_ptr_bad};

    const  DISP_DEVICE_SWITCH: u32       = 0x0F;
    const  DISP_GET_OUTPUT_TYPE: u32     = 0x09;

    ioctl_write_ptr_bad!(get_output, DISP_GET_OUTPUT_TYPE, [usize]); 
    ioctl_write_ptr_bad!(set_output, DISP_DEVICE_SWITCH, [usize]);   
}

pub fn get_output(fd: RawFd) -> io::Result<OutputType> {
    let data: [usize; 4] = [0, 0, 0, 0];
    let raw_value: u32 = from_nix_result(unsafe { ioctl::get_output(fd, &data) })?.try_into().unwrap();
    Ok(if raw_value == 1 { OutputType::LCD } else { OutputType::HDMI })
}

pub fn set_output(fd: RawFd, output_type: OutputType) -> io::Result<()> {
    let data: [usize; 4] = match output_type {
        OutputType::LCD => {[0, 1, 0, 0]}
        OutputType::HDMI => {[0, 4, 5, 0]}
    };
    from_nix_result(unsafe { ioctl::set_output(fd, &data) })?;
    Ok(())
}
