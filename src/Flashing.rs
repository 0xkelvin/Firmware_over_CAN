use std::path::PathBuf;
use std::process::Command;
use std::io::Result;
use std::io::Error;


#[derive(Debug)]
pub struct Flashing {
    pub mcu: String,
    pub path: PathBuf,
}

pub fn update_firmware(mcu_target: &str, binary_file: PathBuf) {
    
   let mut tid_mcu = "000";
   match mcu_target {
        "118" => tid_mcu = "-tid=118",
        "148" => tid_mcu = "-tid=148",
        _ => {
            eprintln!("Only 118 or 148");
            return
            },
        }
    Command::new("openblt/BootCommander")
        .arg("-t=xcp_can")
        .arg("-d=can0")
        .arg("-b=250000")
        .arg("-t1=1000")
        .arg(tid_mcu)
        .arg("-xid=1")
        // .arg("openblt/RealtimeECU.srec")
        .arg(binary_file)
        .status()
        .expect("failed");

    // println!();
}
