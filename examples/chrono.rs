// If use Naives, have to add "features = ["alloc"]" in "Cargo.toml"
use chrono::{prelude::*, Months, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
fn main() {
    // Get DateTime as now
    let utc_now: DateTime<Utc> = Utc::now();             // DateTime of UTC
    let ndt_now: NaiveDateTime = Utc::now().naive_utc(); // DateTime without timezone

    // Get DateTime as you want
    let utc_dt1: DateTime<Utc> = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let nd_date: NaiveDate = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();  // Only date type
    let nt_time: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();     // Only time type
    let ndt_dt1: NaiveDateTime = NaiveDateTime::new(nd_date, nt_time);

    // Cast each other type
    let _utc2ndt: NaiveDateTime = utc_now.naive_utc();
    let _utc2nd: NaiveDate = utc_now.date_naive();
    let _utc2nt: NaiveTime = utc_now.time();
    let _ndt2utc: DateTime<Utc> = ndt_now.and_utc();
    let _ndt2nd: NaiveDate = ndt_now.date();
    let _ndt2nt: NaiveTime = ndt_now.time();
    let utc2unix: i64 = utc_now.timestamp();
    let _unix2utc: DateTime<Utc> = DateTime::from_timestamp(utc2unix, 0).unwrap();

    // Get elements of DateTime (available both utc & ndt)
    assert_eq!(utc_dt1.year(), 2024);
    assert_eq!(utc_dt1.month(), 1);
    assert_eq!(utc_dt1.day(), 1); // else methods ... hour, minute, second, nanosecond

    // Partially change (available both utc & ndt)
    let utc_dt2: DateTime<Utc> = utc_dt1.with_month(3).unwrap();
    let ndt_dt2: NaiveDateTime = ndt_dt1.with_day(30).unwrap(); // else ... with_year, with_hour, etc.

    // Get difference of DateTime
    let time_df1: TimeDelta = utc_dt2 - utc_dt1; // and "ndt - ndt" can also
    let time_df2: TimeDelta = TimeDelta::days(2_i64);
    let time_df3: TimeDelta = TimeDelta::hours(-24_i64);
    assert_eq!(time_df1.num_days(), 60);
    assert_eq!(time_df2.num_hours(), 48);

    // Get Months difference
    let month_df: Months = Months::new(2_u32);
    assert_eq!(month_df.as_u32(), 2);

    // Increase or decrease DateTime
    let _utc_dt3: DateTime<Utc> = utc_dt1 + time_df3;
    let _ndt_dt3: NaiveDateTime = ndt_dt2 - month_df;

    // Get Timezone pattern 1 (https://docs.rs/chrono-tz/latest/chrono_tz/Asia/index.html)
    use chrono_tz::Asia::Tokyo;
    let tz1: Tz = Tokyo;

    // Get Timezone pattern 2 (https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html)
    use chrono_tz::Tz;
    let _tz2: Tz = Tz::Asia__Tokyo;
    let _tz3: Tz = "Asia/Tokyo".parse::<chrono_tz::Tz>().unwrap();

    // Cange Timezone
    let _tokyo: DateTime<Tz> = utc_dt1.with_timezone(&tz1);
    let _local: NaiveDateTime = utc_dt1.naive_local(); // to "Local" timezone

    // Cast from/to String (https://docs.rs/chrono/latest/chrono/format/strftime/index.html)
    let _str2utc: DateTime<Utc> = DateTime::parse_from_str("2024-01-01T00:00:00Z", "%Y-%m-%dT%H:%M:%S%#z").unwrap().to_utc();
    let _ = "2024-01-01T00:00:00+09:00".parse::<DateTime<Utc>>().unwrap();
    let _utc2str: String = utc_dt1.format("%Y/%m/%dT%H:%M:%S%z").to_string();
    let _ = utc_dt1.to_string();

    // Other things
    let _today: DateTime<Tz> = Utc::now().with_timezone(&tz1).with_time(NaiveTime::default()).unwrap();
    println!("{}", DateTime::<Utc>::default()); // 1970-01-01 00:00:00 UTC
    assert!(!time_df1.is_zero());
}