//! Accent table: which letters open a popup, and what it offers.
//!
//! A built-in table for US English is used by default. It can be extended or
//! overridden with a config file at `$XDG_CONFIG_HOME/ibus-hold/config`
//! (usually `~/.config/ibus-hold/config`):
//!
//! ```text
//! # delay before the popup opens, in milliseconds
//! delay 500
//! # a letter followed by its variants (whitespace separated)
//! e é è ê ë ē ė ę
//! # an empty list disables the popup for that letter
//! y
//! ```
//!
//! Uppercase variants are derived automatically for lowercase entries unless
//! the uppercase letter has its own line.
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use log::{info, warn};

/// letter -> candidates shown in the popup (at most 9, one per digit key)
pub type Accents = HashMap<char, Vec<String>>;

/// Maximum candidates per letter: selection uses the digit keys `1` ~ `9`.
pub const MAX_CANDIDATES: usize = 9;

pub const DEFAULT_DELAY: Duration = Duration::from_millis(500);

const BUILTIN: &[(char, &str)] = &[
    ('a', "à á â ä æ ã å ā"),
    ('c', "ç ć č"),
    ('e', "è é ê ë ē ė ę"),
    ('i', "î ï í ī į ì"),
    ('l', "ł"),
    ('n', "ñ ń"),
    ('o', "ô ö ò ó œ ø ō õ"),
    ('s', "ß ś š"),
    ('u', "û ü ù ú ū"),
    ('y', "ÿ"),
    ('z', "ž ź ż"),
];

#[derive(Debug, Clone)]
pub struct Config {
    pub accents: Accents,
    pub delay: Duration,
}

fn builtin_lower() -> Accents {
    BUILTIN.iter().map(|(c, list)| (*c, split(list))).collect()
}

/// The built-in table (lowercase and derived uppercase).
#[cfg(test)]
pub fn builtin() -> Accents {
    with_uppercase(builtin_lower(), &Accents::new())
}

/// Load the built-in table, then apply the user config file if it exists.
///
/// The `IBUS_HOLD_DELAY_MS` environment variable overrides the delay.
pub fn load() -> Config {
    let mut lower = builtin_lower();
    let mut explicit_upper = Accents::new();
    let mut delay = DEFAULT_DELAY;

    if let Some(path) = config_path() {
        match fs::read_to_string(&path) {
            Ok(text) => {
                info!("config: {}", path.display());
                delay = parse(&text, &mut lower, &mut explicit_upper).unwrap_or(delay);
            }
            Err(_) => info!("no config file at {}, using defaults", path.display()),
        }
    }

    if let Ok(ms) = env::var("IBUS_HOLD_DELAY_MS") {
        match ms.parse::<u64>() {
            Ok(ms) => delay = Duration::from_millis(ms),
            Err(_) => warn!("ignoring invalid IBUS_HOLD_DELAY_MS={ms}"),
        }
    }

    Config {
        accents: with_uppercase(lower, &explicit_upper),
        delay,
    }
}

fn config_path() -> Option<PathBuf> {
    let base = match env::var("XDG_CONFIG_HOME") {
        Ok(d) if !d.is_empty() => PathBuf::from(d),
        _ => PathBuf::from(env::var("HOME").ok()?).join(".config"),
    };
    Some(base.join("ibus-hold").join("config"))
}

/// Parse config text into `lower` / `upper`; returns the delay if set.
fn parse(text: &str, lower: &mut Accents, upper: &mut Accents) -> Option<Duration> {
    let mut delay = None;
    for (n, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut words = line.split_whitespace();
        let head = words.next().unwrap_or_default();
        let rest: Vec<String> = words.map(str::to_string).collect();

        if head == "delay" {
            match rest.first().and_then(|ms| ms.parse::<u64>().ok()) {
                Some(ms) => delay = Some(Duration::from_millis(ms)),
                None => warn!("config line {}: expected `delay <ms>`", n + 1),
            }
            continue;
        }

        let mut chars = head.chars();
        let (Some(c), None) = (chars.next(), chars.next()) else {
            warn!("config line {}: `{head}` is not a single letter", n + 1);
            continue;
        };
        let mut list = rest;
        if list.len() > MAX_CANDIDATES {
            warn!(
                "config line {}: only the first {MAX_CANDIDATES} variants are used",
                n + 1
            );
            list.truncate(MAX_CANDIDATES);
        }
        if c.is_uppercase() {
            upper.insert(c, list);
        } else {
            lower.insert(c, list);
        }
    }
    delay
}

/// Add uppercase entries derived from `lower`, unless given in `upper`.
/// Letters with an empty list are removed (popup disabled).
fn with_uppercase(lower: Accents, upper: &Accents) -> Accents {
    let mut all = Accents::new();
    for (c, list) in &lower {
        let Some(uc) = single_upper(*c) else { continue };
        let derived = list
            .iter()
            .filter_map(|s| {
                let u: String = s.to_uppercase();
                // skip variants without a single-letter uppercase form (ß -> SS)
                (u.chars().count() == s.chars().count()).then_some(u)
            })
            .collect();
        all.insert(uc, derived);
    }
    all.extend(lower);
    all.extend(upper.clone());
    all.retain(|_, list| !list.is_empty());
    all
}

fn single_upper(c: char) -> Option<char> {
    let mut u = c.to_uppercase();
    match (u.next(), u.next()) {
        (Some(u), None) if u != c => Some(u),
        _ => None,
    }
}

fn split(list: &str) -> Vec<String> {
    list.split_whitespace().map(str::to_string).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn builtin_has_upper_and_lower() {
        let a = builtin();
        assert_eq!(a[&'e'][1], "é");
        assert_eq!(a[&'E'][1], "É");
        // ß has no single-letter uppercase, so `S` only gets Ś Š
        assert_eq!(a[&'S'], vec!["Ś", "Š"]);
        assert!(
            a.values()
                .all(|l| !l.is_empty() && l.len() <= MAX_CANDIDATES)
        );
        assert!(!a.contains_key(&'t'));
    }

    #[test]
    fn config_overrides() {
        let mut lower = builtin_lower();
        let mut upper = Accents::new();
        let text = "# comment\ndelay 300\ne é è\nY Ÿ Ý\ny\nt ţ ť\nbad line\n";
        let delay = parse(text, &mut lower, &mut upper);
        assert_eq!(delay, Some(Duration::from_millis(300)));

        let a = with_uppercase(lower, &upper);
        assert_eq!(a[&'e'], vec!["é", "è"]);
        assert_eq!(a[&'E'], vec!["É", "È"]);
        assert_eq!(a[&'t'], vec!["ţ", "ť"]);
        // `y` disabled, but the explicit uppercase line is kept
        assert!(!a.contains_key(&'y'));
        assert_eq!(a[&'Y'], vec!["Ÿ", "Ý"]);
    }
}
