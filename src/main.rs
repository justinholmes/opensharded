#[macro_use]
extern crate maplit;

use std::collections::{HashMap, LinkedList};
use config::{Config, FileFormat, Map};
use crate::models::{ApplicationInfo, ConnectionInfo, DatabaseType, StorageType};

mod data_movement;
mod models;
mod api;

fn main() {


    let settings = Config::builder()
        .add_source(config::File::with_name("settings"))
        .add_source(config::Environment::with_prefix("OSH"))
        .build()
        .unwrap();

    let mut connection_info: ConnectionInfo = settings.try_deserialize().unwrap();

    println!(
        "{:?}",
        connection_info
    );
    let connection_info = connection_info.create_storage_types();

    let info = ApplicationInfo::new(
        1,
        "country".to_string(),
        2,
        1,
        connection_info);
    let lower_tier_of_storage = info.get_lower_tier_of_storage();
    println!("{:?}", lower_tier_of_storage);

}
