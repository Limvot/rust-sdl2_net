// sdl stuff here?
use libc::{c_int, c_char, c_void};

#[repr(C)]
pub struct IPaddress { pub host: u32, pub port: u16 }
pub struct _TCPsocket;
pub struct _SDLNet_SocketSet;
pub struct _SDLNet_GenericSocket;

// Linking setup (using https://github.com/xsleonard/rust-sdl2_image/ as an example)
#[cfg(target_os="macos")]
mod mac {
    //#[cfg(mac_framework)]
    //#[link(kind="framework", name="SDL2_net")]
    //extern {}

    //#[cfg(not(mac_framework))]
    #[link(name="SDL2_net")]
    extern {}
}
#[cfg(any(target_os="linux", target_os="freebsd", target_os="windows"))]
mod others {
    #[link(name="SDL2_net")]
    extern {}
}

extern "C" {
    //General
    pub fn SDLNet_Init() -> c_int;
    pub fn SDLNet_Quit() -> ();
    pub fn SDLNet_GetError() -> *const c_char;

    // Name Resolution
    pub fn SDLNet_ResolveHost(address: *mut IPaddress, host: *const c_char, port: u16) -> c_int;
    pub fn SDLNet_ResolveIP(address: *mut IPaddress) -> *const c_char;

    // TCP Sockets
    pub fn SDLNet_TCP_Open(ip: *mut IPaddress) -> *mut _TCPsocket;
    pub fn SDLNet_TCP_Close(sock: *const _TCPsocket) -> ();
    pub fn SDLNet_TCP_Accept(server: *const _TCPsocket) -> *mut _TCPsocket;
    pub fn SDLNet_TCP_GetPeerAddress(sock: *const _TCPsocket) -> *mut IPaddress;
    pub fn SDLNet_TCP_Send(sock: *const _TCPsocket, data : *const c_void, len: c_int) -> c_int;
    pub fn SDLNet_TCP_Recv(sock: *const _TCPsocket, data: *mut c_void, maxlen: c_int) -> c_int;

    // UDP Sockets

    // UDP Packets

    // Socket Sets
    pub fn SDLNet_AllocSocketSet(maxsockets: c_int) -> *const _SDLNet_SocketSet;
    pub fn SDLNet_FreeSocketSet(set: *const _SDLNet_SocketSet) -> ();
    pub fn SDLNet_AddSocket(set: *const _SDLNet_SocketSet, sock: *const _TCPsocket) -> c_int;
    pub fn SDLNet_DelSocket(set: *const _SDLNet_SocketSet, sock: *const _TCPsocket) -> c_int;
    pub fn SDLNet_CheckSockets(set: *const _SDLNet_SocketSet, timeout: u32) -> c_int;
    pub fn SDLNet_SocketReady(sock: *mut c_void) -> c_int; // documentation unclear as to what it takes in - assmuing tcp for now
}

