#[macro_use]
extern crate maplit;

use std::collections::{HashMap, LinkedList};
use std::error::Error;
use std::sync::{Arc, Mutex};
use config::{Config, FileFormat, Map};
use crate::models::{ApplicationInfo, ConnectionInfo, DatabaseType, StorageType};
use log::{error, info};
use tokio::net::{TcpListener, TcpStream};

mod data_movement;
mod models;
mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let settings = Config::builder()
        .add_source(config::File::with_name("config/settings"))
        .add_source(config::Environment::with_prefix("OSH"))
        .build()
        .unwrap();

    let listen_address = settings.clone().get_string("listen_address").unwrap();
    let mut connection_info: ConnectionInfo = settings.clone().try_deserialize().unwrap();

    info!(
        "{:?}",
        connection_info
    );
    let mut connection_info = connection_info.create_storage_types();
    let connection_map = connection_info.get_connection_map(String::from("europe-west1"));
    let region = connection_map.unwrap();
    let connection_map: Arc<Mutex<HashMap<i32, TcpListener>>> = Arc::new(Mutex::new(HashMap::new()));

    // let info = ApplicationInfo::new(
    //     1,
    //     "country".to_string(),
    //     2,
    //     1,
    //     connection_info.clone());
    // let lower_tier_of_storage = info.get_lower_tier_of_storage();
    // println!("{:?}", lower_tier_of_storage);

    let listener = TcpListener::bind(listen_address).await?;

    loop {
        let (mut local_stream, saddr) = listener.accept().await?;
        let forward = region.get(&StorageType::Hdd).cloned().unwrap();
        info!("New client from {}", saddr);
        tokio::spawn(async move {
            let upstream_ret = TcpStream::connect(forward.clone()).await;
            match upstream_ret {
                Ok(mut upstream) => {
                    let ret = tokio::io::copy_bidirectional(&mut local_stream, &mut upstream).await;
                    match ret {
                        Err(e) => info!("Disconnected, {}", e),
                        Ok((in_byte, out_byte)) => {
                            info!("Disconnected, in bytes={}, out bytes={}", in_byte, out_byte)
                        }
                    }
                }
                Err(e) => error!("Failed connect to {}, {}", forward.clone(), e),
            }
        });
    }
}
