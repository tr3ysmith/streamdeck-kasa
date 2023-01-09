use std::{env, time::Duration};

use elgato::{ElgatoSocket, info::ElgatoInfo};
use tracing::{Level, info, error};
use tracing_subscriber::FmtSubscriber;
use tracing_unwrap::{ResultExt, OptionExt};
use anyhow::anyhow;

mod elgato;


/// The program is ran from elgato with the following passed arguments:
/// * `-port` - port used to create the websocket
/// * `-pluginUUID` - UUID of the plugin that we should use when connecting to Elgato
/// * `-registerEvent` - The event type that should be used to register the plugin with Elgato
/// * `-info` - a JSON payload containing the Stream Deck and device information
#[tokio::main]
async fn main() {
    
    let file_appender = tracing_appender::rolling::never("/Users/tr3ysmith/Library/Logs/ElgatoStreamDeck", "com.tr3ysmith.tplinkkasa.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .init();
    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(Level::TRACE)
    //     // .with_writer(non_blocking)
    //     .finish();

    // tracing::subscriber::set_global_default(subscriber)
    //     .expect("Setting default tracing subscriber failed");

    info!("Elgato TP Link Kasa Plugin Starting...");

    let mut port_arg: Option<u16> = None;
    let mut uuid_arg: Option<String> = None;
    let mut event_arg: Option<String> = None;
    let mut info_arg: Option<String> = None;

    let args: Vec<String> = env::args().collect();

    for (index, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-port" => {
                port_arg = Some(args[index + 1].parse().unwrap_or_log());
            },
            "-pluginUUID" => {
                uuid_arg = Some(args[index + 1].clone());
            },
            "-registerEvent" => {
                event_arg = Some(args[index + 1].clone());
            },
            "-info" => {
                info_arg = Some(args[index + 1].clone());
            }
            _ => {
                continue;
            }
        }
    }

    let port = port_arg.ok_or("Port must be specified").unwrap_or_log();
    let uuid = uuid_arg.ok_or("UUID must be specified").unwrap_or_log();
    let event = event_arg.ok_or("Event type must be specified").unwrap_or_log();
    let info: Option<ElgatoInfo>;
    if let Some(info_json) = info_arg {
        info = Some(serde_json::from_str(&info_json).map_err(|e| anyhow!("Unable to parse JSON: {}, {}", e, info_json)).unwrap_or_log());
    } else {
        info = None;
    }

    let elgato = ElgatoSocket::new(port, event, uuid);

    loop {
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }

}
