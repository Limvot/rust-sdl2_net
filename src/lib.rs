extern crate libc;

use libc::c_void;
use std::ffi::CString;
use std::ptr;

pub mod ffi;
pub use ffi::{IPaddress, _TCPsocket, _SDLNet_SocketSet};

pub struct TCPsocket {
    opaque_ptr: *mut _TCPsocket,
}
pub struct SocketSet {
    opaque_ptr: *mut _SDLNet_SocketSet,
}

pub fn init() -> () { unsafe { ffi::SDLNet_Init(); } }
pub fn quit() -> () { unsafe { ffi::SDLNet_Quit(); } }
pub fn get_error() -> String {
    unsafe {
        let raw = &ffi::SDLNet_GetError();
        std::str::from_utf8(std::ffi::c_str_to_bytes(raw)).unwrap().to_string()
    }
}

pub fn become_host(port: u16) -> Option<ffi::IPaddress> {
    let mut address = ffi::IPaddress { host: 0, port: 0};
    let result = unsafe { ffi::SDLNet_ResolveHost(&mut address, ptr::null() , port) };
    if result == 0 {
        Some(address)
    } else {
        None
    }
}

pub fn resolve_host(host: &str, port: u16) -> Option<ffi::IPaddress> {
    let mut address = ffi::IPaddress { host: 0, port: 0 };
    let result = unsafe { ffi::SDLNet_ResolveHost(&mut address, CString::from_slice(host.as_bytes()).as_ptr(), port) };
    if result == 0 {
        Some(address)
    } else {
        None
    }
}

pub fn resolve_ip(mut address: IPaddress) -> String {
    unsafe {
        let raw = &ffi::SDLNet_ResolveIP(&mut address);
        std::str::from_utf8(std::ffi::c_str_to_bytes(raw)).unwrap().to_string()
    }
}

pub fn tcp_open(mut ip: IPaddress) -> TCPsocket {
    unsafe {
        TCPsocket { opaque_ptr: ffi::SDLNet_TCP_Open(&mut ip) }
    }
}

pub fn tcp_close(sock: TCPsocket) -> () {
    unsafe {
        ffi::SDLNet_TCP_Close(sock.opaque_ptr)
    }
}

pub fn tcp_accept(server: TCPsocket) -> TCPsocket {
    unsafe {
        TCPsocket { opaque_ptr: ffi::SDLNet_TCP_Accept(server.opaque_ptr) }
    }
}

pub fn tcp_get_peer_address(sock: TCPsocket) -> Option<IPaddress> {
    unsafe {
        let possible_addr = ffi::SDLNet_TCP_GetPeerAddress(sock.opaque_ptr);
        if possible_addr as *const IPaddress == ptr::null() {
            None
        } else {
            Some(ptr::read(possible_addr))
        }
    }
}

pub fn tcp_send(sock: TCPsocket, data: &[u8]) -> () {
    unsafe {
        ffi::SDLNet_TCP_Send(sock.opaque_ptr, data.as_ptr() as *const c_void, data.len() as i32);
    }
}

pub fn tcp_recv(sock: TCPsocket, maxlen: i32) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::with_capacity(maxlen as usize);
    unsafe {
        let read_ammnt = ffi::SDLNet_TCP_Recv(sock.opaque_ptr, data.as_mut_ptr() as *mut c_void, data.len() as i32);
        data.set_len(read_ammnt as usize);
    }
    data
}

pub fn alloc_socket_set(maxsockets: i32) -> SocketSet {
    unsafe {
        SocketSet { opaque_ptr: ffi::SDLNet_AllocSocketSet(maxsockets) }
    }
}

pub fn free_socket_set(set: SocketSet) -> () {
    unsafe {
        ffi::SDLNet_FreeSocketSet(set.opaque_ptr);
    }
}

pub fn tcp_add_socket(set: SocketSet, sock: TCPsocket) -> i32 {
    unsafe {
        ffi::SDLNet_TCP_AddSocket(set.opaque_ptr, sock.opaque_ptr)
    }
}

pub fn tcp_del_socket(set: SocketSet, sock: TCPsocket) -> i32 {
    unsafe {
        ffi::SDLNet_TCP_DelSocket(set.opaque_ptr, sock.opaque_ptr)
    }
}

pub fn check_sockets(set: SocketSet, timeout: u32) -> i32 {
    unsafe {
        ffi::SDLNet_CheckSockets(set.opaque_ptr, timeout)
    }
}

pub fn socket_ready(sock: TCPsocket) -> i32 {
    unsafe {
        ffi::SDLNet_SocketReady(sock.opaque_ptr)
    }
}
