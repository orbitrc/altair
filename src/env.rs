use std::ffi::CString;
use std::os::raw::c_char;

pub struct Argv {
    args: std::env::Args,
}

impl Argv {
    pub fn len(&self) -> usize {
        self.args.len()
    }

    pub fn as_ptr(self) -> *const *const c_char {
        let v_args = self.args.map(|arg| CString::new(arg).unwrap()).collect::<Vec<CString>>();
        let mut argv = v_args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
        argv.push(std::ptr::null());
        println!("{:?}", argv);

        argv.as_ptr()
    }
}

pub fn argc() -> i32 {
    std::env::args().len() as i32
}

pub fn argv() -> Vec<*const c_char> {
    let v_args = std::env::args().map(|arg| CString::new(arg).unwrap()).collect::<Vec<CString>>();
    let mut argv = v_args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
    argv.push(std::ptr::null());

    argv
}
