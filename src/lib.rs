use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, SecondsFormat, TimeZone, Utc};

fn fo_to_time_ms(date_time: &DateTime<FixedOffset>) -> i64 {
    (date_time.timestamp_nanos() + 500_000) / 1_000_000
}

fn time_ms_to_secs_nsecs(time_ms: i64) -> (i64, u32) {
    // println!("time_ms_to_secs_nsecs: + time_ms={}", time_ms);
    let mut secs = time_ms / 1000;
    let ms: u32 = if time_ms < 0 {
        // When time is less than zero the it's only negative
        // to the "epoch" thus seconds are "negative" but the
        // milli-seconds are positive. Thus -1ms is represented
        // in time as -1sec + 0.999ms. Sooooooo

        // First negate then modulo 1000 to get millis as a u32
        let mut millis = (-time_ms % 1_000) as u32;

        // This is very "likely" and it would be nice to be able
        // to tell the compiler with `if likely(millis > 0) {...}
        if millis > 0 {
            // We need to reduce secs by 1
            secs -= 1;

            // And map ms 1..999 to 999..1
            millis = 1_000 - millis;
            // println!("time_ms_to_secs_nsecs: adjusted   time_ms={} secs={} millis={}", time_ms, secs, millis);
        } else {
            // millis is 0 and secs is correct as is.
            // println!("time_ms_to_secs_nsecs: unadjusted time_ms={} secs={} millis={}", time_ms, secs, millis);
        }

        millis
    } else {
        // This actually caused clippy to output "unnecessarary `let` binding"
        // but for I want to be able to have the pritnln and I've found that
        // allowing unnecessary_cast suppresses the warning.
        #[allow(clippy::unnecessary_cast)]
        let millis = (time_ms % 1000) as u32;
        //println!("time_ms_to_secs_nsecs: unadjusted time_ms={} secs={} millis={}", time_ms, secs, millis);

        millis
    };

    let nsecs = ms * 1_000_000u32;

    // println!("time_ms_to_secs_nsecs: - time_ms={} secs={} nsecs={}", time_ms, secs, nsecs);
    (secs, nsecs)
}

pub fn time_ms_to_utc_string(time_ms: i64) -> String {
    time_ms_to_utc(time_ms).to_rfc3339_opts(SecondsFormat::Millis, false)
}

pub fn time_ms_to_utc_z_string(time_ms: i64) -> String {
    time_ms_to_utc(time_ms).to_rfc3339_opts(SecondsFormat::Millis, true)
}
/// Get Utc::now() and convert to time_ms
///
/// # Example
/// ```
/// use chrono::{DateTime, Utc};
/// use time_ms_conversions::utc_now_to_time_ms;
///
/// let before: i64 = Utc::now().timestamp_nanos() / 1_000_000;
///
/// assert!(utc_now_to_time_ms() >= before);
/// ```
pub fn utc_now_to_time_ms() -> i64 {
    (Utc::now().timestamp_nanos() + 500_000) / 1_000_000
}

/// Convert time_ms to DateTime<Utc>
///
/// # Example
/// ```
/// use chrono::{DateTime, Utc};
/// use time_ms_conversions::{utc_to_time_ms, time_ms_to_utc};
///
/// let epoch: DateTime<Utc> = time_ms_to_utc(0);
/// assert_eq!(utc_to_time_ms(&epoch), 0);
/// ```
pub fn time_ms_to_utc(time_ms: i64) -> DateTime<Utc> {
    let (secs, nsecs) = time_ms_to_secs_nsecs(time_ms);
    let naive_datetime = NaiveDateTime::from_timestamp(secs, nsecs);
    DateTime::from_utc(naive_datetime, Utc)
}

/// Convert a DateTime<Utc> to time_ms
///
/// # Examples
/// ```
/// use chrono::{DateTime, Utc};
/// use time_ms_conversions::time_ms_to_utc;
///
/// let dt: DateTime<Utc> = time_ms_to_utc(0);
/// assert_eq!(dt.to_string(), "1970-01-01 00:00:00 UTC");
/// ```
pub fn utc_to_time_ms(date_time: &DateTime<Utc>) -> i64 {
    (date_time.timestamp_nanos() + 500_000) / 1_000_000
}

pub enum TzMassaging {
    CondAddTzUtc,
    HasTz,
    LocalTz,
}

/// DateTime string converted to utc time_ms with either T or Space seperator
///
/// # Examples
/// ```
/// use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
/// use time_ms_conversions::{dt_str_to_utc_time_ms, TzMassaging};
///
/// // Might not have time zone and if not assume it's UTC
/// let str_time_no_ms = "1970-01-01 00:00:00";
/// let ts = dt_str_to_utc_time_ms(str_time_no_ms, TzMassaging::CondAddTzUtc)
///     .expect("Bad time format");
/// assert_eq!(ts, 0);
///
/// // If it does have UTC that's fine too
/// let str_time_no_ms = "1970-01-01 00:00:00+00:00";
/// let ts = dt_str_to_utc_time_ms(str_time_no_ms, TzMassaging::CondAddTzUtc)
///     .expect("Bad time format");
/// assert_eq!(ts, 0);
///
/// // And CondAddTzUtz handles other time zones and converts them to Utc
/// let str_time_pst = "1969-12-31T16:00:00-0800";
/// let ts_utc = dt_str_to_utc_time_ms(str_time_pst, TzMassaging::CondAddTzUtc)
///     .expect("Bad time format with milliseconds");
/// assert_eq!(ts_utc, 0);
///
/// // Assume there is a timezone, this is more efficient then CondAddTzUtc
/// let str_time_tz = "1970-01-01 00:00:00+0000";
/// let ts = dt_str_to_utc_time_ms(str_time_tz, TzMassaging::HasTz).expect("Bad time format");
/// assert_eq!(ts, 0);
///
/// // UTC time_ms 0 as a Date Time string without time zone
/// let utc_time_ms_0_str = "1970-01-01T00:00:00";
///
/// // Get a NaiveDateTime
/// let ndt: NaiveDateTime = utc_time_ms_0_str.parse().unwrap();
///
/// // Convert to the local time of where ever this is running
/// let ldt = Local.from_local_datetime(&ndt).unwrap();
///
/// // Get the offset from local time to utc, a negative value for west of UTC
/// // This will be -08:00 for PST
/// let ldt_offset = ldt.offset();
/// dbg!(ldt_offset);
///
/// // Get the value to add to UTC to convert to local time
/// // This will be -28800000 = (-8 * 60 * 60 * 1000) for PST
/// let milli_seconds_to_add_to_convert_to_utc = ldt_offset.local_minus_utc() as i64 * 1000;
/// dbg!(milli_seconds_to_add_to_convert_to_utc);
///
/// // Adding milli_seconds_to_add_to_convert_to_utc to tms will equal 0 for all timezones
/// // For PST tms is 28800000 and adding -28800000 is 0
/// let tms = dt_str_to_utc_time_ms(utc_time_ms_0_str, TzMassaging::LocalTz)
///     .expect("Bad time format");
/// assert_eq!(tms + milli_seconds_to_add_to_convert_to_utc, 0);
/// ```
pub fn dt_str_to_utc_time_ms(
    dt_str: &str,
    tz_massaging: TzMassaging,
) -> Result<i64, Box<dyn std::error::Error>> {
    pub fn dt_str_with_fmt_str_to_utc_time_ms(
        dt_str: &str,
        fmt_str: &str,
        tz_massaging: TzMassaging,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let dt_str = dt_str.trim();
        match tz_massaging {
            TzMassaging::HasTz => {
                let fs = format!("{fmt_str}%#z");
                let dtfo = DateTime::parse_from_str(dt_str, &fs)?;
                Ok(fo_to_time_ms(&dtfo))
            }
            TzMassaging::CondAddTzUtc => {
                let fs = format!("{fmt_str}%#z");

                // If there is a '+' then there "must be" a time zone
                let has_pos_tz = dt_str.matches('+').count() > 0;

                // If there is a '-' after the "year" then there must be a time zone
                let mut rmtchr = dt_str.rmatch_indices('-');
                let first_rmatch = rmtchr.next();
                let has_neg_tz = if let Some((idx, _s)) = first_rmatch {
                    // If there is a '-' after index 7 then assume there is a negative time zone
                    //     2020-01-01T...
                    //     01234567
                    idx > 7
                } else {
                    // No numeric timezone
                    false
                };

                let s = if !has_pos_tz && !has_neg_tz {
                    // Add numeric timezone for UTC
                    format!("{dt_str}+0000")
                } else {
                    // Else there is one so just convert dt_str to String
                    dt_str.to_string()
                };
                let dtfo = DateTime::parse_from_str(&s, &fs)?;
                Ok(fo_to_time_ms(&dtfo))
            }
            TzMassaging::LocalTz => {
                // Convert datetime string to DateTime<Local>
                // from: https://stackoverflow.com/questions/65820170/parsing-a-datetime-string-to-local-time-in-rust-chrono?rq=1
                let ndt = NaiveDateTime::parse_from_str(dt_str, fmt_str)?;
                let ldt = match Local.from_local_datetime(&ndt) {
                    chrono::LocalResult::None => {
                        return Err("No result".into());
                    }
                    chrono::LocalResult::Single(dt) => dt,
                    chrono::LocalResult::Ambiguous(_, _) => {
                        return Err("Ambigious result".into());
                    }
                };

                // Convert from DateTime<Local> to DateTime<Utc> with timezone information
                // from: https://stackoverflow.com/questions/56887881/how-do-i-convert-a-chrono-datetimelocal-instance-to-datetimeutc
                let dt_utc = ldt.with_timezone(&Utc);

                Ok(utc_to_time_ms(&dt_utc))
            }
        }
    }

    let tms = if dt_str.matches('T').count() == 1 {
        dt_str_with_fmt_str_to_utc_time_ms(dt_str, "%Y-%m-%dT%H:%M:%S%.f", tz_massaging)?
    } else {
        dt_str_with_fmt_str_to_utc_time_ms(dt_str, "%Y-%m-%d %H:%M:%S%.f", tz_massaging)?
    };

    Ok(tms)
}

#[cfg(test)]
mod test {
    use chrono::SecondsFormat;

    use super::*;
    use std::time::Instant;

    #[test]
    fn test_time_ms_to_secs_nsecs() {
        assert_eq!(time_ms_to_secs_nsecs(-2001), (-3i64, 999_000_000u32));
        assert_eq!(time_ms_to_secs_nsecs(-2000), (-2i64, 0u32));
        //assert_eq!(time_ms_to_secs_nsecs(-2000), (-3i64, 1_000_000_000u32)); // No Adjustment
        assert_eq!(time_ms_to_secs_nsecs(-1999), (-2i64, 1_000_000u32));
        assert_eq!(time_ms_to_secs_nsecs(-1001), (-2i64, 999_000_000u32));
        assert_eq!(time_ms_to_secs_nsecs(-1000), (-1i64, 0u32));
        //assert_eq!(time_ms_to_secs_nsecs(-1000), (0i64, 1_000_000_000u32)); // No adjustment
        assert_eq!(time_ms_to_secs_nsecs(-999), (-1i64, 1_000_000u32));
        assert_eq!(time_ms_to_secs_nsecs(-1), (-1i64, 999_000_000u32));
        assert_eq!(time_ms_to_secs_nsecs(0), (0i64, 0u32));
        assert_eq!(time_ms_to_secs_nsecs(1), (0i64, 1_000_000u32));
        assert_eq!(time_ms_to_secs_nsecs(999), (0i64, 999_000_000u32));
        assert_eq!(time_ms_to_secs_nsecs(1000), (1i64, 0u32));
    }

    #[test]
    fn test_utc_now_to_time_ms() {
        let start = Instant::now();

        // Because we use integer arithmetic we must
        // see 2 milli-second time ticks to see a minimum
        // duration of > 1ms.
        let tms1 = utc_now_to_time_ms();
        let mut tms2 = tms1;
        while tms2 < (tms1 + 2) {
            tms2 = utc_now_to_time_ms();
        }
        let done = Instant::now();
        let duration = done.duration_since(start);

        println!(
            "tms1: {} tms2: {} done: {:?} - start {:?} = {}ns or {}ms",
            tms1,
            tms2,
            done,
            start,
            duration.as_nanos(),
            duration.as_millis()
        );

        assert!(tms2 >= (tms1 + 2));
        assert!(duration.as_millis() >= 1);

        // The duration.as_millis should be < 2ms. But with Tarpaulin
        // I've seen durations over 4ms so we skip this test.
        // assert!(duration.as_millis() < 2);
    }

    #[test]
    fn test_dt_str_with_tee_to_utc_time_ms() {
        let str_time_no_ms = "1970-01-01T00:00:00";
        let ts = dt_str_to_utc_time_ms(str_time_no_ms, TzMassaging::CondAddTzUtc)
            .expect("Bad time format");
        dbg!(ts);
        assert_eq!(ts, 0);

        let str_time_with_ms = "1970-01-01T00:00:00.123";
        let tms = dt_str_to_utc_time_ms(str_time_with_ms, TzMassaging::CondAddTzUtc)
            .expect("Bad time format with milliseconds");
        dbg!(tms);
        assert_eq!(tms, 123);
    }

    #[test]
    fn test_dt_str_with_space_to_utc_time_ms() {
        let str_time_no_ms = "1970-01-01 00:00:00";
        let ts = dt_str_to_utc_time_ms(str_time_no_ms, TzMassaging::CondAddTzUtc)
            .expect("Bad time format");
        dbg!(ts);
        assert_eq!(ts, 0);

        let str_time_with_ms = "1970-01-01 00:00:00.123";
        let tms = dt_str_to_utc_time_ms(str_time_with_ms, TzMassaging::CondAddTzUtc)
            .expect("Bad time format with milliseconds");
        dbg!(tms);
        assert_eq!(tms, 123);
    }

    #[test]
    fn test_dt_str_with_leading_trailing_spaces_to_utc_time_ms() {
        let str_time_no_ms = " 1970-01-01 00:00:00";
        let ts = dt_str_to_utc_time_ms(str_time_no_ms, TzMassaging::CondAddTzUtc)
            .expect("Bad time format");
        dbg!(ts);
        assert_eq!(ts, 0);

        let str_time_with_ms = "1970-01-01 00:00:00.123 ";
        let tms = dt_str_to_utc_time_ms(str_time_with_ms, TzMassaging::CondAddTzUtc)
            .expect("Bad time format with milliseconds");
        dbg!(tms);
        assert_eq!(tms, 123);
        let str_time_no_ms = " 1970-01-01T00:00:00  ";
        let ts = dt_str_to_utc_time_ms(str_time_no_ms, TzMassaging::CondAddTzUtc)
            .expect("Bad time format");
        dbg!(ts);
        assert_eq!(ts, 0);

        let str_time_with_ms = "  1970-01-01T00:00:00.123  ";
        let tms = dt_str_to_utc_time_ms(str_time_with_ms, TzMassaging::CondAddTzUtc)
            .expect("Bad time format with milliseconds");
        dbg!(tms);
        assert_eq!(tms, 123);
    }

    #[test]
    fn test_dt_str_addtzutc_with_utc() {
        let str_time_tz = "1970-01-01 00:00:00+00";
        let ts =
            dt_str_to_utc_time_ms(str_time_tz, TzMassaging::CondAddTzUtc).expect("Bad time format");
        dbg!(ts);
        assert_eq!(ts, 0);
        let str_time_tz = "1970-01-01T00:00:00.1+00";
        let ts =
            dt_str_to_utc_time_ms(str_time_tz, TzMassaging::CondAddTzUtc).expect("Bad time format");
        dbg!(ts);
        assert_eq!(ts, 100);

        let str_time_tz = "1970-01-01T00:00:00.123+0000";
        let ts =
            dt_str_to_utc_time_ms(str_time_tz, TzMassaging::CondAddTzUtc).expect("Bad time format");
        dbg!(ts);
        assert_eq!(ts, 123);

        let str_time_tz = "1970-01-01 00:00:00+00:00";
        let ts =
            dt_str_to_utc_time_ms(str_time_tz, TzMassaging::CondAddTzUtc).expect("Bad time format");
        dbg!(ts);
        assert_eq!(ts, 0);

        let str_time_tz = "1970-01-01 00:00:00.456+00:00";
        let ts =
            dt_str_to_utc_time_ms(str_time_tz, TzMassaging::CondAddTzUtc).expect("Bad time format");
        dbg!(ts);
        assert_eq!(ts, 456);
    }

    #[test]
    fn test_dt_str_with_tz_to_utc_time_ms() {
        let str_time_no_ms = "1970-01-01T00:00:00+0000";
        let ts =
            dt_str_to_utc_time_ms(str_time_no_ms, TzMassaging::HasTz).expect("Bad time format");
        dbg!(ts);
        assert_eq!(ts, 0);

        let str_time_with_ms = "1970-01-01T00:00:00.123+00:00";
        let tms = dt_str_to_utc_time_ms(str_time_with_ms, TzMassaging::HasTz)
            .expect("Bad time format with milliseconds");
        dbg!(tms);
        assert_eq!(tms, 123);
    }

    #[test]
    fn test_dt_str_both_hastz() {
        let str_time_tz = "1970-01-01T00:00:00+0000";
        let ts = dt_str_to_utc_time_ms(str_time_tz, TzMassaging::HasTz).expect("Bad time format");
        dbg!(ts);
        assert_eq!(ts, 0);

        let str_time_pst = "1969-12-31T16:00:00-0800";
        let ts_pst = dt_str_to_utc_time_ms(str_time_pst, TzMassaging::HasTz)
            .expect("Bad time format with milliseconds");
        dbg!(ts_pst);
        assert_eq!(ts, ts_pst);
    }

    #[test]
    fn test_dt_str_addtzutc_hastz() {
        let str_time_tz = "1970-01-01T00:00:00";
        let ts =
            dt_str_to_utc_time_ms(str_time_tz, TzMassaging::CondAddTzUtc).expect("Bad time format");
        dbg!(ts);
        assert_eq!(ts, 0);

        let str_time_pst = "1969-12-31T16:00:00-0800";
        let ts_pst = dt_str_to_utc_time_ms(str_time_pst, TzMassaging::HasTz)
            .expect("Bad time format with milliseconds");
        dbg!(ts_pst);
        assert_eq!(ts, ts_pst);
    }

    #[test]
    fn test_dt_str_to_utc_time_ms_using_localtz() {
        // UTC time_ms 0 as a Date Time string without time zone
        let utc_time_ms_0_str = "1970-01-01T00:00:00";

        // Get a NaiveDateTime
        let ndt: NaiveDateTime = utc_time_ms_0_str.parse().unwrap();
        dbg!(ndt);

        // Convert to local time so at PST this is 1970-10-01T00:00:00-08:00
        let ldt = Local.from_local_datetime(&ndt).unwrap();
        dbg!(ldt);

        // Get the offset from local time to utc, a negative value for west of UTC
        // This will be -08:00 for PST
        let ldt_offset = ldt.offset();
        dbg!(ldt_offset);

        // Get the value to add to UTC to convert to local time
        // This will be -28800000 = (-8 * 60 * 60 * 1000) for PST
        let milli_seconds_to_add_to_convert_to_utc = ldt_offset.local_minus_utc() as i64 * 1000;
        dbg!(milli_seconds_to_add_to_convert_to_utc);

        // Adding milli_seconds_to_add_to_convert_to_utc to tms will equal 0 for all timezones
        // For PST tms is 28800000 and adding -28800000 is 0
        let tms = dt_str_to_utc_time_ms(utc_time_ms_0_str, TzMassaging::LocalTz)
            .expect("Bad time format");
        dbg!(tms);
        assert_eq!(tms + milli_seconds_to_add_to_convert_to_utc, 0);
    }

    #[test]
    fn test_time_ms_to_utc() {
        let dt = time_ms_to_utc(0i64);
        assert_eq!(
            dt.to_rfc3339_opts(SecondsFormat::Millis, true),
            "1970-01-01T00:00:00.000Z"
        );
        assert_eq!(
            dt.to_rfc3339_opts(SecondsFormat::Millis, false),
            "1970-01-01T00:00:00.000+00:00"
        );
    }

    #[test]
    fn test_time_ms_to_utc_string() {
        let dt = time_ms_to_utc_string(0i64);
        assert_eq!(dt, "1970-01-01T00:00:00.000+00:00");
    }

    #[test]
    fn test_time_ms_to_utc_z_string() {
        let dt = time_ms_to_utc_z_string(0i64);
        assert_eq!(dt, "1970-01-01T00:00:00.000Z");
    }

    #[test]
    fn test_date_time_parse_from_rfc3339() {
        let s = format!("1970-01-01T00:00:00.000{}", "Z");
        let dt = match DateTime::parse_from_rfc3339(&s) {
            Ok(v) => v,
            Err(e) => panic!("shit {e}"),
        };
        println!("test_date_teim_parse_from_rfc3339: {dt}");
    }
}
