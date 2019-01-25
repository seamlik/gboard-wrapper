use dbus_codegen::GenOpts;
use failure::Context;
use failure::Error;
use failure::ResultExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let path = download_interface_ibus().unwrap();
    run_dbus_codegen(&path);
}

fn run_dbus_codegen(path_interface_ibus: &Path) {
    let path_code_ibus = Path::new("src/dbus/ibus/mod.rs");

    if !path_code_ibus.exists() {
        std::fs::create_dir_all(path_code_ibus.parent().unwrap()).expect(&format!(
            "Cannot create directories for {}!",
            path_code_ibus.to_str().unwrap()
        ));
        let code = dbus_codegen::generate(
            &std::fs::read_to_string(&path_interface_ibus).unwrap(),
            &GenOpts::default(),
        )
        .expect("Cannot generate code for the D-Bus interface!");
        std::fs::write(path_code_ibus, code).expect("Cannot write the generated code!");
    }
}

fn download_interface_ibus() -> Result<PathBuf, Error> {
    let url =
        "https://raw.githubusercontent.com/ibus/ibus/1.5.19/portal/org.freedesktop.IBus.Portal.xml";
    let out_dir = std::env::var("OUT_DIR").context("Perhaps not running inside Cargo?")?;
    let path_interface_ibus = Path::new(&out_dir)
        .join("org.freedesktop.IBus.Portal.xml")
        .to_owned();

    if !path_interface_ibus.exists() {
        let status = Command::new("wget")
            .current_dir(&out_dir)
            .arg(url)
            .status()
            .context("Cannot run `wget`!")?;
        if status.success() {
            Ok(path_interface_ibus)
        } else {
            Err(failure::err_msg("Cannot download the iBus interface!"))
        }
    } else {
        Ok(path_interface_ibus)
    }
}
