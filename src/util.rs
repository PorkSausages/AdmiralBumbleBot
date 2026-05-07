use std::env;

use anyhow::{anyhow, bail};
use rand::{seq::SliceRandom, Rng};

pub fn get_id_from_env(key: &str) -> Result<u64, anyhow::Error> {
    Ok(env::var(key)?.parse::<u64>()?)
}

pub fn random_string(strings: &[&str]) -> String {
    strings
        .choose(&mut rand::thread_rng())
        .expect("Passed array of static strings should always have atleast 1 member")
        .to_string()
}

pub fn roll_dice(notation: &str) -> Result<u32, anyhow::Error> {
    let s = notation.to_lowercase();

    let (count_str, sides_str) = s
        .split_once('d')
        .ok_or_else(|| anyhow!("Can't split string"))?;

    let count: u32 = if count_str.is_empty() {
        1
    } else {
        count_str.parse()?
    };

    let sides: u32 = sides_str.parse()?;

    if sides == 0 {
        bail!("No sides");
    }

    let mut rng = rand::thread_rng();
    let mut total = 0;

    for _ in 0..count {
        total += rng.gen_range(1..=sides);
    }

    Ok(total)
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

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Channel {
    General,
    Daw,
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

pub fn id2channel(id: u64) -> Result<Option<Channel>, anyhow::Error> {
    for (chan_key, chan) in [
        ("ABB_GENERAL", Channel::General),
        ("ABB_DAW", Channel::Daw),
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
        if get_id_from_env(chan_key)? == id {
            return Ok(Some(chan));
        }
    }
    Ok(None)
}

pub fn channel2name(channel: Channel) -> &'static str {
    match channel {
        Channel::Bot => "bot",
        Channel::Code => "code",
        Channel::Daw => "DAW",
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
