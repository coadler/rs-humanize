use chrono::prelude::*;
use std::time::Duration;

#[derive(Debug)]
struct RelativeTimeMagnitude {
    duration: Duration,
    fmt: &'static str,
    div_by: Duration,
}

const MINUTE_SECS: u64 = 60;
const HOUR_SECS: u64 = MINUTE_SECS * 60;
const DAY_SECS: u64 = HOUR_SECS * 24;
const WEEK_SECS: u64 = DAY_SECS * 7;
const MONTH_SECS: u64 = DAY_SECS * 30;
const YEAR_SECS: u64 = MONTH_SECS * 12;
const LONG_TIME_SECS: u64 = YEAR_SECS * 37;

const DEFAULT_MAGNITUDES: [RelativeTimeMagnitude; 17] = [
    // now
    RelativeTimeMagnitude {
        duration: Duration::SECOND,
        fmt: "now",
        div_by: Duration::SECOND,
    },
    // 1 second ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(2),
        fmt: "1 second {label}",
        div_by: Duration::from_nanos(1),
    },
    // 10 seconds ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(MINUTE_SECS),
        fmt: "{amt} seconds {label}",
        div_by: Duration::SECOND,
    },
    // 1 minute ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(MINUTE_SECS * 2),
        fmt: "1 minute {label}",
        div_by: Duration::from_nanos(1),
    },
    // 10 minutes ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(HOUR_SECS),
        fmt: "{amt} minutes {label}",
        div_by: Duration::from_secs(MINUTE_SECS),
    },
    // 1 hour ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(HOUR_SECS * 2),
        fmt: "1 hour {label}",
        div_by: Duration::from_nanos(1),
    },
    // 10 hours ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(DAY_SECS),
        fmt: "{amt} hours {label}",
        div_by: Duration::from_secs(HOUR_SECS),
    },
    // 1 day ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(DAY_SECS * 2),
        fmt: "1 day {label}",
        div_by: Duration::from_nanos(1),
    },
    // 10 days ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(WEEK_SECS),
        fmt: "{amt} days {label}",
        div_by: Duration::from_secs(DAY_SECS),
    },
    // 1 week ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(WEEK_SECS * 2),
        fmt: "1 week {label}",
        div_by: Duration::from_nanos(1),
    },
    // 10 weeks ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(MONTH_SECS),
        fmt: "{amt} weeks {label}",
        div_by: Duration::from_secs(WEEK_SECS),
    },
    // 1 month ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(MONTH_SECS * 2),
        fmt: "1 month {label}",
        div_by: Duration::from_nanos(1),
    },
    // 10 months ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(YEAR_SECS),
        fmt: "{amt} months {label}",
        div_by: Duration::from_secs(MONTH_SECS),
    },
    // 1 year ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(MONTH_SECS * 18),
        fmt: "1 year {label}",
        div_by: Duration::from_nanos(1),
    },
    // 2 years ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(YEAR_SECS * 2),
        fmt: "2 years {label}",
        div_by: Duration::from_nanos(1),
    },
    // 10 years ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(LONG_TIME_SECS),
        fmt: "{amt} years {label}",
        div_by: Duration::from_secs(YEAR_SECS),
    },
    // a long while ago
    RelativeTimeMagnitude {
        duration: Duration::from_secs(std::u64::MAX),
        fmt: "a long while {label}",
        div_by: Duration::from_nanos(1),
    },
];

pub fn format(then: DateTime<Utc>) -> String {
    let now = Utc::now();
    return format_rel(then, now, "ago", "from now");
}

pub fn format_rel(a: DateTime<Utc>, b: DateTime<Utc>, a_label: &str, b_label: &str) -> String {
    let (diff, label) = if a > b {
        ((a - b).to_std().unwrap(), b_label)
    } else {
        ((b - a).to_std().unwrap(), a_label)
    };

    let mut magnitude = &DEFAULT_MAGNITUDES[0];
    for mag in &DEFAULT_MAGNITUDES {
        if mag.duration > diff {
            magnitude = mag;
            break;
        }
    }

    let amt_str = (diff.as_nanos() / magnitude.div_by.as_nanos()).to_string();
    let replace_amt = magnitude.fmt.replace("{amt}", &amt_str);
    replace_amt.replace("{label}", label)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_rel() {
        use chrono::Duration;

        struct TimePair {
            a: DateTime<Utc>,
            b: DateTime<Utc>,
            expected_str: &'static str,
        }

        let now = Utc::now();
        let tests = vec![
            // singular second
            TimePair {
                a: now - Duration::seconds(1),
                b: now,
                expected_str: "1 second ago",
            },
            TimePair {
                a: now + Duration::seconds(1),
                b: now,
                expected_str: "1 second from now",
            },
            // multiple seconds
            TimePair {
                a: now - Duration::seconds(5),
                b: now,
                expected_str: "5 seconds ago",
            },
            TimePair {
                a: now + Duration::seconds(5),
                b: now,
                expected_str: "5 seconds from now",
            },
            // singular minute
            TimePair {
                a: now - Duration::minutes(1),
                b: now,
                expected_str: "1 minute ago",
            },
            TimePair {
                a: now + Duration::minutes(1),
                b: now,
                expected_str: "1 minute from now",
            },
            // multiple minutes
            TimePair {
                a: now - Duration::minutes(5),
                b: now,
                expected_str: "5 minutes ago",
            },
            TimePair {
                a: now + Duration::minutes(5),
                b: now,
                expected_str: "5 minutes from now",
            },
            // singular hour
            TimePair {
                a: now - Duration::hours(1),
                b: now,
                expected_str: "1 hour ago",
            },
            TimePair {
                a: now + Duration::hours(1),
                b: now,
                expected_str: "1 hour from now",
            },
            // multiple hours
            TimePair {
                a: now - Duration::hours(5),
                b: now,
                expected_str: "5 hours ago",
            },
            TimePair {
                a: now + Duration::hours(5),
                b: now,
                expected_str: "5 hours from now",
            },
            // singular day
            TimePair {
                a: now - Duration::days(1),
                b: now,
                expected_str: "1 day ago",
            },
            TimePair {
                a: now + Duration::days(1),
                b: now,
                expected_str: "1 day from now",
            },
            // multiple days
            TimePair {
                a: now - Duration::days(5),
                b: now,
                expected_str: "5 days ago",
            },
            TimePair {
                a: now + Duration::days(5),
                b: now,
                expected_str: "5 days from now",
            },
            // singular week
            TimePair {
                a: now - Duration::weeks(1),
                b: now,
                expected_str: "1 week ago",
            },
            TimePair {
                a: now + Duration::weeks(1),
                b: now,
                expected_str: "1 week from now",
            },
            // multiple weeks
            TimePair {
                a: now - Duration::weeks(3),
                b: now,
                expected_str: "3 weeks ago",
            },
            TimePair {
                a: now + Duration::weeks(3),
                b: now,
                expected_str: "3 weeks from now",
            },
            // singular month
            TimePair {
                a: now - Duration::days(30),
                b: now,
                expected_str: "1 month ago",
            },
            TimePair {
                a: now + Duration::days(30),
                b: now,
                expected_str: "1 month from now",
            },
            // multiple months
            TimePair {
                a: now - Duration::days(30 * 5),
                b: now,
                expected_str: "5 months ago",
            },
            TimePair {
                a: now + Duration::days(30 * 5),
                b: now,
                expected_str: "5 months from now",
            },
            // singular year
            TimePair {
                a: now - Duration::days(365),
                b: now,
                expected_str: "1 year ago",
            },
            TimePair {
                a: now + Duration::days(365),
                b: now,
                expected_str: "1 year from now",
            },
            // multiple years
            TimePair {
                a: now - Duration::days(365 * 5),
                b: now,
                expected_str: "5 years ago",
            },
            TimePair {
                a: now + Duration::days(365 * 5),
                b: now,
                expected_str: "5 years from now",
            },
            // long time
            TimePair {
                a: now - Duration::days(365 * 1000),
                b: now,
                expected_str: "a long while ago",
            },
            TimePair {
                a: now + Duration::days(365 * 1000),
                b: now,
                expected_str: "a long while from now",
            },
        ];

        for test in tests.iter() {
            let fmted = format_rel(test.a, test.b, "ago", "from now");
            assert_eq!(fmted, test.expected_str);
        }
    }
}
