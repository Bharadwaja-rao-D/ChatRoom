use log::info;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main(){
    let host = "ws://127.0.0.1:5757";
    let listner = TcpListener::bind(host).await.unwrap();
    info!("Running server: {}",host);
    while let Ok((conn, _)) = listner.accept().await {
        tokio::spawn(async move{
            handle(conn).await;
        });
    }
}

pub async fn handle(conn: TcpStream){
}
