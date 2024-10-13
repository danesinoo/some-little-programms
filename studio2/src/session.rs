use chrono;

pub trait Session {
    fn new(begin: &str, end: &str) -> Self;
    fn begin(&self) -> String;
    fn end(&self) -> String;
    fn duration(&self) -> String;
    fn day(&self) -> chrono::NaiveDate;
    fn week(&self) -> chrono::NaiveWeek {
        let weekday = chrono::Weekday::from(chrono::Local::now().date());
        self.day().week()
    }
}

pub struct SessionImpl {
    begin: chrono::NaiveDateTime,
    end: chrono::NaiveDateTime,
}

impl Session for SessionImpl {
    fn new(begin: &str, end: &str) -> Self {
        let begin = chrono::NaiveDateTime::parse_from_str(begin, "%Y-%m-%d %H:%M:%S").unwrap();
        let end = chrono::NaiveDateTime::parse_from_str(end, "%Y-%m-%d %H:%M:%S").unwrap();
        SessionImpl { begin, end }
    }

    fn day(&self) -> chrono::NaiveDate {
        self.begin.date()
    }

    fn begin(&self) -> String {
        self.begin.to_string()
    }

    fn end(&self) -> String {
        self.end.to_string()
    }

    fn duration(&self) -> String {
        let diff = self.end - self.begin;
        duration_to_str(diff)
    }
}

pub fn duration_to_str(duration: chrono::Duration) -> String {
    let mut hours = (duration.num_hours() % 24).to_string();
    let mut minutes = (duration.num_minutes() % 60).to_string();
    if hours.len() < 2 {
        hours = format!("0{}", hours);
    }
    if minutes.len() < 2 {
        minutes = format!("0{}", minutes);
    }
    format!("{}:{}", hours, minutes)
}
