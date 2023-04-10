use futures::stream::{SplitSink, SplitStream};
use tokio::{net::TcpStream, sync::broadcast::{Receiver, Sender}};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

type Sink = SplitSink<WebSocketStream<TcpStream>, Message>;
type Stream = SplitStream<WebSocketStream<TcpStream>>;
type SendRx = Sender<String>;
type ReciverRx = Receiver<String>;

pub struct RoomSender{
    pub sink: Sink,
    pub send_rx: ReciverRx, //tx will be with the RoomControl
}

pub struct RoomReciever{
    pub stream: Stream,
    pub recv_tx: SendRx, //recv_rx will be with the RoomControl
}

pub struct RoomControl{
    pub send_tx: SendRx,
    pub recv_rx: ReciverRx,
}
