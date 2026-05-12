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

pub fn is_grownup(id:u64) -> Result<bool, anyhow::Error> {
    let grownups = [
        get_id_from_env("ABB_PORKSAUSAGES_ID")?,
        get_id_from_env("ABB_WRL_ID")?,
        get_id_from_env("ABB_M4X_ID")?,
    ];
    Ok(grownups.contains(&id))
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
