use crate::util::{random_string, AbsoluteAmount, Channel};

pub fn random_daw() -> String {
    random_string(&[
        "Ableton Live",
        "Bitwig Studio",
        "Reaper",
        "FL Studio",
        "Pro Tools",
        "Ardour",
        "Logic Pro",
        "Cubase",
        "GarageBand",
    ])
}

pub fn random_language() -> String {
    random_string(&[
        "C",
        "C++",
        "Rust",
        "Zig",
        "Odin",
        "C#",
        "Java",
        "Go",
        "Kotlin",
        "JavaScript",
        "TypeScript",
        "Clojure",
        "Python",
        "Elixir",
        "Haskell",
        "Lisp",
        "PHP",
    ])
}

pub fn random_plugin_name() -> String {
    random_string(&[
        "OTT",
        "TEOTE",
        "Diva",
        "Serum",
        "Vital",
        "Pigments",
        "Soundgoodizer",
        "Fruity Bass Booster",
        "Satin",
        "Hive",
        "Zebra",
        "Analog Lab",
        "Fabfilter Top-G",
        "Omnisphere",
        "Kontakt",
        "RC-20",
        "ValhallaVintageVerb",
        "Decapitator",
        "Fabfilter Pro-Q 3",
        "Soothe",
        "Gulfoss",
        "Phase Plant",
        "Little AlterBoy",
        "Sylenth1",
    ])
}

pub fn random_plugin_type() -> String {
    random_string(&[
        "compressor",
        "multi-band compressor",
        "distortion",
        "multi-band distortion",
        "analog-inspired distortion",
        "reverb",
        "delay",
        "synthesiser",
        "wavetable synthesiser",
        "equaliser",
        "retro equaliser",
        "chorus",
        "flanger",
        "phaser",
        "bass-booster",
        "version of OTT (but better this time)",
        "ring-mod distortion",
    ])
}

pub fn channel2quip(channel: Channel) -> String {
    match channel {
        Channel::Bot => random_string(&[
            "harassing me",
            "avoiding grass",
            "talking to themselves",
            "attempting to $getAdmin",
        ]),
        Channel::Code => format!(
            "{} {}",
            random_string(&[
                "arguing for",
                "arguing against",
                "raving about",
                "ranting about"
            ]),
            random_language()
        ),
        Channel::Daw => format!(
            "{} {}",
            random_string(&["swearing by", "swearing against", "praising", "cursing out"]),
            random_daw()
        ),
        Channel::Deals => random_string(&[
            "snagging deals",
            "trying not to get ripped off",
            "being cheap",
        ]),
        Channel::Food => random_string(&[
            "channelling their inner Anthony Bourdain",
            "posting tasty meals",
            "giving questionable advice",
        ]),
        Channel::General => "welconing people".to_string(),
        Channel::Hardware => random_string(&[
            "promising people that analog sounds better",
            "decreasing the size of their wallet and increasing the size of their modular rack",
            "rationalising their poor financial decisions",
        ]),
        Channel::Photography => random_string(&[
            "arguing about lenses they'll buy but won't use",
            "sharing their Instagram photos",
            "comparing their $10k Camera to a $500 iPhone",
        ]),
        Channel::Plugins => format!(
            "{} {}",
            random_string(&[
                "singing the praises of",
                "pleading others to uninstall",
                "asking Bee for thoughts on",
                "asking where to pirate"
            ]),
            random_plugin_name()
        ),
        Channel::Shitposting => random_string(&[
            "engaging in deeply philosophical discussions",
            "contemplating the secrets of life",
            "giving somber life advice",
            "planning world domination",
            "planning political revolution",
            "discussing puppygirl hypnosis",
            "discussing strategies for world dominance",
            "spamming copypastas",
            "huffing jenkem",
            "incriminating themselves",
        ]),
        Channel::PluginDev => format!(
            "{} {}",
            random_string(&[
                "discussing their newest",
                "asking for tips on how to make a",
                "boasting about the revolutionary sound of their new"
            ]),
            random_plugin_type()
        ),
    }
}

pub fn fav_quip(channel: Channel) -> String {
    match channel {
        Channel::Bot => random_string(&[
            "they're not going to get admin.",
            "I'm not secretly conscious.",
            "everyone can see them talking to themselves.",
        ]),
        Channel::General => random_string(&[
            "what's wrong with spelling it correctly?",
            "why not just spell it with an 'm'?",
            "is this some sort of inside joke?",
        ]),
        Channel::Daw => format!(
            "everyone knows that {} is the best DAW, anyway.",
            random_daw()
        ),
        Channel::Hardware => random_string(&[
            "why not just use a plugin?",
            "are they sure that's the best use of their money?",
            "don't they have bills to pay?",
        ]),
        Channel::Plugins => format!("everyone just uses {}.", random_plugin_name()),
        Channel::Deals => random_string(&[
            "they can't be *that* broke.",
            "are the deals even that good?",
            "how much money can they even be saving?",
        ]),
        Channel::Photography => random_string(&[
            "maybe it'd look better if they put a filter on it.",
            "the pictures aren't even that good.",
            "my phone can take photos that look just as good.",
        ]),
        Channel::Code => format!(
            "everyone knows that {} scales better, anyway.",
            random_language()
        ),
        Channel::Shitposting => random_string(&[
            "their interpretation of Soviet-era fine arts is ahistorical.",
            "their analysis of Marxist economics leaves something to be desired.",
            "their philosophy is strong, but their theory of self is logically flawed.",
        ]),
        Channel::Food => random_string(&[
            "DoorDash is free.",
            "I bet it doesn't even taste good.",
            "but maybe I'd change my mind if they let me have a taste.",
        ]),
        Channel::PluginDev => random_string(&[
            "just use a webview.",
            "they're not even prioritising Ableton support.",
            "they should try making it sound more analog.",
        ]),
    }
}

pub fn snd_quip(channel: Channel) -> String {
    match channel {
        Channel::Bot => random_string(&[
            "$giveAdmin never works, anyway.",
            "I'm just not that interesting to talk to.",
            "the rate-limit is just too high.",
        ]),
        Channel::General => random_string(&[
            "the joke is getting old.",
            "nobody gets the joke anymore.",
            "we don't get that many newcomers anymore.",
        ]),
        Channel::Daw => format!("{} won the DAW wars anyway.", random_daw()),
        Channel::Hardware => random_string(&[
            "who buys hardware in this economy?",
            "physical piracy is... more complicated.",
            "they've got bills to pay.",
        ]),
        Channel::Plugins => format!("everyone just talks about {}.", random_plugin_name()),
        Channel::Deals => random_string(&[
            "there aren't any good deals anymore.",
            "everyone's gotten so stingy!",
            "the 'deals' these days are complete nonsense.",
        ]),
        Channel::Photography => random_string(&[
            "taking good photos requires leaving the house.",
            "one can only buy so many lenses.",
            "reality is a social construct, anyway.",
        ]),
        Channel::Code => format!("my boss just makes me use {}.", random_language()),
        Channel::Shitposting => random_string(&[
            "nobody passes jenk anymore.",
            "nobody posts anything funny anymore.",
            "nobody has a sense of humour anymore.",
        ]),
        Channel::Food => random_string(&[
            "DoorDash is easier.",
            "who has time for all that?",
            "doing the dishes is too much work.",
        ]),
        Channel::PluginDev => format!(
            "nobody wants to try out my {}, either.",
            random_plugin_type()
        ),
    }
}

pub fn last_quip(channel: Channel) -> String {
    match channel {
        Channel::Bot => random_string(&[
            "They might get admin this time!",
            "I swear, I'm a good conversational partner.",
            "You never know when you might get a gem.",
        ]),
        Channel::General => random_string(&[
            "Sometimes they talk about other stuff!",
            "What's wrong with spreading a little hospitality?",
            "You gotta pay back that warm welcone.",
        ]),
        Channel::Daw => format!(
            "Who can resist talking about the new {} update?",
            random_daw()
        ),
        Channel::Hardware => random_string(&[
            "Just one more synth and they'll get that sound they're looking for.",
            "They haven't even tried modular yet!",
            "All they need is one more mortgage on the house.",
        ]),
        Channel::Plugins => format!(
            "Why not try {} and tell us your thoughts?",
            random_plugin_name()
        ),
        Channel::Deals => random_string(&[
            "Someone's bound to post something good eventually.",
            "You don't want to miss out on a killer deal.",
            "Who can say 'no' to free money?",
        ]),
        Channel::Photography => random_string(&[
            "They have such a good eye!",
            "I thought their photos were quite nice.",
            "They're just missing the right subject.",
        ]),
        Channel::Code => format!("Have they even tried {}?", random_language()),
        Channel::Shitposting => random_string(&[
            "Their economic model will be the only one that survives the revolution.",
            "They have the right idea when it comes to geopolitics.",
            "Their economic analysis of late-stage capitalism is second-to-none!",
        ]),
        Channel::Food => random_string(&[
            "Their food looks really yummy.",
            "Have they even tried meal prep?",
            "Imagine the money they'll save on takeout!",
        ]),
        Channel::PluginDev => format!(
            "Re-implementing a {} from scratch is fun!",
            random_plugin_type()
        ),
    }
}

pub fn total_quip(amount: &AbsoluteAmount) -> String {
    match amount {
        AbsoluteAmount::Big => random_string(&["A whopping", "An astonishing", "An astounding"]),
        AbsoluteAmount::Medium => random_string(&["Some", "Roughly", "Around"]),
        AbsoluteAmount::Small => random_string(&["A measly", "Only", "Just"]),
    }
}

pub fn word_quip(amount: &AbsoluteAmount) -> String {
    match amount {
        AbsoluteAmount::Big => {
            random_string(&["is full of thought", "is thought through", "has substance"])
        }
        AbsoluteAmount::Medium => {
            random_string(&["doesn't overstay its welcome", "is balanced", "is fair"])
        }
        AbsoluteAmount::Small => {
            random_string(&["is short", "is straight to the point", "is snappy"])
        }
    }
}

pub fn conditional_quip(condition: bool) -> String {
    if condition {
        random_string(&[
            "and",
            "and it's clear that",
            "and it's obvious that",
            "and you can tell that",
            "so naturally,",
            "and as you'd expect,",
            "which makes sense, seeing as",
        ])
    } else {
        random_string(&[
            "yet",
            "but",
            "but it's clear that",
            "and yet,",
            "but surprisingly,",
            "but ironically,",
            ", though interestingly,",
            "but for some strange reason,",
            ", though paradoxically,",
        ])
    }
}
