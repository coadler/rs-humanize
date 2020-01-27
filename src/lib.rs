#![feature(duration_constants)]
pub mod time;

#[cfg(test)]
mod tests {
    use chrono::Utc;

    #[test]
    fn format_now() {
        let fmted = super::time::format(Utc::now());
        assert_eq!("now", fmted);
    }
}
