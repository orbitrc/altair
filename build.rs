use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    const ALTAIR_QT_INCLUDES: &[&str] = &[
        "-I/usr/include/qt",
        "-I/usr/include/qt/QtCore",
        "-I/usr/include/qt/QtWidgets",
    ];

    let gpp = Command::new("g++")
        .args(&["-c", "-fPIC"])
        .args(ALTAIR_QT_INCLUDES)
        .args(&["src/altair.cpp", "-o"])
        .arg(&format!("{}/altair.o", out_dir))
        .status();
    match gpp {
        Ok(_exit_status) => {}
        Err(e) => panic!("g++ compile error: {:?}", e),
    }

    Command::new("ar")
        .args(&["rcs", "libaltair.a"])
        .arg("altair.o")
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=altair");
}
