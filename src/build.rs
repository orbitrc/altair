use std::process::Command;
use std::path::Path;
use std::env;
use std::fs;

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

pub fn find_out_dir() -> String {
    let build_dir = match env::var("PROFILE") {
        Ok(profile) => {
            format!("target/{}/build", profile)
        }
        Err(_) => panic!("Failed to get environment variable PROFILE"),
    };
    let read_dir = fs::read_dir(&build_dir).unwrap();
    for entry in read_dir {
        let entry = entry.unwrap();
        let file_name = entry.file_name().into_string().unwrap();
        if file_name.starts_with("altair-") && file_name.len() == 23 {
            let path = Path::new(&build_dir).join(&file_name);
            let read_dir = fs::read_dir(&path).unwrap();
            let name_list = read_dir.map(|x| x.unwrap().file_name().into_string().unwrap())
                .filter(|y| y == "out")
                .collect::<Vec<String>>();
            if name_list.len() == 1 {
                return Path::new(&build_dir).join(&file_name).join("out").to_str().unwrap().to_string();
            }
        }
    }

    panic!("Could not found out_dir.");
}