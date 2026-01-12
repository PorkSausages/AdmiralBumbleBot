use crate::storage_models::{
    DatabaseLayer, JenkemModel, MessageModel, TABLE_HISTORY, TABLE_JENKEM,
};

pub fn log_activity(
    user_id: u64,
    channel_id: u64,
    word_count: u16,
    timestamp: u64,
    db: &DatabaseLayer,
) {
    db.update(TABLE_HISTORY, user_id, |history: &mut Vec<MessageModel>| {
        history.push(MessageModel {
            channel: channel_id,
            time: timestamp,
            words: word_count,
        });
    });
}

pub fn get_user_message_data(user_id: u64, db: &DatabaseLayer) -> Vec<MessageModel> {
    db.get(TABLE_HISTORY, user_id)
}

pub fn pass_jenkem(recipient: u64, db: &DatabaseLayer) -> i32 {
    db.update(TABLE_JENKEM, 0, |jenk: &mut JenkemModel| {
        jenk.pass(recipient);
    })
    .huff_count
}

pub fn reject_jenkem(db: &DatabaseLayer) -> Result<(), ()> {
    let mut res = Ok(());
    db.update(TABLE_JENKEM, 0, |jenk: &mut JenkemModel| {
        res = jenk.reject();
    });
    res
}

pub fn locate_jenkem(db: &DatabaseLayer) -> u64 {
    db.get::<u8, JenkemModel>(TABLE_JENKEM, 0).current_holder
}

pub fn init_jenkem(brewer: u64, db: &DatabaseLayer) {
    db.update(TABLE_JENKEM, 0, |jenk: &mut JenkemModel| {
        *jenk = JenkemModel {
            current_holder: brewer,
            previous_holder: 0,
            huff_count: 0,
            top_streak: jenk.top_streak,
        };
    });
}

pub fn update_jenkem_streak(streak: i32, db: &DatabaseLayer) {
    db.update(TABLE_JENKEM, 0, |jenk: &mut JenkemModel| {
        if jenk.top_streak >= streak {
            return;
        }
        jenk.top_streak = streak;
    });
}

pub fn get_jenkem_streak(db: &DatabaseLayer) -> i32 {
    db.get::<u8, JenkemModel>(TABLE_JENKEM, 0).top_streak
}
