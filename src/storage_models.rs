use std::{
    collections::HashMap,
    fs::{rename, File},
    io::BufWriter,
    sync::{Arc, Mutex},
};

use redb::TableDefinition;
use serde::{Deserialize, Serialize};
use serde_json::to_writer_pretty;
use serenity::json::from_reader;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JenkemModel {
    pub current_holder: u64,
    pub previous_holder: u64,
    pub huff_count: i32,
    pub top_streak: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PastaModel {
    pub triggers: Vec<String>,
    pub payload: String,
    pub chance: u32,
    pub includes_mention: bool,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct BlockModel {
    pub message_count: u8,
    pub streak_start_seconds: u64,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ScratchpadModel {
    pub dumb_channels: HashMap<String, String>,
    pub block_map: HashMap<u64, BlockModel>,
    pub jenkem: JenkemModel,
    pub pastas: HashMap<String, PastaModel>,
}

pub struct Scratchpad {
    data: Arc<Mutex<ScratchpadModel>>,
}

impl Scratchpad {
    pub fn new(reset: bool) -> Result<Self, anyhow::Error> {
        if !reset {
            let f = File::open("scratchpad.json")?;
            Ok(Self {
                data: Arc::new(Mutex::new(from_reader(f)?)),
            })
        } else {
            Ok(Self {
                data: (Arc::new(Mutex::new(ScratchpadModel::default()))),
            })
        }
    }
    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&ScratchpadModel) -> R,
    {
        f(&self
            .data
            .lock()
            .expect("Scratchpad mutex should not be poisoned")) //should be more rigorous but overkill considering what we use it for
    }

    pub fn with_mut<F, R>(&self, f: F) -> Result<R, anyhow::Error>
    where
        F: FnOnce(&mut ScratchpadModel) -> R,
    {
        let mut guard = self
            .data
            .lock()
            .expect("Scratchpad mutex should not be poisoned");
        let mut copy = guard.clone();
        let res = f(&mut copy);
        let f = File::create("scratchpad.temp")?;
        let writer = BufWriter::new(f);
        to_writer_pretty(writer, &copy)?;
        rename("scratchpad.temp", "scratchpad.json")?;
        *guard = copy;
        Ok(res)
    }
}

pub const TABLE_HISTORY: TableDefinition<u64, &[u8]> = TableDefinition::new("user_history");

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageModel {
    pub channel: u64,
    pub time: u64,
    pub words: u16,
}
