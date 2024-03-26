mod display;

use std::fs::File;
use std::os::fd::{AsRawFd};
use clap::{Parser, Subcommand};

static DISP_DEV: &str = "/dev/disp";

/// A RG35XX Plus/H toolbox CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "rg35xx-plus-toolbox")]
#[command(about = "A RG35XX Plus/H toolbox CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// set or get LCD brightness in percents
    #[command(name = "brightness")]
    Brightness {
        #[arg(short, long, value_name = "value")]
        value: Option<u32>
    }
}

fn main() {
    let args = Cli::parse();

    let dev = File::options()
        .read(true)
        .write(true)
        .open(DISP_DEV).unwrap();

    match args.command {
        Commands::Brightness { value } => {
            if value.is_none() {
                let result = display::get_brightness(dev.as_raw_fd());
                println!("{}", result.unwrap());
            } else {
                let result = display::set_brightness(dev.as_raw_fd(), value.unwrap());
                println!("{}", result.unwrap());
            };
        }
    };
}