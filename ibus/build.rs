use dbus_codegen::GenOpts;
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
        std::fs::create_dir_all(path_code_ibus.parent().unwrap()).unwrap();
        let code = dbus_codegen::generate(
            &std::fs::read_to_string(&path_interface_ibus).unwrap(),
            &GenOpts::default(),
        )
        .unwrap();
        std::fs::write(path_code_ibus, code).unwrap();
    }
}

fn download_interface_ibus() -> std::io::Result<PathBuf> {
    let url =
        "https://raw.githubusercontent.com/ibus/ibus/1.5.19/portal/org.freedesktop.IBus.Portal.xml";
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let path_interface_ibus = Path::new(&out_dir)
        .join("org.freedesktop.IBus.Portal.xml")
        .to_owned();

    if !path_interface_ibus.exists() {
        Command::new("wget")
            .current_dir(&out_dir)
            .arg(url)
            .status()
            .and_then(|status| {
                if status.success() {
                    Ok(path_interface_ibus)
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Cannot download the iBus interface!",
                    ))
                }
            })
    } else {
        Ok(path_interface_ibus)
    }
}
