use std::path::Path;
use std::process::Command;
use std::fs;

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
        Ok(exit_status) => {
            if !exit_status.success() {
                panic!("g++ compile error with status {}", exit_status.code().unwrap());
            }
        }
        Err(e) => panic!("g++ compile error: {:?}", e),
    }

    let ar = Command::new("ar")
        .args(&["rcs", "libaltair.a"])
        .arg("altair.o")
        .current_dir(&Path::new(&out_dir))
        .status();
    match ar {
        Ok(_exit_status) => {}
        Err(e) => panic!("ar error: {:?}", e),
    }

    let copy = fs::copy("src/altair.h", &format!("{}/altair.h", out_dir));
    match copy {
        Ok(_) => {}
        Err(e) => panic!("altair.h file copy is failed: {}", e),
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=altair");

    println!("cargo:rustc-link-lib=dylib=Qt5Core");
    println!("cargo:rustc-link-lib=dylib=Qt5Widgets");
    println!("cargo:rustc-link-lib=dylib=Qt5Qml");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
