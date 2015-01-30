extern crate sdl2_net;
extern crate sdl2;

#[test]
fn init() {
    sdl2::init(sdl2::INIT_VIDEO);

    sdl2_net::init();

    let socket_set = sdl2_net::alloc_socket_set(20);

    let thing = sdl2_net::become_host(1234);

    match thing {
        Some(x) => println!("Started a server."),
        None => println!("Failed to init."),
    }


    let socket_some = sdl2_net::tcp_open();

    let mut socket: sdl2_net::TCPsocket;

    match socket_some {
        Some(s) => {socket = s; println!("Opened up a listening socket!")},
        None => {println!("Could not open a socket."); return}, 
    }

    sdl2_net::add_socket(&socket_set, &socket);
    
    let mut running = true;

    while running {

        let amnt = sdl2_net::check_sockets(&socket_set, 20);
        if amnt > 0{
            println!("Data is ready to be processed. {} stuffs.", amnt);
        }

        if sdl2_net::socket_ready(&socket) != 0{
            println!("New connection!!");
            /* Accept the connection here */
            running = false;
        }
    }

    println!("Closing server...");

    sdl2_net::free_socket_set(&socket_set);
    sdl2_net::tcp_close(&socket);
    sdl2_net::quit();

    println!("Finished.");
}
