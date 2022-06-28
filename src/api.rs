use crate::{ConnectionInfo, StorageType};
use config::Config;
use log::{error, info};
use std::error::Error;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub async fn run(settings: Config, connection_info: ConnectionInfo) -> Result<(), Box<dyn Error>> {
    let connection_map = connection_info.get_connection_map(String::from("europe-west1"));
    let region = connection_map.unwrap();
    let listen_address = settings.clone().get_string("listen_address").unwrap();
    let listener = TcpListener::bind(listen_address).await?;

    loop {
        let (mut local_stream, _saddr) = listener.accept().await?;
        let forward = region.get(&StorageType::Hdd).cloned().unwrap();

        tokio::spawn(async move {
            let std_stream = std::net::TcpStream::connect(forward.clone());
            let stream = TcpStream::from_std(std_stream.unwrap());
            match stream {
                Ok(mut upstream) => {
                    let ret = tokio::io::copy_bidirectional(&mut local_stream, &mut upstream).await;
                    match ret {
                        Err(e) => info!("Disconnected, {}", e),
                        Ok((in_byte, out_byte)) => {
                            info!("Disconnected, in bytes={}, out bytes={}", in_byte, out_byte)
                        }
                    }
                }
                Err(e) => {
                    error!("Failed connect to {}, {}", forward.clone(), e)
                }
            }
        });
    }
}
