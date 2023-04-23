use std::{env, io::Error};

use chat_room::room::{Guest, Room, RoomManager};
use futures_util::StreamExt;
use log::{info, debug};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);


    let mut r1 = Room::new();
    let c1 =r1.control.start();



    while let Ok((stream, _)) = listener.accept().await {
        debug!("Accepting stream");
        tokio::spawn(accept_connection(stream));
    }



    futures::join!( c1 );

        Ok(())

}

async fn accept_connection(stream: TcpStream) {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();

    let mut g1 = Guest::new(read, write);
    g1.start().await;

}
