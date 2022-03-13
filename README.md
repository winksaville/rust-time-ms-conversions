# Time Conversion library
[![codecov](https://codecov.io/gh/winksaville/time-ms-conversion/branch/main/graph/badge.svg?token=cowZtK1KK1)](https://codecov.io/gh/winksaville/time-ms-conversion)
Various routines to convert time_ms to and from UTC

```
pub fn time_ms_to_utc(time_ms: i64) -> DateTime<Utc>
pub fn time_ms_utc_to_naive_local(time_ms: i64) -> NaiveDateTime
pub fn time_ms_to_utc_string(time_ms: i64) -> String
pub fn utc_now_to_time_ms() -> i64
pub fn utc_to_time_ms(date_time: &DateTime<Utc>) -> i64
pub fn fo_to_time_ms(date_time: &DateTime<FixedOffset>) -> i64

pub enum TzMassaging {
    CondAddTzUtc,
    HasTz,
    LocalTz,
}

///! DateTime string converted to utc time_ms with either T or Space seperator
pub fn dt_str_to_utc_time_ms(
    dt_str: &str,
    tz_massaging: TzMassaging,
) -> Result<i64, Box<dyn std::error::Error>>
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
