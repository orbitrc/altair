use std::process::Command;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    Command::new("g++").args(&["-c", "-fPIC", "-I/usr/include/qt", "-I/usr/include/qt/QtCore", "src/altair.cpp", "-o"])
                       .arg(&format!("{}/altair.o", out_dir))
                       .status().unwrap();

    Command::new("ar").args(&["rcs", "libaltair.a"])
                      .arg("altair.o")
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=altair");
}
