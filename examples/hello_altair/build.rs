use std::process::Command;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let rcc = Command::new("rcc")
        .args(&["resources/resources.qrc", "-o"])
        .arg(&format!("{}/qrc_resources.cpp", out_dir))
        .status();
    match rcc {
        Ok(exit_status) => {
            if !exit_status.success() {
                panic!("rcc error with status {}", exit_status.code().unwrap());
            }
        }
        Err(e) => panic!("rcc error: {:?}", e),
    }

    let rcc_gpp = Command::new("g++")
        .args(&["-c", "-fPIC"])
        .arg(&format!("{}/qrc_resources.cpp", out_dir))
        .arg("-o")
        .arg(&format!("{}/qrc_resources.o", out_dir))
        .status();

    let cargo_pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let ar = Command::new("ar")
        .arg("rcs")
        .arg(&format!("lib{}.a", cargo_pkg_name))
        .arg("qrc_resources.o")
        .current_dir(&Path::new(&out_dir))
        .status();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static={}", cargo_pkg_name);
}
