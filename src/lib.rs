//! Hello world server.
//!
//! A simple server that accepts connections, writes "hello world\n", and closes
//! the connection.
//!
//! You can test this out by running:
//!
//!     cargo run --example hello_world
//!
//! And then in another terminal run:
//!
//!     telnet localhost 6142
//!

#![deny(warnings)]

extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

#[macro_use] extern crate log;
extern crate android_logger;

use android_logger::Filter;
use log::Level;

fn native_activity_create() {
    android_logger::init_once(Filter::default().with_min_level(Level::Trace), None);
}

#[no_mangle]
pub extern fn hello_world() -> i32 {

    native_activity_create();

    warn!("I am Started");
    
    5+5
}


#[no_mangle]
pub extern fn hello_world1() -> i32 {
    
    let addr = "127.0.0.1:6142".parse().unwrap();

    warn!("before listener bind");
    // Bind a TCP listener to the socket address.
    //
    // Note that this is the Tokio TcpListener, which is fully async.
    let listener = TcpListener::bind(&addr);

    warn!("after listener bind");
    
    let listener_unwrap = listener.unwrap();

    warn!("listener_unwrap");

    // The server task asynchronously iterates over and processes each
    // incoming connection.
    let server = listener_unwrap.incoming().for_each(|socket| {
        warn!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

        let connection = io::write_all(socket, "hello world\n")
            .then(|res| {
                warn!("wrote message; success={:?}", res.is_ok());
                Ok(())
            });

        // Spawn a new task that processes the socket:
        tokio::spawn(connection);

        Ok(())
    })
    .map_err(|err| {
        // All tasks must have an `Error` type of `()`. This forces error
        // handling and helps avoid silencing failures.
        //
        // In our example, we are only going to log the error to STDOUT.
        warn!("accept error = {:?}", err);
    });

    warn!("server running on localhost:6142");

    // Start the Tokio runtime.
    //
    // The Tokio is a pre-configured "out of the box" runtime for building
    // asynchronous applications. It includes both a reactor and a task
    // scheduler. This means applications are multithreaded by default.
    //
    // This function blocks until the runtime reaches an idle state. Idle is
    // defined as all spawned tasks have completed and all I/O resources (TCP
    // sockets in our case) have been dropped.
    //
    // In our example, we have not defined a shutdown strategy, so this will
    // block until `ctrl-c` is pressed at the terminal.
    tokio::run(server);
    warn!("tokio run");


    5+5
}

/// Expose the JNI interface for android below
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass};
    use self::jni::sys::{jint};

    #[no_mangle]
    pub unsafe extern fn Java_ru_hintsolution_hbbft_hbbft_MainActivity_helloworld(_env: JNIEnv, _: JClass) -> jint {
        // Our Java companion code might pass-in "world" as a string, hence the name.
        let a = hello_world();
        a
    }

    #[no_mangle]
    pub unsafe extern fn Java_ru_hintsolution_hbbft_hbbft_MainActivity_helloworld1(_env: JNIEnv, _: JClass) -> jint {
        // Our Java companion code might pass-in "world" as a string, hence the name.
        let a = hello_world1();
        a
    }
}
