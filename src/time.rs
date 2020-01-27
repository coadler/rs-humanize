use chrono::prelude::*;
use std::time::Duration;

#[derive(Debug)]
struct RelativeTimeMagnitude {
    duration: Duration,
    fmt: &'static str,
    div_by: u32,
}

const MINUTE_SECS: u64 = 60;
const HOUR_SECS: u64 = MINUTE_SECS * 60;
const DAY_SECS: u64 = HOUR_SECS * 24;
const WEEK_SECS: u64 = DAY_SECS * 7;
const MONTH_SECS: u64 = DAY_SECS * 30;
const YEAR_SECS: u64 = MONTH_SECS * 12;

const DEFAULT_MAGNITUDES: [RelativeTimeMagnitude; 17] = [
    // now
    RelativeTimeMagnitude {
        duration: Duration::SECOND,
        fmt: "now",
        div_by: Duration::SECOND.as_nanos() as u32,
    },
    // 1 second ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(2),
        fmt: "1 second {label}",
        div_by: 1,
    },
    // 10 seconds ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(MINUTE_SECS),
        fmt: "{amt} seconds {label}",
        div_by: Duration::SECOND.as_nanos() as u32,
    },
    // 1 minute ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(MINUTE_SECS * 2),
        fmt: "1 minute {label}",
        div_by: 1,
    },
    // 10 minutes ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(HOUR_SECS),
        fmt: "{amt} minutes {label}",
        div_by: Duration::from_secs(MINUTE_SECS).as_nanos() as u32,
    },
    // 1 hour ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(HOUR_SECS * 2),
        fmt: "1 hour {label}",
        div_by: 1,
    },
    // 10 hours ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(DAY_SECS),
        fmt: "{amt} hours {label}",
        div_by: Duration::from_secs(HOUR_SECS).as_nanos() as u32,
    },
    // 1 day ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(DAY_SECS * 2),
        fmt: "1 day {label}",
        div_by: 1,
    },
    // 10 days ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(WEEK_SECS),
        fmt: "{amt} days {label}",
        div_by: Duration::from_secs(DAY_SECS).as_nanos() as u32,
    },
    // 1 week ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(WEEK_SECS * 2),
        fmt: "1 week {label}",
        div_by: 1,
    },
    // 10 weeks ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(MONTH_SECS),
        fmt: "{amt} weeks {label}",
        div_by: Duration::from_secs(WEEK_SECS).as_nanos() as u32,
    },
    // 1 month ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(MONTH_SECS * 2),
        fmt: "1 month {label}",
        div_by: 1,
    },
    // 10 months ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(YEAR_SECS),
        fmt: "{amt} months {label}",
        div_by: Duration::from_secs(MONTH_SECS).as_nanos() as u32,
    },
    // 1 year ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(MONTH_SECS * 18),
        fmt: "1 year {label}",
        div_by: 1,
    },
    // 2 years ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(YEAR_SECS * 2),
        fmt: "2 years {label}",
        div_by: 1,
    },
    // 10 years ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(YEAR_SECS * 2),
        fmt: "{amt} years {label}",
        div_by: Duration::from_secs(YEAR_SECS).as_nanos() as u32,
    },
    // a long while ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(std::u64::MAX),
        fmt: "a long while {label}",
        div_by: 1,
    },
];

pub fn format(then: DateTime<Utc>) -> String {
    let now = Utc::now();
    let (diff, label) = if now > then {
        ((now - then).to_std().unwrap(), "ago")
    } else {
        ((then - now).to_std().unwrap(), "from now")
    };

    let mut magnitude = &DEFAULT_MAGNITUDES[0];
    for mag in &DEFAULT_MAGNITUDES {
        if mag.duration > diff {
            magnitude = mag;
            break;
        }
    }

    let amt_str = (diff / magnitude.div_by).as_nanos().to_string();
    let replace_amt = magnitude.fmt.replace("{amt}", &amt_str);
    replace_amt.replace("{label}", label)
}
