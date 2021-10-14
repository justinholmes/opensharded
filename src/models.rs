use std::collections::HashMap;
use std::panic;

use sorted_vec::SortedSet;

#[derive(Debug)]
pub struct DatacenterLocation {
    locations: HashMap<String, String>,
}

#[derive(Debug)]
pub enum DatabaseType { Cassandra, Postgresql, Mysql }

#[derive(Eq, PartialEq, Hash, Debug, Ord, PartialOrd, Clone, Copy)]
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


#[derive(Debug)]
pub struct ConnectionInfo {
    pub database_type: DatabaseType,
    pub connection_map: HashMap<String, HashMap<StorageType, String>>,
    _storage_types: SortedSet<StorageType>,
}

impl ConnectionInfo {
    pub fn new(database_type: DatabaseType, connection_map: HashMap<String,
        HashMap<StorageType, String>>) -> Self {
        let values = HashMap::clone(&connection_map);
        let mut info = ConnectionInfo {
            database_type,
            connection_map,
            _storage_types: SortedSet::new(),
        };
        for class in values.values().into_iter() {
            for key in class.keys().into_iter() {
                info._storage_types.insert(*key);
            }
        }
        info
    }
}



