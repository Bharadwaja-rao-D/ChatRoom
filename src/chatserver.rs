use std::collections::HashMap;
use tokio::sync::{mpsc, broadcast};
use crate::guest::{Message, Msg};

type Rx = mpsc::Receiver<Message>;
type Tx = mpsc::Sender<Message>;

const BUFFER: usize = 32;

pub struct ChatRoom{
    _rx: Rx,
    _tx: broadcast::Sender<Message>,

    pub tx: Tx,

}

impl ChatRoom{
    pub fn new() -> Self{
        let (tx, _rx) = mpsc::channel(BUFFER);
        let (_tx, _) = broadcast::channel(BUFFER);
        return Self { _rx, _tx, tx  };
    }

    pub async fn start(&mut self, mut rx: Rx) -> (){

        while let Some(msg) = rx.recv().await {
            let msg = msg.msg;
            if let Msg::Command(cmd) = msg {
                // For now only one command that is add
                // Will give on mpsc tx and one broadcast rx to the client
            }
        }
    }
}

pub struct ChatServer{
    pub rooms: HashMap<String, Tx>
}

impl ChatServer{
    pub fn new() -> Self{
        return Self { rooms: HashMap::new() }
    }

}


