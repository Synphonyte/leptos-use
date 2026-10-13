use super::CalendarDateType;
use chrono::{DateTime, Datelike, FixedOffset, Local, Months, NaiveDate, TimeZone, Utc};

impl CalendarDateType for NaiveDate {
    fn today() -> Self {
        Local::now().date_naive()
    }

    fn ymd(&self) -> (i32, u8, u8) {
        (self.year(), self.month() as u8, self.day() as u8)
    }

    fn prev_day(&self) -> Option<Self> {
        self.pred_opt()
    }

    fn next_day(&self) -> Option<Self> {
        self.succ_opt()
    }

    fn weekday_from_monday(&self) -> u8 {
        self.weekday().num_days_from_monday() as u8
    }

    fn add_months(&self, months: i32) -> Option<Self> {
        if months >= 0 {
            self.checked_add_months(Months::new(months as u32))
        } else {
            self.checked_sub_months(Months::new(months.unsigned_abs()))
        }
    }
}

/// Replaces the local date of `date_time` by applying `f` to it while keeping the wall clock time.
/// If that time doesn't exist on the new date (DST gap) noon is used instead.
fn map_local_date<Tz: TimeZone>(
    date_time: &DateTime<Tz>,
    f: impl FnOnce(NaiveDate) -> Option<NaiveDate>,
) -> Option<DateTime<Tz>> {
    let naive = date_time.naive_local();
    let date = f(naive.date())?;
    let tz = date_time.timezone();

    tz.from_local_datetime(&date.and_time(naive.time()))
        .earliest()
        .or_else(|| {
            tz.from_local_datetime(&date.and_hms_opt(12, 0, 0)?)
                .earliest()
        })
}

macro_rules! impl_calendar_date_type_for_date_time {
    ($tz:ty, $today:expr) => {
        impl CalendarDateType for DateTime<$tz> {
            fn today() -> Self {
                $today
            }

            fn ymd(&self) -> (i32, u8, u8) {
                (self.year(), self.month() as u8, self.day() as u8)
            }

            fn prev_day(&self) -> Option<Self> {
                map_local_date(self, |date| date.prev_day())
            }

            fn next_day(&self) -> Option<Self> {
                map_local_date(self, |date| date.next_day())
            }

            fn weekday_from_monday(&self) -> u8 {
                self.weekday().num_days_from_monday() as u8
            }

            fn add_months(&self, months: i32) -> Option<Self> {
                map_local_date(self, |date| date.add_months(months))
            }
        }
    };
}

impl_calendar_date_type_for_date_time!(Utc, Utc::now());
impl_calendar_date_type_for_date_time!(Local, Local::now());
impl_calendar_date_type_for_date_time!(FixedOffset, Local::now().fixed_offset());

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_calendar::CalendarDate;
    use crate::use_calendar::tests::check_date_type;

    fn naive(year: i32, month: u8, day: u8) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month as u32, day as u32).unwrap()
    }

    #[test]
    fn naive_date() {
        check_date_type(naive);
    }

    #[test]
    fn date_time_utc() {
        check_date_type(|y, m, d| naive(y, m, d).and_hms_opt(23, 30, 0).unwrap().and_utc());
    }

    #[test]
    fn date_time_local() {
        check_date_type(|y, m, d| {
            Local
                .from_local_datetime(&naive(y, m, d).and_hms_opt(0, 15, 0).unwrap())
                .earliest()
                .unwrap()
        });
    }

    #[test]
    fn date_time_fixed_offset() {
        let tz = FixedOffset::east_opt(-11 * 3600).unwrap();
        check_date_type(|y, m, d| {
            tz.from_local_datetime(&naive(y, m, d).and_hms_opt(23, 59, 0).unwrap())
                .unwrap()
        });
    }

    #[test]
    fn compares_by_calendar_day() {
        let morning = naive(2026, 2, 15).and_hms_opt(1, 0, 0).unwrap().and_utc();
        let evening = naive(2026, 2, 15).and_hms_opt(22, 0, 0).unwrap().and_utc();
        let date = CalendarDate::Current(morning);
        assert!(date.is_selected(&evening));
        assert!(!date.is_before(&evening));
        assert!(date.is_between(&evening, &evening));
        assert_eq!(evening.next_day().unwrap().time(), evening.time());
    }
}
