extern crate libc;
// export it for now

use libc::{c_int, c_char, c_void};
use std::ffi::CString;

pub mod ffi;
pub use ffi::{IPaddress, TCPsocket, SDLNet_SocketSet};

pub fn init() -> () { unsafe { ffi::SDLNet_Init(); } }
pub fn quit() -> () { unsafe { ffi::SDLNet_Quit(); } }
pub fn get_error() -> String {
    unsafe {
        let raw = &ffi::SDLNet_GetError();
        std::str::from_utf8(std::ffi::c_str_to_bytes(raw)).unwrap().to_string()
    }
}

pub fn resolve_host(mut address: IPaddress, host: &str, port: u16) -> i32 {
    unsafe {
        ffi::SDLNet_ResolveHost(&mut address, CString::from_slice(host.as_bytes()).as_ptr(), port)
    }
}

pub fn resolve_ip(mut address: IPaddress) -> String {
    unsafe {
        let raw = &ffi::SDLNet_ResolveIP(&mut address);
        std::str::from_utf8(std::ffi::c_str_to_bytes(raw)).unwrap().to_string()
    }
}

pub fn tcp_open(mut ip: IPaddress) -> Box<*mut TCPsocket> {
    unsafe {
        Box::new(ffi::SDLNet_TCP_Open(&mut ip))
    }
}

pub fn tcp_close(sock: &mut TCPsocket) -> () {
    unsafe {
        ffi::SDLNet_TCP_Close(sock)
    }
}

pub fn tcp_accept(server: &mut TCPsocket) -> Box<*mut TCPsocket> {
    unsafe {
        Box::new(ffi::SDLNet_TCP_Accept(server))
    }
}

pub fn tcp_get_peer_address(sock: &mut TCPsocket) -> Box<*mut IPaddress> {
    unsafe {
        Box::new(ffi::SDLNet_TCP_GetPeerAddress(sock))
    }
}

pub fn tcp_send(sock: &mut TCPsocket, data: &[u8]) -> () {
    unsafe {
        ffi::SDLNet_TCP_Send(sock, data.as_ptr() as *const c_void, data.len() as i32);
    }
}

pub fn tcp_recv(sock: &mut TCPsocket, maxlen: i32) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::with_capacity(maxlen as usize);
    unsafe {
        let read_ammnt = ffi::SDLNet_TCP_Recv(sock, data.as_mut_ptr() as *mut c_void, data.len() as i32);
        data.set_len(read_ammnt as usize);
    }
    data
}

pub fn alloc_socket_set(maxsockets: i32) -> Box<*mut SDLNet_SocketSet> {
    unsafe {
        Box::new(ffi::SDLNet_AllocSocketSet(maxsockets))
    }
}

pub fn free_socket_set(set: Box<*mut SDLNet_SocketSet>) -> () {
    unsafe {
        ffi::SDLNet_FreeSocketSet(*set);
    }
}

pub fn tcp_add_socket(set: &mut SDLNet_SocketSet, sock: &mut TCPsocket) -> i32 {
    unsafe {
        ffi::SDLNet_TCP_AddSocket(set, sock)
    }
}

// Maybe should take in the box here as it may get deleted.... not sure
pub fn tcp_del_socket(set: &mut SDLNet_SocketSet, sock: &mut TCPsocket) -> i32 {
    unsafe {
        ffi::SDLNet_TCP_DelSocket(set, sock)
    }
}

pub fn check_sockets(set: &mut SDLNet_SocketSet, timeout: u32) -> i32 {
    unsafe {
        ffi::SDLNet_CheckSockets(set, timeout)
    }
}

pub fn socket_ready(sock: &mut TCPsocket) -> i32 {
    unsafe {
        ffi::SDLNet_SocketReady(sock)
    }
}
