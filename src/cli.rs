use std::path::PathBuf;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(
    name = "ION FoC",
    about = "Ion Mobility Firmware Update over CAN (ION FoC)"
)]
pub struct CommandLineArgs {
    /// MCU target (Realtime nxps32k118, Telematic nxpsnos32k148)
    // #[structopt(short, long)]
    pub mcu_target: String,

    /// Path to binary file
    #[structopt(parse(from_os_str))]
    pub binary_file: PathBuf,
}