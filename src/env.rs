use std::ffi::CString;
use std::os::raw::c_char;

pub fn argc() -> i32 {
    std::env::args().len() as i32
}

pub fn argv() -> Vec<*const c_char> {
    let v_args = std::env::args().map(|arg| CString::new(arg).unwrap()).collect::<Vec<CString>>();
    let mut argv = v_args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
    argv.push(std::ptr::null());

    argv
}
