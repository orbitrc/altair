pub mod env;

use std::ffi::c_void;
// use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Application {
    argc: i32,
    app: *const c_void,
    engine: *const c_void,
}

extern "C" {
    fn Application__new(argc: i32, argv: *const *const c_char) -> Application;
    fn Application__load(url: AString);
    fn Application__exec(application: Application) -> i32;
}

impl Application {
    pub fn new(argc: i32, argv: &Vec<*const c_char>) -> Application {
        unsafe {
            Application__new(argc, argv.as_ptr())
        }
    }

    pub fn load(url: String) {
        let a_str = AString::from_string(&url);
        unsafe {
            Application__load(a_str);
        }
    }

    pub fn exec(&self) -> i32 {
        unsafe {
            Application__exec(*self)
        }
    }
}


#[repr(C)]
#[derive(Clone)]
pub struct AByteArray {
    len: usize,
    data: *mut u8,
}

extern "C" {
    fn AByteArray__new(data: *const u8, len: usize) -> AByteArray;
    fn AByteArray__to_a_string(arr: AByteArray) -> AString;
    fn AByteArray__drop(arr: AByteArray);
}

impl Drop for AByteArray {
    fn drop(&mut self) {
        unsafe {
            AByteArray__drop(self.clone());
        }
    }
}


#[repr(C)]
#[derive(Clone, Copy)]
pub struct AString {
    ptr: *mut c_void,
}

extern "C" {
    fn AString__len(s: AString) -> usize;
    // fn AString__drop(s:AString);
}

impl AString {
    pub fn len(&self) -> usize {
        return unsafe {
            AString__len(*self)
        }
    }

    fn from_string(s: &String) -> AString {
        AString__from_string(s)
    }
}

#[no_mangle]
#[allow(non_snake_case)]
fn AString__from_string(s: &String) -> AString {
    let c_data = s.as_bytes().as_ptr();
    let ba = unsafe {
        AByteArray__new(c_data, s.len())
    };

    unsafe {
        AByteArray__to_a_string(ba)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
