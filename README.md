# rs-humanize

[![Crates.io](https://img.shields.io/crates/v/rs-humanize.svg)](https://crates.io/crates/rs-humanize)
[![docs.rs badge](https://docs.rs/rs-humanize/badge.svg)](https://docs.rs/rs-humanize)

This is a Rust port of [go-humanize](https://github.com/dustin/go-humanize).
Right now only `humanize.Time` is ported over but I plan to do the rest.

## Time

This lets you take a `DateTime<Utc>` and spit it out in relative terms. For
example, `12 seconds ago` or `3 days from now`.

Code example:
```rust
use rs_humanize::time;
use chrono::Utc;

println!("{}", time::format(Utc::now()));
println!("{}", time::format(Utc.ymd(2018, 2, 18).and_hms(8, 30, 0)));
```
