#![feature(duration_constants)]
pub mod time;

#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    #[test]
    fn format_now() {
        let d = Utc::now();
        let fmted = super::time::format(d);
        println!("{}, {}", fmted, d.to_string());
        assert_eq!("now", fmted);
    }
}
