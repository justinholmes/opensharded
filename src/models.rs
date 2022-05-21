use std::borrow::Borrow;
use std::collections::HashMap;
use std::panic;

use sorted_vec::SortedSet;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct DatacenterLocation {
    locations: HashMap<String, String>,
}


#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum DatabaseType { Cassandra, Postgresql, Mysql }

#[derive(Eq, PartialEq, Hash, Debug, Ord, PartialOrd, Clone, Copy, Deserialize)]
pub enum StorageType { Memory = 1, Nvme = 2, Ssd = 3, Hdd = 4 }

// #[derive(Debug, IntoUserType, FromUserType)]
// pub struct Hits {
//     pub shard_key: String,
//     pub location_count: HashMap<String, Counter>,
//     pub application_code: u32,
//
// }

#[derive(Debug)]
pub struct ShardInfo {
    pub key: String,
    pub where_id: String,
    pub application_code: u32,
    pub current_datacenter: DatacenterLocation,
    pub in_migration: bool,
    pub country_constraints: Vec<String>,
    pub where_clause: String,
    pub storage_type: StorageType,
}

#[derive(Debug)]
pub struct ApplicationInfo {
    pub application_code: u32,
    pub country_constraints_column: String,
    pub minimum_dc_redundancy: u8,
    pub minimum_rack_quorum: u8,
    connection: ConnectionInfo,
}

impl ApplicationInfo {
    pub fn get_lower_tier_of_storage(&self) -> Option<StorageType> {
        let mut vec = self.connection._storage_types.clone().into_vec();
        if !vec.is_empty() {
            vec.remove(0);
            vec.first().cloned()
        } else {
            None
        }
    }
    pub fn new(application_code: u32, country_constraints_column: String, minimum_dc_redundancy: u8,
               minimum_rack_quorum: u8, connection: ConnectionInfo) -> Self {
        ApplicationInfo {
            application_code,
            country_constraints_column,
            minimum_dc_redundancy,
            minimum_rack_quorum,
            connection,
        }
    }
    pub fn get_connection_info_storage_types(&self) -> Vec<StorageType> {
        self.connection._storage_types.clone().into_vec()
    }
}


#[derive(Debug, Deserialize, Clone)]
pub struct ConnectionInfo {
    pub database_type: DatabaseType,
    pub connection_map: HashMap<String, HashMap<StorageType, String>>,
    #[serde(skip)]
    _storage_types: SortedSet<StorageType>
}

impl ConnectionInfo {
    pub fn create_storage_types(mut self) -> Self {
        for class in self.connection_map.values() {
            for key in class.keys() {
                self._storage_types.insert(*key);
            }
        }
        self
    }
    pub fn new(database_type:DatabaseType, connection_map:HashMap<String, HashMap<StorageType, String>>) -> ConnectionInfo {
        ConnectionInfo{
            database_type,
            connection_map,
            _storage_types: Default::default()
        }
    }
    pub fn get_connection_map(&self, id: String) -> Option<HashMap<StorageType, String>> {
        return self.connection_map.get(&*id).cloned();
    }
}

#[cfg(test)]
mod tests {
    use crate::{ApplicationInfo, ConnectionInfo, DatabaseType, StorageType};

    #[test]
    fn it_test() {
        let mut connection = ConnectionInfo::new(DatabaseType::Postgresql,
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
        connection = connection.create_storage_types();
        let info = ApplicationInfo::new(
            1,
            "country".to_string(),
            2,
            1,
            connection);
        let lower_tier_of_storage = info.get_lower_tier_of_storage().unwrap();
        assert_eq!(StorageType::Hdd, lower_tier_of_storage);
        assert_eq!(StorageType::Ssd, info.get_connection_info_storage_types()[0]);
        assert_eq!(DatabaseType::Postgresql, info.connection.database_type);
        assert_eq!(1, info.minimum_rack_quorum);
        assert_eq!(2, info.minimum_dc_redundancy);
        assert_eq!(1, info.application_code);
        assert_eq!("country".to_string(), info.country_constraints_column);
    }
}

