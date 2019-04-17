use failure::Error;
use failure::ResultExt;
use std::process::Command;

pub mod dbus;

fn main() {
    println!("{}", get_dbus_address_ibus().unwrap());
}

/// Gets the D-Bus address for iBus.
///
/// This is required for talking to iBus as it does not operate through the central D-Bus.
///
/// # Errors
///
/// This function queries the address by running `ibus address`, so expect errors related to process executions.
pub fn get_dbus_address_ibus() -> Result<String, Error> {
    let output = Command::new("ibus")
        .arg("address")
        .output()
        .context("Failed to execute `ibus address`!")?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)
            .context("`ibus` returned invalid UTF-8 data!")?
            .trim()
            .to_string())
    } else {
        Err(failure::err_msg(
            "Could not find the D-Bus address of iBus! Perhaps it's not running?",
        ))
    }
}
