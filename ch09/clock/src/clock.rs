use chrono::{DateTime, Local, TimeZone};

pub struct Clock;

impl Clock {
    pub fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(windows)]
    pub fn set<Tz: TimeZone>(t: DateTime<Tz>) {
        use std::mem::zeroed;

        use chrono::{Datelike, Local, Timelike, Weekday};
        use winapi::{
            shared::minwindef::WORD,
            um::{minwinbase::SYSTEMTIME, sysinfoapi::SetSystemTime},
        };

        let t = t.with_timezone(&Local);

        let mut systime = unsafe { zeroed::<SYSTEMTIME>() };

        let dow = match t.weekday() {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 0,
        };

        let mut ns = t.nanosecond();
        let is_leap_second = ns > 1_000_000_000;

        if is_leap_second {
            ns -= 1_000_000_000;
        }

        systime.wYear = t.year() as WORD;
        systime.wMonth = t.month() as WORD;
        systime.wDayOfWeek = dow as WORD;
        systime.wDay = t.day() as WORD;
        systime.wHour = t.hour() as WORD;
        systime.wMinute = t.minute() as WORD;
        systime.wSecond = t.second() as WORD;
        systime.wMilliseconds = (ns / 1_000_000) as WORD;

        let systime_ptr = &systime as *const SYSTEMTIME;
        unsafe {
            SetSystemTime(systime_ptr);
        }
    }

    #[cfg(not(windows))]
    pub fn set<Tz: TimeZone>(t: DateTime<Tz>) {
        use std::mem::zeroed;

        use libc::{settimeofday, suseconds_t, time_t, timeval};

        let t = t.with_timezone(&Local);
        let mut u = unsafe { zeroed::<timeval>() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }
}
