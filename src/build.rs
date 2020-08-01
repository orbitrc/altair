use std::process::Command;
use std::path::Path;

pub struct Rcc {
    dir: String,
    filename: String,
}

impl Rcc {
    pub fn new() -> Rcc {
        Rcc {
            dir: String::from("resources"),
            filename: String::from("resources.qrc"),
        }
    }

    pub fn build(&self) -> Result<i32, &'static str> {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let file_stem = Path::new(&self.filename).file_stem().unwrap().to_str().unwrap();

        let rcc = Command::new("rcc")
            .arg(&format!("{}/{}", self.dir, self.filename))
            .arg("-o")
            .arg(&format!("{}/qrc_{}.cpp", out_dir, file_stem))
            .status();
        match rcc {
            Ok(exit_status) => {
                if !exit_status.success() {
                    panic!("rcc error with status {}", exit_status.code().unwrap());
                }
                Ok(exit_status.code().unwrap())
            }
            Err(_e) => Err("rcc error"),
        }
    }
}
