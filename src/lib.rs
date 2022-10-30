use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

const KB: i64 = 1024;
const MB: i64 = KB * 1024;
const GB: i64 = MB * 1024;
const TB: i64 = GB * 1024;

struct Kind {
    size: i64,
    symbol: &'static str,
}

const DENOMINATIONS: &[Kind] = &[
    Kind {
        size: TB,
        symbol: "T",
    },
    Kind {
        size: GB,
        symbol: "G",
    },
    Kind {
        size: MB,
        symbol: "M",
    },
    Kind {
        size: KB,
        symbol: "K",
    },
];

pub fn humanize_number(line: &str) -> Cow<str> {
    lazy_static! {
        static ref NUMBER: Regex = Regex::new(r"\b(?P<number>\d{3,})\b",).unwrap();
    }
    NUMBER.replace_all(line, |caps: &regex::Captures| {
        let n = &caps[1].parse::<i64>().unwrap();
        for d in DENOMINATIONS {
            if *n >= d.size {
                let f = *n as f64 / (d.size as f64);
                return format!("{:.2}{}", f, d.symbol);
            }
        }
        caps[1].to_string()
    })
}
