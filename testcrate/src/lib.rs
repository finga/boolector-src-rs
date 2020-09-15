extern crate libc;

use libc::c_char;

extern "C" {
    pub fn boolector_version() -> *const c_char;
}

#[test]
fn version_works() {
    use std::ffi::CStr;

    unsafe {
        let version = CStr::from_ptr(boolector_version());
        println!("{:?}", version);
        assert!(version.to_str().unwrap() == "3.2.1");
    }
}
