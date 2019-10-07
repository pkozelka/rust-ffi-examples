extern crate libc;

use std::ffi::CStr;
use std::os::raw::c_char;

extern {
    // NOTE: external name can differ from rust fn name
    // NOTE: this way we can handle returned C strings
    // TODO: show something with classes
    #[link_name = "say_something"]
    fn __external_say_something() -> *const c_char;

    fn triple_input(input: libc::c_int) -> libc::c_int;
}

fn say_something() -> String {
    unsafe {
        CStr::from_ptr(__external_say_something()).to_string_lossy().into_owned()
    }
}

fn main() {
    let input = 4;
    let output = unsafe { triple_input(input) };
    println!("{} * 3 = {}", input, output);
    println!("Say something: {}", say_something());
}
