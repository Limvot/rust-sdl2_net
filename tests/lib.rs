extern crate sdl2_net;

#[test]
fn init() {
    unsafe { sdl2_net::ffi::SDLNet_Init(); }
}
