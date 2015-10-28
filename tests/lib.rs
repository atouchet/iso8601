extern crate iso8601;
extern crate nom;

use nom::IResult::*;

use iso8601::*;

#[test]
fn parse_year() {
    assert_eq!(Done(&[][..], 2015), year(b"2015"));
    assert_eq!(Done(&[][..], -0333), year(b"-0333"));

    assert_eq!(Done(&b"-"[..], 2015), year(b"2015-"));

    assert!(year(b"abcd").is_err());
    assert!(year(b"2a03").is_err());
}

#[test]
fn parse_month() {
    assert_eq!(Done(&[][..], 1), month(b"01"));
    assert_eq!(Done(&[][..], 6), month(b"06"));
    assert_eq!(Done(&[][..], 12), month(b"12"));
    assert_eq!(Done(&b"-"[..], 12), month(b"12-"));

    assert!(month(b"13").is_err());
    assert!(month(b"00").is_err());
}

#[test]
fn parse_day() {
    assert_eq!(Done(&[][..], 1),  day(b"01"));
    assert_eq!(Done(&[][..], 12), day(b"12"));
    assert_eq!(Done(&[][..], 20), day(b"20"));
    assert_eq!(Done(&[][..], 28), day(b"28"));
    assert_eq!(Done(&[][..], 30), day(b"30"));
    assert_eq!(Done(&[][..], 31), day(b"31"));
    assert_eq!(Done(&b"-"[..], 31), day(b"31-"));

    assert!(day(b"00").is_err());
    assert!(day(b"32").is_err());
}

//#[test]
//fn parse_week() {
//    assert_eq!(Done(&[][..],  1), month(b"01"));
//    assert!(month(b"1").is_err());
//    assert_eq!(Done(&[][..],  6), month(b"06"));
//    assert_eq!(Done(&[][..], 12), month(b"12"));
//}

#[test]
fn parse_date() {
    assert_eq!(Done(&[][..], Date::YMD{ year: 2015, month: 6, day: 26 }), date(b"2015-06-26"));
    assert_eq!(Done(&[][..], Date::YMD{ year: -333, month: 7, day: 11 }), date(b"-0333-07-11"));

    assert!(date(b"201").is_incomplete());
    assert!(date(b"2015p00p00").is_err());
    assert!(date(b"pppp").is_err());
}

#[test]
fn parse_hour() {
    assert_eq!(Done(&[][..], 0), hour(b"00"));
    assert_eq!(Done(&[][..], 1), hour(b"01"));
    assert_eq!(Done(&[][..], 6), hour(b"06"));
    assert_eq!(Done(&[][..], 12), hour(b"12"));
    assert_eq!(Done(&[][..], 13), hour(b"13"));
    assert_eq!(Done(&[][..], 20), hour(b"20"));
    assert_eq!(Done(&[][..], 24), hour(b"24"));

    assert!(hour(b"25").is_err());
    assert!(hour(b"30").is_err());
    assert!(hour(b"ab").is_err());
}

#[test]
fn parse_minute() {
    assert_eq!(Done(&[][..], 0), minute(b"00"));
    assert_eq!(Done(&[][..], 1), minute(b"01"));
    assert_eq!(Done(&[][..], 30), minute(b"30"));
    assert_eq!(Done(&[][..], 59), minute(b"59"));

    assert!(minute(b"60").is_err());
    assert!(minute(b"61").is_err());
    assert!(minute(b"ab").is_err());
}

#[test]
fn parse_second() {
    assert_eq!(Done(&[][..], 0), second(b"00"));
    assert_eq!(Done(&[][..], 1), second(b"01"));
    assert_eq!(Done(&[][..], 30), second(b"30"));
    assert_eq!(Done(&[][..], 59), second(b"59"));
    assert_eq!(Done(&[][..], 60), second(b"60"));

    assert!(second(b"61").is_err());
    assert!(second(b"ab").is_err());
}

#[test]
fn parse_time() {
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 0}), time(b"16:43:16"));
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second:  0, tz_offset: 0}), time(b"16:43"));

    assert!(time(b"20:").is_incomplete());
    assert!(time(b"20p42p16").is_err());
    assert!(time(b"pppp").is_err());
}

#[test]
fn parse_time_with_timezone() {
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 0}),       time_with_timezone(b"16:43:16"));
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 0}),       time_with_timezone(b"16:43:16Z"));
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 0}),       time_with_timezone(b"16:43:16+00:00"));
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 0}),       time_with_timezone(b"16:43:16-00:00"));
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 5*60*60}), time_with_timezone(b"16:43:16+05:00"));
    assert_eq!(Done(&b"+"[..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 0}),     time_with_timezone(b"16:43:16+"));
    assert_eq!(Done(&b"+0"[..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 0}),    time_with_timezone(b"16:43:16+0"));
    assert_eq!(Done(&b"+05:"[..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 0}),  time_with_timezone(b"16:43:16+05:"));

    assert!(time_with_timezone(b"20:").is_incomplete());
    assert!(time_with_timezone(b"20p42p16").is_err());
    assert!(time_with_timezone(b"pppp").is_err());

}

#[test]
fn parse_iso_week_date() {
    assert_eq!(Done(&[][..], Date::YWeek{ year: 2015,ww:5, d: 7 }),
                                iso_week_date(b"2015-W05-7"));
    assert_eq!(Done(&[][..], Date::YWeek{ year: 2015,ww:6, d: 6 }),
                                iso_week_date(b"2015-W06-6"));
    assert!( iso_week_date(b"2015-W06-8").is_err());
    assert!( iso_week_date(b"2015-W06-0").is_err());
    assert!( iso_week_date(b"2015-W00-2").is_err());
    assert!( iso_week_date(b"2015-W54-2").is_err());
}

#[test]
fn parse_datetime_correct() {

    // TODO add milliseconds format to seconds (SS.mmm)
    let test_datetimes = vec![
        ("20060831T16:47+00:00",       DateTime{ date: Date::YMD  { year: 2006,  month:08,  day:31},  time: Time{ hour: 16,  minute:47,  second:0,   tz_offset:0*3600}}),
        ("2007-08-31T16:47+00:00",     DateTime{ date: Date::YMD  { year: 2007,  month:08,  day:31},  time: Time{ hour: 16,  minute:47,  second:0,   tz_offset:0*3600}}),
        ("20070831T1647+00:00",        DateTime{ date: Date::YMD  { year: 2007,  month:08,  day:31},  time: Time{ hour: 16,  minute:47,  second:0,   tz_offset:0*3600}}),
        ("20070831T1647+0000",         DateTime{ date: Date::YMD  { year: 2007,  month:08,  day:31},  time: Time{ hour: 16,  minute:47,  second:0,   tz_offset:0*3600}}),
        ("20070831T1647Z",             DateTime{ date: Date::YMD  { year: 2007,  month:08,  day:31},  time: Time{ hour: 16,  minute:47,  second:0,   tz_offset:0*3600}}),
        ("2008-12-24T18:21Z",          DateTime{ date: Date::YMD  { year: 2008,  month:12,  day:24},  time: Time{ hour: 18,  minute:21,  second:0,   tz_offset:0*3600}}),
        ("2009-02-01T09:00:22+05",     DateTime{ date: Date::YMD  { year: 2009,  month:02,  day:01},  time: Time{ hour: 9,   minute:0,   second:22,  tz_offset:5*3600}}),
        ("2010-01-01T12:00:00+01:00",  DateTime{ date: Date::YMD  { year: 2010,  month:1,   day:1},   time: Time{ hour: 12,  minute:0,   second:0,   tz_offset:1*3600}}),
        ("2011-06-30T18:30:00+02:00",  DateTime{ date: Date::YMD  { year: 2011,  month:06,  day:30},  time: Time{ hour: 18,  minute:30,  second:0,   tz_offset:2*3600}}),
        ("2015-06-29T23:07+02:00",     DateTime{ date: Date::YMD  { year: 2015,  month:06,  day:29},  time: Time{ hour: 23,  minute:07,  second:0,   tz_offset:2*3600}}),
        ("2015-06-26T16:43:16",        DateTime{ date: Date::YMD  { year: 2015,  month:06,  day:26},  time: Time{ hour: 16,  minute:43,  second:16,  tz_offset:0*3600}}),
        ("2015-06-26T16:43:16",        DateTime{ date: Date::YMD  { year: 2015,  month:06,  day:26},  time: Time{ hour: 16,  minute:43,  second:16,  tz_offset:0*3600}}),
        ("2015-W05-6T04:05:06+07:00",  DateTime{ date: Date::YWeek{ year: 2015,  ww:05,  d:6},      time: Time{ hour: 04,  minute:5,  second:6,  tz_offset:7*3600}})
    ];

    for (iso_string, data) in test_datetimes {
        assert_eq!(Done(&[][..], data), datetime(iso_string.as_bytes()));
    }
}

#[test]
fn parse_datetime_error() {
    let test_datetimes = vec![
        "ppp",
        "dumd-di-duTmd:iu:m"
    ];

    for iso_string in test_datetimes {
        let res = datetime(iso_string.as_bytes());
        assert!(res.is_err() || res.is_incomplete());
    }
}

#[test]
fn disallows_notallowed() {
    assert!(time(b"30:90:90").is_err());
    assert!(date(b"0000-20-40").is_err());
}
