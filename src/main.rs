#[macro_use]
extern crate maplit;

use crate::models::{ApplicationInfo, ConnectionInfo, DatabaseType, StorageType};

mod data_movement;
mod models;
mod api;

fn main() {
    let connection = ConnectionInfo::new(DatabaseType::Postgresql,
                                         hashmap! { "europe-west1-a".to_string() =>
                                             hashmap! {StorageType::Hdd =>
                                                 "postgresql://".to_string(),
                                                 StorageType::Ssd => "postgresql:".to_string()},
                "europe-west1-b".to_string() =>
                hashmap! {StorageType::Hdd => "postgresql:".to_string()},
                 "us-central-a".to_string() =>
                hashmap! {StorageType::Hdd => "postgresql:".to_string()},
                 "us-central-b".to_string() =>
                hashmap! {StorageType::Ssd => "postgresql:".to_string()},
            });
    let info = ApplicationInfo::new(
        1,
        "country".to_string(),
        2,
        1,
        connection);
    let lower_tier_of_storage = info.get_lower_tier_of_storage();
    println!("{:?}", lower_tier_of_storage);

    println!("{:?}", StorageType::Ssd == info.get_connection_info_storage_types()[0]);
}
