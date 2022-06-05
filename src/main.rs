use crate::models::{ConnectionInfo, StorageType};
use config::Config;
use log::info;
use std::error::Error;

mod api;
mod data_movement;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let settings = Config::builder()
        .add_source(config::File::with_name("config/settings"))
        .add_source(config::Environment::with_prefix("OSH"))
        .build()
        .unwrap();

    let connection_info: ConnectionInfo = settings.clone().try_deserialize().unwrap();

    info!("{:?}", connection_info);
    let connection_info = connection_info.create_storage_types();

    api::run(settings, connection_info).await
}
