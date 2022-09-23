use std::path::Path;
use std::process::Command;
use cli::CommandLineArgs;
use structopt::StructOpt;
mod cli;
mod Flashing;

fn main() {
    let CommandLineArgs {
        mcu_target,
        binary_file,
    }  = cli::CommandLineArgs::from_args();
    println!("{:#?}", cli::CommandLineArgs::from_args());
    
    Flashing::update_firmware(&mcu_target, binary_file);
}
