use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Weekday};
use home::home_dir;
use prettytable::{row, Row, Table};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

pub fn path() -> PathBuf {
    let mut home = home_dir().expect("u windows, ah");
    home.push(".config");
    home.push("programmini");
    home.push("buff");
    home.push("studio.log");
    home
}

pub fn display_info() {
    let mut raw_data = RawData::from_file(&path());
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

    fn push(&mut self, time: &DateTime<Local>, start: bool) {
        if start && self.start_time.len() == self.end_time.len() {
            self.start_time.push(*time);
        } else if !start && self.start_time.len() > self.end_time.len() {
            self.end_time.push(*time);
        }
    }

    fn compute_index(name: &str, flag: bool, time: &DateTime<Local>, acc: &mut Vec<RawData>) {
        if let Some(raw) = acc.iter_mut().find(|x| x.name == name) {
            raw.push(time, flag);
        } else {
            acc.push(RawData::new(&name));
            Self::compute_index(name, flag, time, acc);
        }
    }

    fn parse_time(string: &str) -> DateTime<Local> {
        Local
            .datetime_from_str(string, "%Y-%m-%d %H:%M:%S%.f %:z")
            .unwrap()
    }

    fn from_file(file: &PathBuf) -> Vec<Self> {
        let file = OpenOptions::new().read(true).open(file).unwrap();

        let reader = BufReader::new(file).lines();
        reader
            .map(|line| {
                let line = line.unwrap();
                let words = line.split_whitespace().collect::<Vec<_>>();
                let time = Self::parse_time(&words[2..].join(" "));
                match words[1] {
                    "begin" => (words[0].to_string(), true, time),
                    "end" => (words[0].to_string(), false, time),
                    _ => unreachable!(),
                }
            })
            .fold(Vec::new(), |mut acc, (name, flag, time)| {
                Self::compute_index(&name, flag, &time, &mut acc);
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
            self.end_time.push(Local::now());
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
    w: Weekday,
    st: [u16; 7],
    et: [u16; 7],
    tt: [u16; 7],
    at: [u16; 7],
}

impl DataOnTime {
    fn new(w: Weekday) -> Self {
        DataOnTime {
            w,
            st: [0; 7],
            et: [0; 7],
            tt: [0; 7],
            at: [0; 7],
        }
    }

    fn start_time(&mut self, raw: &RawData) -> &mut Self {
        self.st.iter_mut().for_each(|start| {
            if let Some(data) = raw
                .start_time
                .iter()
                // sarebbe meglio andare dalla direzione giusta e non dalla fine...
                .rev()
                .find(|data| data.weekday() == self.w)
            {
                *start = (data.hour() * 60 + data.minute()) as u16;
            } else {
                *start = 0;
            }
            self.w = self.w.succ();
        });
        self
    }

    fn end_time(&mut self, raw: &RawData) -> &mut Self {
        self.et.iter_mut().for_each(|x| {
            if let Some(data) = raw.end_time.iter().find(|x| x.weekday() == self.w) {
                *x = (data.hour() * 60 + data.minute()) as u16;
            } else {
                *x = 0;
            }
            self.w = self.w.succ();
        });
        self
    }

    // NB qui c'è un buggino: si chiama il next() e poi break, quindi dopo il
    // primo next, la catena si sfasa e non funziona più
    fn total_time(&mut self, raw: &RawData) -> &mut Self {
        self.tt.iter_mut().for_each(|tt| {
            let from = raw
                .start_time
                .iter()
                .position(|x| x.weekday() == self.w)
                .unwrap_or(raw.end_time.len());
            let mut to = raw
                .start_time
                .iter()
                .rposition(|x| x.weekday() == self.w)
                .unwrap_or(0);
            if to != 0 {
                to += 1;
            }
            *tt = (from..to)
                .map(|i| (raw.end_time[i] - raw.start_time[i]).num_minutes() as u16)
                .sum();
            self.w = self.w.succ();
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
                self.w = self.w.succ();
                self.w.pred().to_string()
            },
        ))));

        fn add_row(table: &mut Vec<Row>, data: &[u16], title: &str) {
            table.push(Row::from(
                vec![title.to_string()]
                    .into_iter()
                    .chain(data.iter().map(|&x| format!("{}:{}", x / 60, x % 60))),
            ));
        }

        add_row(&mut rows, &self.st, "Start");
        add_row(&mut rows, &self.et, "End");
        add_row(&mut rows, &self.tt, "Total");
        add_row(&mut rows, &self.at, "Average");
        rows
    }
}
