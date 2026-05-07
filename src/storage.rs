use redb::{Database, ReadableDatabase, ReadableTable};

use crate::storage_models::{JenkemModel, MessageModel, Scratchpad, TABLE_HISTORY};

pub fn log_activity(
    user_id: u64,
    channel_id: u64,
    word_count: u16,
    timestamp: u64,
    db: &Database,
) -> Result<(), anyhow::Error> {
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(TABLE_HISTORY)?;
        let mut data = match table.get(user_id)? {
            Some(access) => bincode::deserialize(access.value())?,
            None => Vec::new(),
        };
        data.push(MessageModel {
            channel: channel_id,
            time: timestamp,
            words: word_count,
        });
        let bytes = bincode::serialize(&data)?;
        table.insert(user_id, bytes.as_slice())?;
    };
    write_txn.commit()?;
    Ok(())
}

pub fn get_user_message_data(
    user_id: u64,
    db: &Database,
) -> Result<Vec<MessageModel>, anyhow::Error> {
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(TABLE_HISTORY)?;
    match table.get(user_id)? {
        Some(access) => Ok(bincode::deserialize(access.value()).unwrap_or_default()),
        None => Ok(Vec::new()),
    }
}

pub fn pass_jenkem(recipient: u64, pad: &Scratchpad) -> Result<i32, anyhow::Error> {
    pad.with_mut(|pad| {
        pad.jenkem.huff_count += 1;
        pad.jenkem.previous_holder = pad.jenkem.current_holder;
        pad.jenkem.current_holder = recipient;
        pad.jenkem.huff_count
    })
}

pub fn reject_jenkem(pad: &Scratchpad) -> Result<Result<(), ()>, anyhow::Error> {
    //it's either this or i create a dedicated RejectedJenkemError enum
    pad.with_mut(|pad| {
        if pad.jenkem.current_holder == 0 || pad.jenkem.previous_holder == 0 {
            return Ok(Err(()));
        }
        pad.jenkem.huff_count -= 1;
        pad.jenkem.current_holder = pad.jenkem.previous_holder;
        pad.jenkem.previous_holder = 0;
        Ok(Ok(()))
    })?
}

pub fn locate_jenkem(pad: &Scratchpad) -> u64 {
    pad.with(|pad| pad.jenkem.current_holder)
}

pub fn init_jenkem(brewer: u64, pad: &Scratchpad) -> Result<(), anyhow::Error> {
    pad.with_mut(|pad| {
        pad.jenkem = JenkemModel {
            current_holder: brewer,
            previous_holder: 0,
            huff_count: 0,
            top_streak: pad.jenkem.top_streak,
        }
    })
}

pub fn update_jenkem_streak(streak: i32, pad: &Scratchpad) -> Result<(), anyhow::Error> {
    if get_jenkem_streak(pad) >= streak {
        return Ok(());
    }
    pad.with_mut(|pad| pad.jenkem.top_streak = streak)?;
    Ok(())
}

pub fn get_jenkem_streak(pad: &Scratchpad) -> i32 {
    pad.with(|pad| pad.jenkem.top_streak)
}
