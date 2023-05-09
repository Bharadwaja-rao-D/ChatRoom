use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures::{TryStreamExt, SinkExt};
use futures::stream::{SplitStream, SplitSink};
use log::{info, debug};
use tokio::sync::mpsc;
use tokio::{net::TcpStream, sync::broadcast, task::JoinHandle};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

use crate::RoomId;

type Sender = mpsc::Sender<String>;
type Receiver = mpsc::Receiver<String>;
type Stream = SplitStream<WebSocketStream<TcpStream>>;
type Sink = SplitSink<WebSocketStream<TcpStream>, Message>;


pub struct ToControl{
    read: Stream,
    senders: HashMap<RoomId, Sender>, //groupid and Sender
}

impl ToControl{
    async fn start(&mut self) -> () {
        debug!("Starting tocontrol");
        //Read a Message from stream which contains to which group the msg should be sent and
        //then send the message to the respective sender

        // For now only one room is present

        while let Ok(Some(Message::Text(msg))) = self.read.try_next().await{
            self.senders.get_mut(&1).unwrap().send(msg).await.unwrap();
        }

        return ();
    }
}

pub struct FromControl{
    write: Sink,
    receivers: HashMap<RoomId, broadcast::Receiver<String>>, //groupid and reciever
}

impl FromControl{
     async fn start(&mut self) -> (){
        debug!("Starting fromControl");

            // For now only one room is present

            let receiver = self.receivers.get_mut(&1).unwrap();
            while let Ok(msg) = receiver.recv().await {
                self.write.send(Message::Text(msg)).await.unwrap();
            }

            return ();
    }
}

pub struct Guest{
    tocontrol: ToControl,
    fromcontrol: FromControl
}

impl Guest{
    ///Creates a tocontrol and fromcontrol pieces
    pub fn new(stream: Stream, sink: Sink) -> Self{

        info!("Created new guest");

        let tocontrol = ToControl {read: stream, senders: HashMap::new() } ;
        let fromcontrol = FromControl {write: sink, receivers: HashMap::new()};

        return Self { tocontrol, fromcontrol };

    }

    ///Starts the fromControl and toControl
    pub async fn start(&mut self) {

        futures::join! {
            self.tocontrol.start(),
            self.fromcontrol.start(),
        };
    }
}



#[derive(Debug)]
pub struct Control{
    sender: broadcast::Sender<String>,
    receiver: Receiver,
    t_sender: Sender,
}

impl Control {
    pub fn new() -> Self {
        let (t_sender, receiver) = mpsc::channel(32);
        let (sender, _) = broadcast::channel(32);
        return Self { sender, receiver, t_sender};
    }

    ///A task that runs the controller
    //takes the ownership of Control
    pub async fn start(&mut self) -> (){
        debug!("Starting controller for the room");
        tokio::time::sleep(Duration::from_secs(30)).await;
        while let Some(msg) = self.receiver.recv().await {
            debug!("Recieved: {}", msg);
            self.sender.send(msg).unwrap();
        }
        debug!("Ending controller for the room");
        return ();
    }
}

#[derive(Debug)]
pub struct Room{
    pub control: Control,
    room_id: u32,
}

impl Room {
    pub fn new() -> Self{
        let room_id= 1;
        info!("Created room {}", room_id);
        return Room { control: Control::new(), room_id }
    }

    ///Whenever a guest joins a room tx end of mpsc channel and rx end of a broadcast channel will be
    ///given to him
    pub fn add_guest(&self, guest: &mut Guest) {
        let room_id = self.room_id;
        guest.tocontrol.senders.insert(room_id, self.control.t_sender.clone());
        guest.fromcontrol.receivers.insert(room_id, self.control.sender.subscribe());

        debug!("Added new guest into room: {}", room_id);
    }
}

#[derive(Debug)]
pub struct RoomManager{
    pub rooms: HashMap<u32,Arc<Mutex<Room>>>
}

impl RoomManager{
    pub fn new() -> Self {
        return RoomManager { rooms: HashMap::new() };
    }
    pub fn join_room(&self, room_id: RoomId, guest: &mut Guest){
        let rooms = &self.rooms;
        let room = rooms.get(&room_id).unwrap();
        let room = room.lock().unwrap();
        room.add_guest(guest);
    }

}

