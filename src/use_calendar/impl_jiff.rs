use super::CalendarDateType;
use jiff::Zoned;
use jiff::civil::Date;
use jiff::{Span, ToSpan};

fn months_span(months: i32) -> Option<Span> {
    Span::new().try_months(months).ok()
}

impl CalendarDateType for Date {
    fn today() -> Self {
        Zoned::now().date()
    }

    fn ymd(&self) -> (i32, u8, u8) {
        (self.year() as i32, self.month() as u8, self.day() as u8)
    }

    fn prev_day(&self) -> Option<Self> {
        self.yesterday().ok()
    }

    fn next_day(&self) -> Option<Self> {
        self.tomorrow().ok()
    }

    fn weekday_from_monday(&self) -> u8 {
        self.weekday().to_monday_zero_offset() as u8
    }

    fn add_months(&self, months: i32) -> Option<Self> {
        self.checked_add(months_span(months)?).ok()
    }
}

impl CalendarDateType for Zoned {
    fn today() -> Self {
        Zoned::now()
    }

    fn ymd(&self) -> (i32, u8, u8) {
        (self.year() as i32, self.month() as u8, self.day() as u8)
    }

    fn prev_day(&self) -> Option<Self> {
        self.checked_sub(1.day()).ok()
    }

    fn next_day(&self) -> Option<Self> {
        self.checked_add(1.day()).ok()
    }

    fn weekday_from_monday(&self) -> u8 {
        self.weekday().to_monday_zero_offset() as u8
    }

    fn add_months(&self, months: i32) -> Option<Self> {
        self.checked_add(months_span(months)?).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_calendar::CalendarDate;
    use crate::use_calendar::tests::check_date_type;
    use jiff::civil::date;

    #[test]
    fn civil_date() {
        check_date_type(|y, m, d| date(y as i16, m as i8, d as i8));
    }

    #[test]
    fn zoned() {
        for tz in ["UTC", "Europe/Berlin", "America/Sao_Paulo", "Pacific/Apia"] {
            check_date_type(|y, m, d| {
                date(y as i16, m as i8, d as i8)
                    .at(2, 30, 0, 0)
                    .in_tz(tz)
                    .unwrap()
            });
        }
    }

    #[test]
    fn zoned_steps_across_dst_gap() {
        // 2026-03-29 02:30 doesn't exist in Europe/Berlin
        let before = date(2026, 3, 28)
            .at(2, 30, 0, 0)
            .in_tz("Europe/Berlin")
            .unwrap();
        let gap = before.next_day().unwrap();
        assert_eq!(gap.ymd(), (2026, 3, 29));
        assert_eq!(gap.next_day().unwrap().ymd(), (2026, 3, 30));
        assert_eq!(gap.prev_day().unwrap().ymd(), (2026, 3, 28));
    }

    #[test]
    fn compares_by_calendar_day() {
        let morning = date(2026, 2, 15).at(1, 0, 0, 0).in_tz("UTC").unwrap();
        let evening = date(2026, 2, 15).at(22, 0, 0, 0).in_tz("UTC").unwrap();
        let date = CalendarDate::Current(morning);
        assert!(date.is_selected(&evening));
        assert!(!date.is_before(&evening));
        assert!(date.is_between(&evening, &evening));
    }
}
