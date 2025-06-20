use chrono::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct CronExpression {
    pub second: String,
    pub minute: String,
    pub hour: String,
    pub day_of_month: String,
    pub month: String,
    pub day_of_week: String,
}

impl CronExpression {
    pub fn new() -> Self {
        CronExpression {
            second: "0".to_string(),
            minute: "*".to_string(),
            hour: "*".to_string(),
            day_of_month: "*".to_string(),
            month: "*".to_string(),
            day_of_week: "*".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.second, self.minute, self.hour, self.day_of_month, self.month, self.day_of_week
        )
    }
}

fn static_map(
    pairs: &[(&'static str, &'static str)],
) -> &'static HashMap<&'static str, &'static str> {
    static MAP: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    MAP.get_or_init(|| pairs.iter().cloned().collect())
}

fn day_name_to_number(day: &str) -> Option<&'static str> {
    static DAYS: &[(&str, &str)] = &[
        ("sunday", "0"),
        ("sun", "0"),
        ("monday", "1"),
        ("mon", "1"),
        ("tuesday", "2"),
        ("tue", "2"),
        ("wednesday", "3"),
        ("wed", "3"),
        ("thursday", "4"),
        ("thu", "4"),
        ("friday", "5"),
        ("fri", "5"),
        ("saturday", "6"),
        ("sat", "6"),
    ];
    static_map(DAYS).get(day).copied()
}

fn month_name_to_number(month: &str) -> Option<&'static str> {
    static MONTHS: &[(&str, &str)] = &[
        ("january", "1"),
        ("jan", "1"),
        ("february", "2"),
        ("feb", "2"),
        ("march", "3"),
        ("mar", "3"),
        ("april", "4"),
        ("apr", "4"),
        ("may", "5"),
        ("june", "6"),
        ("jun", "6"),
        ("july", "7"),
        ("jul", "7"),
        ("august", "8"),
        ("aug", "8"),
        ("september", "9"),
        ("sep", "9"),
        ("october", "10"),
        ("oct", "10"),
        ("november", "11"),
        ("nov", "11"),
        ("december", "12"),
        ("dec", "12"),
    ];
    static_map(MONTHS).get(month).copied()
}

fn parse_hour_minute(caps: &regex::Captures) -> (u32, String) {
    let mut hour = caps[1].parse::<u32>().unwrap();
    let minute = caps[2].to_string();
    if let Some(period) = caps.get(3) {
        match period.as_str() {
            "pm" if hour < 12 => hour += 12,
            "am" if hour == 12 => hour = 0,
            _ => {}
        }
    }
    (hour, minute)
}

pub fn parse_natural_language(input: &str) -> Result<CronExpression, String> {
    let input = input.to_lowercase();
    let mut cron = CronExpression::new();

    macro_rules! try_match {
        ($pattern:expr, $expr:expr) => {
            if let Some(caps) = Regex::new($pattern).unwrap().captures(&input) {
                $expr(&caps);
                return Ok(cron);
            }
        };
    }

    try_match!(
        r"every\s+(\d+)\s+(second|seconds|sec|secs)",
        |caps: &regex::Captures| {
            cron.second = format!("*/{}", &caps[1]);
        }
    );

    try_match!(
        r"every\s+(\d+)\s+(minute|minutes|min|mins)",
        |caps: &regex::Captures| {
            cron.minute = format!("*/{}", &caps[1]);
        }
    );

    try_match!(r"every\s+(\d+)\s+hour", |caps: &regex::Captures| {
        cron.hour = format!("*/{}", &caps[1]);
        cron.minute = "0".into();
        cron.second = "0".into();
    });

    try_match!(
        r"every\s+day\s+at\s+(\d{1,2}):(\d{2}):(\d{2})(?:\s*(am|pm))?",
        |caps: &regex::Captures| {
            let mut hour = caps[1].parse::<u32>().unwrap();
            let minute = caps[2].to_string();
            let second = caps[3].to_string();
            if let Some(period) = caps.get(4) {
                match period.as_str() {
                    "pm" if hour < 12 => hour += 12,
                    "am" if hour == 12 => hour = 0,
                    _ => {}
                }
            }
            cron.hour = hour.to_string();
            cron.minute = minute;
            cron.second = second;
        }
    );

    try_match!(
        r"every\s+day\s+at\s+(\d{1,2}):(\d{2})(?:\s*(am|pm))?",
        |caps: &regex::Captures| {
            let (hour, minute) = parse_hour_minute(caps);
            cron.hour = hour.to_string();
            cron.minute = minute;
        }
    );

    try_match!(
        r"at\s+(\d{1,2}):(\d{2}):(\d{2})(?:\s*(am|pm))?",
        |caps: &regex::Captures| {
            let mut hour = caps[1].parse::<u32>().unwrap();
            let minute = caps[2].to_string();
            let second = caps[3].to_string();
            if let Some(period) = caps.get(4) {
                match period.as_str() {
                    "pm" if hour < 12 => hour += 12,
                    "am" if hour == 12 => hour = 0,
                    _ => {}
                }
            }
            cron.hour = hour.to_string();
            cron.minute = minute;
            cron.second = second;
        }
    );

    try_match!(
        r"at\s+(\d{1,2}):(\d{2})(?:\s*(am|pm))?",
        |caps: &regex::Captures| {
            let (hour, minute) = parse_hour_minute(caps);
            cron.hour = hour.to_string();
            cron.minute = minute;
        }
    );

    try_match!(
        r"every\s+(monday|tuesday|wednesday|thursday|friday|saturday|sunday|mon|tue|wed|thu|fri|sat|sun)",
        |caps: &regex::Captures| {
            if let Some(day) = day_name_to_number(&caps[1]) {
                cron.day_of_week = day.to_string();
                cron.hour = "0".into();
                cron.minute = "0".into();
                cron.second = "0".into();
            }
        }
    );

    try_match!(
        r"every\s+(monday|tuesday|wednesday|thursday|friday|saturday|sunday|mon|tue|wed|thu|fri|sat|sun)\s+at\s+(\d{1,2}):(\d{2}):(\d{2})(?:\s*(am|pm))?",
        |caps: &regex::Captures| {
            if let Some(day) = day_name_to_number(&caps[1]) {
                let mut hour = caps[2].parse::<u32>().unwrap();
                let minute = caps[3].to_string();
                let second = caps[4].to_string();
                if let Some(period) = caps.get(5) {
                    match period.as_str() {
                        "pm" if hour < 12 => hour += 12,
                        "am" if hour == 12 => hour = 0,
                        _ => {}
                    }
                }
                cron.day_of_week = day.to_string();
                cron.hour = hour.to_string();
                cron.minute = minute;
                cron.second = second;
            }
        }
    );

    try_match!(
        r"every\s+(monday|tuesday|wednesday|thursday|friday|saturday|sunday|mon|tue|wed|thu|fri|sat|sun)\s+at\s+(\d{1,2}):(\d{2})(?:\s*(am|pm))?",
        |caps: &regex::Captures| {
            if let Some(day) = day_name_to_number(&caps[1]) {
                let (hour, minute) = parse_hour_minute(caps);
                cron.day_of_week = day.to_string();
                cron.hour = hour.to_string();
                cron.minute = minute;
            }
        }
    );

    try_match!(
        r"every\s+(january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)",
        |caps: &regex::Captures| {
            if let Some(month) = month_name_to_number(&caps[1]) {
                cron.month = month.to_string();
                cron.day_of_month = "1".into();
                cron.hour = "0".into();
                cron.minute = "0".into();
                cron.second = "0".into();
            }
        }
    );

    try_match!(r"every\s+weekday", |_| {
        cron.day_of_week = "1-5".into();
        cron.hour = "9".into();
        cron.minute = "0".into();
        cron.second = "0".into();
    });

    try_match!(r"every\s+weekend", |_| {
        cron.day_of_week = "0,6".into();
        cron.hour = "10".into();
        cron.minute = "0".into();
        cron.second = "0".into();
    });

    try_match!(r"every\s+(\d+)\s+days?", |caps: &regex::Captures| {
        cron.day_of_month = format!("*/{}", &caps[1]);
        cron.hour = "0".into();
        cron.minute = "0".into();
        cron.second = "0".into();
    });

    try_match!(r"every\s+(\d+)\s+weeks?", |caps: &regex::Captures| {
        let weeks = caps[1].parse::<i64>().unwrap();
        let future = Local::now() + chrono::Duration::weeks(weeks);
        cron.day_of_month = future.day().to_string();
        cron.hour = "0".into();
        cron.minute = "0".into();
        cron.second = "0".into();
    });

    try_match!(r"every\s+(\d+)\s+months?", |caps: &regex::Captures| {
        cron.month = format!("*/{}", &caps[1]);
        cron.day_of_month = "1".into();
        cron.hour = "0".into();
        cron.minute = "0".into();
        cron.second = "0".into();
    });

    if input.contains("hourly") {
        cron.minute = "0".into();
        cron.second = "0".into();
        return Ok(cron);
    }

    if input.contains("daily") {
        cron.hour = "0".into();
        cron.minute = "0".into();
        cron.second = "0".into();
        return Ok(cron);
    }

    if input.contains("weekly") {
        cron.day_of_week = "0".into();
        cron.hour = "0".into();
        cron.minute = "0".into();
        cron.second = "0".into();
        return Ok(cron);
    }

    if input.contains("monthly") {
        cron.day_of_month = "1".into();
        cron.hour = "0".into();
        cron.minute = "0".into();
        cron.second = "0".into();
        return Ok(cron);
    }

    if input.contains("yearly") || input.contains("annually") {
        cron.month = "1".into();
        cron.day_of_month = "1".into();
        cron.hour = "0".into();
        cron.minute = "0".into();
        cron.second = "0".into();
        return Ok(cron);
    }

    Err(format!("Could not parse '{input}' into a cron expression",))
}
