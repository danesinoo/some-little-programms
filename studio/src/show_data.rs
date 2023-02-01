use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Weekday};
use prettytable::{row, Row, Table};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

pub fn display_info() {
    let mut raw_data = RawData::from_file("/Users/carlorosso/.config/programmini/studio.log");
    raw_data.iter_mut().for_each(|raw| {
        raw.clean_up();
    });
    raw_data.iter().for_each(|raw_data| {
        let mut data = DataOnTime::new(raw_data.weekday());
        let mut last_week = raw_data.get_week(0);
        last_week.clean_up();
        data.start_time(&last_week)
            .end_time(&last_week)
            .total_time(&last_week)
            .average_time(raw_data);

        // format data
        let mut table = Table::init(data.to_rows());
        table.set_titles(row![raw_data.name]);
        table.printstd();
        println!();
    });
}

#[derive(Debug, Clone)]
struct RawData {
    name: String,
    start_time: Vec<DateTime<Local>>,
    end_time: Vec<DateTime<Local>>,
}

impl RawData {
    fn new(name: &str) -> Self {
        RawData {
            name: name.to_string(),
            start_time: Vec::new(),
            end_time: Vec::new(),
        }
    }

    fn push(&mut self, time: DateTime<Local>, start: bool) {
        if start && self.start_time.len() == self.end_time.len() {
            self.start_time.push(time);
        } else if !start && self.start_time.len() > self.end_time.len() {
            self.end_time.push(time);
        }
    }

    fn compute_index(name: &str, flag: &str, time: &DateTime<Local>, acc: &mut Vec<RawData>) {
        if let Some(raw) = acc.iter_mut().find(|x| x.name == name) {
            match flag.as_ref() {
                "begin" => raw.push(*time, true),
                "end" => raw.push(*time, false),
                _ => unreachable!(),
            }
        } else {
            acc.push(RawData::new(&name));
            Self::compute_index(name, flag, time, acc);
        }
    }

    fn parse_time(string: &str) -> Result<DateTime<Local>, chrono::ParseError> {
        match Local.datetime_from_str(string, "%Y-%m-%d %H:%M:%S%.f %:z") {
            Ok(datetime) => Ok(datetime),
            Err(e) => Err(e),
        }
    }

    fn from_file(file: &str) -> Vec<Self> {
        let file = OpenOptions::new().read(true).open(file).unwrap();

        let reader = BufReader::new(file).lines();
        reader
            .map(|line| {
                if let Some((name, time)) = line.unwrap().split_once(' ') {
                    (name.to_string(), time.to_string())
                } else {
                    unreachable!()
                }
            })
            .map(|(name, time)| {
                if let Some((name1, flag)) = name.split_once('_') {
                    (
                        name1.to_string(),
                        flag.to_string(),
                        Self::parse_time(&time).unwrap(),
                    )
                } else {
                    unreachable!()
                }
            })
            .fold(Vec::new(), |mut acc, (name, flag, time)| {
                Self::compute_index(&name, &flag, &time, &mut acc);
                acc
            })
    }

    fn weekday(&self) -> Weekday {
        self.end_time.last().unwrap().weekday().succ()
    }

    // se comincio a studiare un giorno e finisco oltre la mezzanotte, i tempi
    // di studio vengono suddivisi in due giorni, not yet
    fn clean_up(&mut self) -> &mut Self {
        /*
        println!("{:?}", self);
        let mut starts = Vec::new();
        let mut ends = Vec::new();
        let mut i = 0;
        while i < self.start_time.len() {
            if let Some(end) = self.end_time.iter().find(|x| *x > &self.start_time[i]) {
                starts.push(self.start_time[i]);
                ends.push(end.clone());
                if let Some(index) = self.start_time.iter().position(|x| x > end) {
                    i = index;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        self.start_time = starts;
        self.end_time = ends;
        */
        if self.start_time.len() > self.end_time.len() {
            self.start_time.pop();
        }
        self.start_time.sort();
        self.start_time.reverse();
        self.end_time.sort();
        self.end_time.reverse();
        self
    }

    fn get_week(&self, which: u16) -> Self {
        RawData {
            name: self.name.clone(),
            start_time: get_week(&mut self.start_time.clone(), which),
            end_time: get_week(&mut self.end_time.clone(), which),
        }
    }
}

fn get_week(raws: &Vec<DateTime<Local>>, which: u16) -> Vec<DateTime<Local>> {
    raws.iter()
        .partition(|x| {
            (Local::now().num_days_from_ce() - x.num_days_from_ce()) as u16 / 7 < which + 1
        })
        .0
}

#[derive(Debug, Clone, Copy)]
struct DataOnTime {
    weekday: Weekday,
    start_times: [u16; 7],
    end_times: [u16; 7],
    total_times: [u16; 7],
    average_times: [u16; 7],
}

impl DataOnTime {
    fn new(weekday: Weekday) -> Self {
        DataOnTime {
            weekday,
            start_times: [0; 7],
            end_times: [0; 7],
            total_times: [0; 7],
            average_times: [0; 7],
        }
    }

    fn start_time(&mut self, raw: &RawData) -> &mut Self {
        self.start_times.iter_mut().for_each(|start| {
            self.weekday = self.weekday.succ();
            if let Some(data) = raw
                .start_time
                .iter()
                // sarebbe meglio andare dalla direzione giusta e non dalla fine...
                .rev()
                .find(|data| data.weekday() == self.weekday)
            {
                *start = (data.hour() * 60 + data.minute()) as u16;
            } else {
                *start = 0;
            }
        });
        self
    }

    fn end_time(&mut self, raw: &RawData) -> &mut Self {
        self.end_times.iter_mut().for_each(|x| {
            self.weekday = self.weekday.succ();
            if let Some(data) = raw.end_time.iter().find(|x| x.weekday() == self.weekday) {
                *x = (data.hour() * 60 + data.minute()) as u16;
            } else {
                *x = 0;
            }
        });
        self
    }

    // NB qui c'è un buggino: si chiama il next() e poi break, quindi dopo il
    // primo next, la catena si sfasa e non funziona più
    fn total_time(&mut self, raw: &RawData) -> &mut Self {
        (0..7).for_each(|x| {
            self.weekday = self.weekday.succ();
            let from = raw
                .start_time
                .iter()
                .position(|x| x.weekday() == self.weekday)
                .unwrap_or(raw.end_time.len());
            let mut to = raw
                .start_time
                .iter()
                .rposition(|x| x.weekday() == self.weekday)
                .unwrap_or(0);
            if to != 0 {
                to += 1;
            }
            self.total_times[x] = (from..to)
                .map(|i| (raw.end_time[i] - raw.start_time[i]).num_minutes() as u16)
                .sum();
        });
        self
    }

    fn average_time(&mut self, _datetimes: &RawData) -> &mut Self {
        // TODO
        self
    }

    fn to_rows(&mut self) -> Vec<Row> {
        let mut rows = Vec::new();
        rows.push(Row::from(["".to_string()].into_iter().chain((0..7).map(
            |_| {
                self.weekday = self.weekday.succ();
                self.weekday.to_string()
            },
        ))));

        fn add_row(table: &mut Vec<Row>, data: &[u16], title: &str) {
            table.push(Row::from(
                vec![title.to_string()]
                    .into_iter()
                    .chain(data.iter().map(|&x| format!("{}:{}", x / 60, x % 60))),
            ));
        }

        add_row(&mut rows, &self.start_times, "Start");
        add_row(&mut rows, &self.end_times, "End");
        add_row(&mut rows, &self.total_times, "Total");
        add_row(&mut rows, &self.average_times, "Average");
        rows
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn prova() {
        assert_eq!((0..7).any(|x| x == 7), false);
    }
}
