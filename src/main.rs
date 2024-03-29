mod lcd;
mod output;

use std::fs::File;
use std::os::fd::{AsRawFd};
use std::process::{ExitCode};
use clap::{Parser, Subcommand};

use output::{OutputType};

static DISP_DEV: &str = "/dev/disp";

/// A RG35XX Plus/H toolbox CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "rg35xx-plus-toolbox")]
#[command(about = "A RG35XX Plus/H toolbox CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// set or get LCD brightness in percents
    #[command(name = "brightness")]
    Brightness {
        #[arg(short, long, value_name = "percent")]
        percent: Option<u32>
    },
    /// set or get the current display output
    #[command(name = "display")]
    Display {
        #[arg(short, long, value_name = "type")]
        output_type: Option<OutputType>
    }
}

fn main() -> ExitCode {
    let args = Cli::parse();

    let dev = File::options()
        .read(true)
        .write(true)
        .open(DISP_DEV).unwrap();

    match args.command {
        Commands::Brightness { percent } => {
            if percent.is_none() {
                let result = lcd::get_brightness(dev.as_raw_fd());

                match result {
                    Ok(percent) => { println!("{percent}"); }
                    Err(e) => {
                        eprintln!("failed to get brightness: {e}");
                        return ExitCode::from(1);
                    }
                }
            } else {
                let result = lcd::set_brightness(dev.as_raw_fd(), percent.unwrap());
                
                match result {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("failed to set brightness: {e}");
                        return ExitCode::from(1);
                    }
                }
            };
        }

        Commands::Display { output_type } => {
            if output_type.is_none() {
                let result = output::get_output(dev.as_raw_fd());

                match result {
                    Ok(output_type) => {
                        match output_type {
                            OutputType::LCD => { println!("lcd") }
                            OutputType::HDMI => { println!("hdmi") }
                        };
                    }
                    Err(e) => {
                        eprintln!("failed to set output: {e}");
                        return ExitCode::from(1);
                    }
                }

            } else {
                let result = output::set_output(dev.as_raw_fd(), output_type.unwrap());

                match result {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("failed to set output: {e}");
                        return ExitCode::from(1);
                    }
                }
            }
        }
    };

    ExitCode::from(0)
}
