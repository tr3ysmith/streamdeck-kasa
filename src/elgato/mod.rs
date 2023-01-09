use std::net::{SocketAddr, IpAddr, Ipv4Addr};

use serde_json::{json, Value};
use soketto::handshake::{Client, ServerResponse};
use tokio::{sync::mpsc, net::TcpStream};
use tokio_util::compat::TokioAsyncReadCompatExt;
use tracing::info;

mod commands;
mod events;
pub mod info;
use self::commands::ElgatoCommand;


pub struct ElgatoSocket {
    tx: mpsc::Sender<ElgatoCommand>
}

impl ElgatoSocket {

    pub fn new(port: u16, register_event: String, uuid: String) -> Self {
        let (tx, rx) = mpsc::channel(32);
        tokio::spawn(run_socket(port, register_event, uuid, rx));
        Self { tx }
    }

}


async fn run_socket(port: u16, register_event: String, uuid: String, mut rx: mpsc::Receiver<ElgatoCommand>) {

    let addr = format!("127.0.0.1:{}", port);

    let socket = TcpStream::connect(&addr).await.expect("Unable to connect to Elgato");

    let mut client = Client::new(socket.compat(), &addr, "/");

    let (mut sender, mut receiver) = match client.handshake().await.expect("Unable to perform websocket handshake") {
        ServerResponse::Accepted { .. } => client.into_builder().finish(),
        ServerResponse::Redirect { status_code, location } => panic!("Elgato asked for a redirect, not supported."),
        ServerResponse::Rejected { status_code } => panic!("Elgato rejected the websocket")
    };

    let register = json!({
        "event": register_event,
        "uuid": uuid
    });

    let stringified = serde_json::to_string(&register).unwrap();

    sender.send_text(&stringified).await.expect("Unable to send registration back to Elgato");
    sender.flush().await.unwrap();

    
    loop {
        
        let mut data = Vec::new();
        match receiver.receive_data(&mut data).await {
            Ok(_) => {
                let raw = String::from_utf8_lossy(&data).to_string();
                let value: Value = serde_json::from_str(&raw).unwrap();
                info!("Received: \n{}", serde_json::to_string_pretty(&value).unwrap());
                
            },
            Err(e) => {
                panic!("Unable to receive from Elgato Websocket: {}", e);
            }
        }

    }

}

#[cfg(test)]
mod tests {
    
    use super::*;
    
    #[tokio::test]
    async fn it_works_async() {
    
    }
    
}