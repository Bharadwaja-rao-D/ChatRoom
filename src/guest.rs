use std::collections::HashMap;

use tokio::sync::{mpsc, broadcast};

#[derive(Clone)]
pub enum Msg {
   Command(String),
   Msg(String),
}

#[derive(Clone)]
pub struct _Message{
    pub guest: String,
    pub msg: Msg,
}

pub struct GuestConn{
    tx: mpsc::Sender<_Message>,
    rx: broadcast::Receiver<_Message>,
}

pub struct Guest{
    pub id: String,
    pub rooms: HashMap<String, GuestConn>
}
