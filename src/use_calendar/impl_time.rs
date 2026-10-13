use super::CalendarDateType;
use time::{Date, Duration, Month, OffsetDateTime};

fn now() -> OffsetDateTime {
    OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc())
}

fn add_months_to_date(date: Date, months: i32) -> Option<Date> {
    let total_months = date.year() as i64 * 12 + (date.month() as i64 - 1) + months as i64;
    let year = i32::try_from(total_months.div_euclid(12)).ok()?;
    let month = Month::try_from(total_months.rem_euclid(12) as u8 + 1).ok()?;
    let day = date.day().min(month.length(year));

    Date::from_calendar_date(year, month, day).ok()
}

impl CalendarDateType for Date {
    fn today() -> Self {
        now().date()
    }

    fn ymd(&self) -> (i32, u8, u8) {
        (self.year(), self.month() as u8, self.day())
    }

    fn prev_day(&self) -> Option<Self> {
        self.previous_day()
    }

    fn next_day(&self) -> Option<Self> {
        Date::next_day(*self)
    }

    fn weekday_from_monday(&self) -> u8 {
        self.weekday().number_days_from_monday()
    }

    fn add_months(&self, months: i32) -> Option<Self> {
        add_months_to_date(*self, months)
    }
}

impl CalendarDateType for OffsetDateTime {
    fn today() -> Self {
        now()
    }

    fn ymd(&self) -> (i32, u8, u8) {
        (self.year(), self.month() as u8, self.day())
    }

    fn prev_day(&self) -> Option<Self> {
        self.checked_sub(Duration::DAY)
    }

    fn next_day(&self) -> Option<Self> {
        self.checked_add(Duration::DAY)
    }

    fn weekday_from_monday(&self) -> u8 {
        self.weekday().number_days_from_monday()
    }

    fn add_months(&self, months: i32) -> Option<Self> {
        Some(self.replace_date(add_months_to_date(self.date(), months)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_calendar::CalendarDate;
    use crate::use_calendar::tests::check_date_type;
    use time::{Time, UtcOffset};

    fn date(year: i32, month: u8, day: u8) -> Date {
        Date::from_calendar_date(year, Month::try_from(month).unwrap(), day).unwrap()
    }

    #[test]
    fn time_date() {
        check_date_type(date);
    }

    #[test]
    fn offset_date_time() {
        let offset = UtcOffset::from_hms(-11, 0, 0).unwrap();
        check_date_type(|y, m, d| {
            date(y, m, d)
                .with_time(Time::from_hms(23, 59, 0).unwrap())
                .assume_offset(offset)
        });
    }

    #[test]
    fn compares_by_calendar_day() {
        let morning = date(2026, 2, 15).midnight().assume_utc();
        let evening = date(2026, 2, 15)
            .with_time(Time::from_hms(22, 0, 0).unwrap())
            .assume_utc();
        let date = CalendarDate::Current(morning);
        assert!(date.is_selected(&evening));
        assert!(!date.is_before(&evening));
        assert!(date.is_between(&evening, &evening));
    }
}
