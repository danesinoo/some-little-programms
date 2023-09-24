use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Weekday};
use home::home_dir;
use prettytable::{Row, Table};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Instant;

#[inline]
pub fn path() -> PathBuf {
    let mut home = home_dir().expect("u windows, ah");
    home.push(".config");
    home.push("programmini");
    home.push("buff");
    home.push("studio.log");
    home
}

pub fn display_info(name: &str) {
    let now = Instant::now();
    let raw_data = RawData::from_file(&path());
    raw_data.iter().for_each(|raw_data| {
        if raw_data.name == name || name == "all" {
            display_data(raw_data);
        }
    });

    let f = |x: u16| {
        if x < 10 {
            format!("0{}", x)
        } else {
            format!("{}", x)
        }
    };

    println!(
        "now: {}:{}",
        f(Local::now().hour() as u16),
        f(Local::now().minute() as u16)
    );
    println!("Time elapsed: {:?}ms", now.elapsed());
}

fn display_data(raw_data: &RawData) {
    let mut data = DataOnTime::new(Local::now().weekday().succ());
    let last_week = raw_data.week(0);
    data.start_time(&last_week)
        .end_time(&last_week)
        .total_time(&last_week)
        .average_time(raw_data);

    // format data
    println!("{}:", raw_data.name.to_ascii_uppercase());
    let table = Table::init(data.to_rows());
    table.printstd();
    println!();
}

/// Memorizza i dati relativi ad una attività
#[derive(Debug, Clone)]
struct RawData {
    name: String,
    start_time: Vec<DateTime<Local>>,
    end_time: Vec<DateTime<Local>>,
}

/// I dati che sono mostrati alla fine
#[derive(Debug, Clone, Copy)]
struct DataOnTime {
    w: Weekday,
    st: [u16; 7],
    et: [u16; 7],
    tt: [u16; 7],
    at: [u16; 7],
}

impl RawData {
    #[inline]
    fn new(name: &str) -> Self {
        RawData {
            name: name.to_string(),
            start_time: Vec::new(),
            end_time: Vec::new(),
        }
    }

    #[inline]
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

    #[inline]
    fn parse_time(string: &str) -> DateTime<Local> {
        Local
            .datetime_from_str(string, "%Y-%m-%d %H:%M:%S%.f %:z")
            .unwrap()
    }

    /// Vettore di attività
    /// i dati in ciascuna attività sono ordinati in base
    /// al tempo: prima i dati più vecchi
    fn from_file(file: &PathBuf) -> Vec<Self> {
        let file = OpenOptions::new().read(true).open(file).unwrap();

        let mut res = BufReader::new(file)
            .lines()
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
            });
        res.iter_mut().for_each(|x| {
            if x.start_time.len() > x.end_time.len() {
                x.end_time.push(Local::now());
            }
            x.day();
        });
        res
    }

    fn day(&mut self) {
        let mut i = 0;
        while i < self.start_time.len() {
            if self.start_time[i].weekday() != self.end_time[i].weekday() {
                // spezzo lo studio nei giorni che lo compongono
                let mut end = self.start_time[i].clone();
                end = end.with_hour(23).unwrap().with_minute(59).unwrap();
                let mut start = self.end_time[i].clone();
                start = start.with_hour(0).unwrap().with_minute(0).unwrap();

                self.end_time.insert(i, end);
                self.start_time.insert(i + 1, start);
            }
            i += 1;
        }
    }

    /// prende i dati relativi alla settimana indicata da which:
    /// 0 -> settimana corrente
    /// 1 -> settimana scorsa
    /// ...
    fn week(&self, which: u16) -> Self {
        let cond1 = |time: &DateTime<Local>| {
            (Local::now().num_days_from_ce() - time.num_days_from_ce()) as u16 / 7 == which
        };

        let cond2 = |x: &DateTime<Local>| !cond1(x);

        // prendo gli indici dei dati che mi interessano
        let from = self
            .start_time
            .iter()
            .position(cond1)
            .unwrap_or(self.start_time.len());

        let to = self.end_time[from..]
            .iter()
            .position(cond2)
            .unwrap_or(self.start_time.len());

        if from == to {
            return RawData::new(&self.name);
        }

        RawData {
            name: self.name.clone(),
            start_time: self.start_time[from..to].to_vec(),
            end_time: self.end_time[from..to].to_vec(),
        }
    }
}

impl DataOnTime {
    #[inline]
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
        let mut pos = 0;
        self.st.iter_mut().for_each(|start| {
            if let Some(index) = raw.start_time[pos..]
                .iter()
                .position(|data| data.weekday() == self.w)
            {
                *start = num_min(&raw.start_time[index + pos]);
                pos = index + 1;
            } else {
                *start = 0;
            }
            self.w = self.w.succ();
        });
        self
    }

    fn end_time(&mut self, raw: &RawData) -> &mut Self {
        let mut pos = raw.end_time.len();
        self.et.iter_mut().rev().for_each(|end| {
            self.w = self.w.pred();
            if let Some(index) = raw.end_time[..pos]
                .iter()
                .rposition(|x| x.weekday() == self.w)
            {
                *end = num_min(&raw.end_time[index]);
                pos = index;
            } else {
                *end = 0;
            }
        });
        self
    }

    #[inline]
    fn tt_index(t: &DateTime<Local>, w: Weekday) -> usize {
        (7 + t.weekday().num_days_from_monday() - w.num_days_from_monday()) as usize % 7
    }

    // NB qui c'è un buggino: si chiama il next() e poi break, quindi dopo il
    // primo next, la catena si sfasa e non funziona più
    fn total_time(&mut self, raw: &RawData) -> &mut Self {
        raw.start_time
            .iter()
            .zip(raw.end_time.iter())
            .for_each(|(st, et)| {
                let index = Self::tt_index(st, self.w);
                self.tt[index] += min(et) - min(st);
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

        #[inline]
        fn add_row(table: &mut Vec<Row>, data: &[u16], title: &str) {
            let f = |x: u16| {
                if x < 10 {
                    format!("0{}", x)
                } else {
                    format!("{}", x)
                }
            };

            table.push(Row::from(vec![title.to_string()].into_iter().chain(
                data.iter().map(|x| {
                    let hour = x / 60;
                    let min = x % 60;
                    format!("{}:{}", f(hour), f(min))
                }),
            )));
        }

        add_row(&mut rows, &self.st, "Start");
        add_row(&mut rows, &self.et, "End");
        add_row(&mut rows, &self.tt, "Total");
        add_row(&mut rows, &self.at, "Average");
        rows
    }
}

#[inline]
fn min(t: &DateTime<Local>) -> u16 {
    (t.hour() * 60 + t.minute()) as u16
}

#[inline]
fn num_min(time: &DateTime<Local>) -> u16 {
    (time.hour() * 60 + time.minute()) as u16
}
