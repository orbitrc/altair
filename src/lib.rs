use std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AString {
    ptr: *mut c_void
}

extern "C" {
    fn AString__len(s: AString) -> usize;
}

impl AString {
    pub fn len(&self) -> usize {
        return unsafe {
            AString__len(*self)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
