use std::env;

use rand::{seq::SliceRandom, Rng};

pub fn get_id_from_env(key: &str) -> u64 {
    env::var(key)
        .unwrap_or_else(|_| panic!("{}", format!("Error getting {} from env", key).to_string()))
        .parse()
        .unwrap_or_else(|_| panic!("{}", format!("Error parsing {} from env", key).to_string()))
}

pub fn random_string(strings: &[&str]) -> String {
    strings.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn roll_dice(notation: &str) -> Result<u32, String> {
    let s = notation.to_lowercase();

    let (count_str, sides_str) = s
        .split_once('d')
        .ok_or_else(|| "Invalid format: missing 'd'".to_string())?;

    let count: u32 = if count_str.is_empty() {
        1
    } else {
        count_str
            .parse()
            .map_err(|_| "Invalid number of dice".to_string())?
    };

    let sides: u32 = sides_str
        .parse()
        .map_err(|_| "Invalid number of sides".to_string())?;

    if sides == 0 {
        return Err("Dice cannot have 0 sides".to_string());
    }

    let mut rng = rand::thread_rng();
    let mut total = 0;

    for _ in 0..count {
        total += rng.gen_range(1..=sides);
    }

    Ok(total)
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Channel {
    General,
    DAW,
    Hardware,
    Plugins,
    Deals,
    Photography,
    Code,
    PluginDev,
    Shitposting,
    Food,
    Bot,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum AbsoluteAmount {
    Small,
    Medium,
    Big,
}

pub fn get_absolute_amount(value: usize, big_bound: usize, mid_bound: usize) -> AbsoluteAmount {
    if value >= big_bound {
        AbsoluteAmount::Big
    } else if value >= mid_bound {
        AbsoluteAmount::Medium
    } else {
        AbsoluteAmount::Small
    }
}

pub fn id2channel(id: u64) -> Option<Channel> {
    for (chan_key, chan) in [
        ("ABB_GENERAL", Channel::General),
        ("ABB_DAW", Channel::DAW),
        ("ABB_WARE", Channel::Hardware),
        ("ABB_PLUG", Channel::Plugins),
        ("ABB_DEAL", Channel::Deals),
        ("ABB_PHOTO", Channel::Photography),
        ("ABB_CODE", Channel::Code),
        ("ABB_DEV", Channel::PluginDev),
        ("ABB_SHIT", Channel::Shitposting),
        ("ABB_FOOD", Channel::Food),
        ("ABB_BOT", Channel::Bot),
    ] {
        if get_id_from_env(chan_key) == id {
            return Some(chan);
        }
    }
    None
}

pub fn channel2name(channel: &Channel) -> &str {
    match channel {
        Channel::Bot => "bot",
        Channel::Code => "code",
        Channel::DAW => "DAW",
        Channel::Deals => "deals",
        Channel::Food => "food",
        Channel::General => "general",
        Channel::Hardware => "hardware",
        Channel::Photography => "photography",
        Channel::Plugins => "plugins",
        Channel::PluginDev => "plugin dev",
        Channel::Shitposting => "shitposting",
    }
}
