extern crate libc;
// At this early stage, make the unwrapped functions available
pub mod ffi;

pub fn testing2() {
    println!("2adsfasfd");
}

#[test]
fn it_works() {
    // SDL2 is broken right now, and SDL2_NET needs SDL2 to be init'd before doing it's thing, so
    // no tests for now.
}





