use assert_cmd::prelude::*; // add methods on commands
use ntest::timeout;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use std::{thread, time::Duration};

#[test]
#[test]
#[test]
#[test]
#[test]
#[test]
#[test]
fn foc_118_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("Ion_FoC")?;

    cmd.arg("118").arg("openblt/RealtimeECU.srec");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Finishing programming session"));

    Ok(())
}


// #[test]
// fn multi_foc_118_test() {
//     for n in 0..3 {
//         foc_118_test();
//         thread::sleep(Duration::from_millis(1000));
//     }
// }
