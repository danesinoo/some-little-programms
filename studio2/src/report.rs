use crate::session::{Session, duration_to_str};

pub trait Report<T: Session> {
    fn new(name: &str, session: [T]) -> Self;

    /// Returns a slice of the sessions, ordered by begin time (considering the day).
    fn sessions(&self) -> &[T];

    fn hours_by_day(&self, day: chrono::NaiveDate) -> String {
        let duration = self.sessions().iter()
            .filter(|s| s.day() == day)
            .map(|s| s.duration())
            .map(|d| {
                let hours = chrono::Duration::hours(d[0..2].parse().unwrap());
                let minutes = chrono::Duration::minutes(d[3..5].parse().unwrap());
                hours + minutes
            })
        .sum::<chrono::Duration>();
        duration_to_str(duration)
    }

    fn begin_time(&self, day: chrono::NaiveDate) -> String {
        match self.sessions().iter()
            .find(|s| s.day() == day) {
            Some(s) => s.begin(),
            None => String::from("00:00"),
        }
    }

    fn end_time(&self, day: chrono::NaiveDate) -> String {
        match self.sessions().iter()
            .rev()
            .find(|s| s.day() == day) {
            Some(s) => s.end(),
            None => String::from("00:00"),
        }
    }

    fn hours_by_week(&self, week: chrono::NaiveWeek) -> String {
        let duration = self.sessions().iter()
            .filter(|s| s.week() == week)
            .map(|s| s.duration())
            .map(|d| {
                let hours = chrono::Duration::hours(d[0..2].parse().unwrap());
                let minutes = chrono::Duration::minutes(d[3..5].parse().unwrap());
                hours + minutes
            })
        .sum::<chrono::Duration>();
        duration_to_str(duration)
    }
}
