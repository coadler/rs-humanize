use chrono::prelude::*;
use std::collections::HashMap;
use std::time::Duration;
use strfmt::strfmt;

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

#[inline]
pub fn format(then: DateTime<Utc>) -> String {
    let now = Utc::now();
    return format_rel(then, now, "ago", "from now");
}

#[inline]
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

    let mut vars = HashMap::with_capacity(2);
    vars.insert("amt".to_string(), amt_str);
    vars.insert("label".to_string(), label.to_owned());

    strfmt(magnitude.fmt, &vars).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_rel() {
        use chrono::Duration;

        struct TimeTest(DateTime<Utc>, &'static str);

        let now = Utc::now();
        let tests = vec![
            // singular second
            TimeTest(now - Duration::seconds(1), "1 second ago"),
            TimeTest(now + Duration::seconds(1), "1 second from now"),
            // multiple seconds
            TimeTest(now - Duration::seconds(5), "5 seconds ago"),
            TimeTest(now + Duration::seconds(5), "5 seconds from now"),
            // singular minute
            TimeTest(now - Duration::minutes(1), "1 minute ago"),
            TimeTest(now + Duration::minutes(1), "1 minute from now"),
            // multiple minutes
            TimeTest(now - Duration::minutes(5), "5 minutes ago"),
            TimeTest(now + Duration::minutes(5), "5 minutes from now"),
            // singular hour
            TimeTest(now - Duration::hours(1), "1 hour ago"),
            TimeTest(now + Duration::hours(1), "1 hour from now"),
            // multiple hours
            TimeTest(now - Duration::hours(5), "5 hours ago"),
            TimeTest(now + Duration::hours(5), "5 hours from now"),
            // singular day
            TimeTest(now - Duration::days(1), "1 day ago"),
            TimeTest(now + Duration::days(1), "1 day from now"),
            // multiple days
            TimeTest(now - Duration::days(5), "5 days ago"),
            TimeTest(now + Duration::days(5), "5 days from now"),
            // singular week
            TimeTest(now - Duration::weeks(1), "1 week ago"),
            TimeTest(now + Duration::weeks(1), "1 week from now"),
            // multiple weeks
            TimeTest(now - Duration::weeks(3), "3 weeks ago"),
            TimeTest(now + Duration::weeks(3), "3 weeks from now"),
            // singular month
            TimeTest(now - Duration::days(30), "1 month ago"),
            TimeTest(now + Duration::days(30), "1 month from now"),
            // multiple months
            TimeTest(now - Duration::days(30 * 5), "5 months ago"),
            TimeTest(now + Duration::days(30 * 5), "5 months from now"),
            // singular year
            TimeTest(now - Duration::days(365), "1 year ago"),
            TimeTest(now + Duration::days(365), "1 year from now"),
            // multiple years
            TimeTest(now - Duration::days(365 * 5), "5 years ago"),
            TimeTest(now + Duration::days(365 * 5), "5 years from now"),
            // long time
            TimeTest(now - Duration::days(365 * 1000), "a long while ago"),
            TimeTest(now + Duration::days(365 * 1000), "a long while from now"),
        ];

        for test in tests.iter() {
            let fmted = format_rel(test.0, now, "ago", "from now");
            assert_eq!(fmted, test.1);
        }
    }

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_format_rel(b: &mut Bencher) {
        use chrono::Duration;

        let now = Utc::now();
        let then = now - Duration::days(30);

        b.iter(|| format_rel(then, now, "ago", "from now"))
    }
}
