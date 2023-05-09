use std::{env, io::Error, collections::HashMap, sync::{Mutex, Arc}};

use chat_room::room::{Guest, Room, RoomManager};
use futures_util::StreamExt;
use log::{info, debug};
use tokio::net::{TcpListener, TcpStream};

type Rooms = Arc<Mutex<HashMap<u32, Arc<Mutex<Room>>>>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    let rooms: Rooms = Arc::new(Mutex::new(HashMap::new()));

    let rooms_ = rooms.clone();

    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            let rooms = rooms_.clone();
            debug!("Accepting stream");
            tokio::spawn(accept_connection(stream, rooms));
        }
    });

    let r1 = Arc::new(Mutex::new(Room::new()));
    {
        let mut rooms_lock = rooms.lock().unwrap();
        rooms_lock.insert(1, r1.clone());
    }

    //TODO: Lock will also be here deadlock...
    let mut r1 = r1.lock().unwrap();
    r1.control.start().await;


    Ok(())

}

async fn accept_connection(stream: TcpStream, rooms: Rooms) {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();

    let mut g1 = Guest::new(read, write);

    debug!("Here");
    {
        let mut rooms = rooms.lock().unwrap();
        let room = rooms.get_mut(&1).unwrap();
        debug!("Here 2");
        let room = room.lock().unwrap();
        room.add_guest(&mut g1);
    }

    g1.start().await;

    return ();
}
