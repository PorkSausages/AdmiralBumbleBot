use std::borrow::Borrow;

use redb::{
    Database as RedbDatabase, Key, ReadableDatabase, ReadableTable, TableDefinition, Value,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub const TABLE_HISTORY: TableDefinition<u64, &[u8]> = TableDefinition::new("user_history");
pub const TABLE_JENKEM: TableDefinition<u8, &[u8]> = TableDefinition::new("jenkem");

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageModel {
    pub channel: u64,
    pub time: u64,
    pub words: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct JenkemModel {
    pub current_holder: u64,
    pub previous_holder: u64,
    pub huff_count: i32,
    pub top_streak: i32,
}

impl JenkemModel {
    pub fn pass(&mut self, recipient: u64) -> i32 {
        self.huff_count += 1;
        self.previous_holder = self.current_holder;
        self.current_holder = recipient;
        self.huff_count
    }

    pub fn reject(&mut self) -> Result<(), ()> {
        if self.current_holder == 0 || self.previous_holder == 0 {
            return Err(());
        }
        self.current_holder = self.previous_holder;
        self.previous_holder = 0;
        Ok(())
    }
}

pub struct DatabaseLayer {
    db: RedbDatabase,
}

impl DatabaseLayer {
    pub fn new(path: &str) -> Self {
        let db = RedbDatabase::create(path).expect("Failed to open DB");
        Self { db }
    }

    pub fn get<K, V>(&self, table_def: TableDefinition<K, &[u8]>, key: K) -> V
    where
        K: Key + 'static,
        K: for<'a> Borrow<<K as Value>::SelfType<'a>>,
        V: DeserializeOwned + Default,
    {
        let read_txn = self.db.begin_read().expect("Read txn failed");
        let table = read_txn.open_table(table_def).expect("Open table failed");

        match table.get(key).expect("Read failed") {
            Some(access) => bincode::deserialize(access.value()).unwrap_or_default(),
            None => V::default(),
        }
    }

    pub fn update<K, V, F>(&self, table_def: TableDefinition<K, &[u8]>, key: K, mut f: F) -> V
    where
        K: Key + 'static + Clone,
        K: for<'a> Borrow<<K as Value>::SelfType<'a>>,
        V: DeserializeOwned + Default + Serialize,
        F: FnMut(&mut V),
    {
        let write_txn = self.db.begin_write().expect("Write txn failed");
        let updated = {
            let mut table = write_txn.open_table(table_def).expect("Open table failed");
            let mut data: V = match table.get(key.clone()).expect("Read failed") {
                Some(access) => bincode::deserialize(access.value()).expect("Corrupt DB"),
                None => V::default(),
            };
            f(&mut data);
            let bytes = bincode::serialize(&data).expect("Serialization failed");
            table.insert(key, bytes.as_slice()).expect("Write failed");
            data
        };
        write_txn.commit().expect("Commit failed");
        updated
    }
}
