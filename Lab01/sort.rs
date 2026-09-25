use std::env;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

struct Record {
    first_name: String,
    last_name: String,
    score: u8, // Between 0 and 100 inclusive
}

impl Record {
    fn new(first: &str, last: &str, score: u8) -> Self {
        Self {
            first_name: first.to_string(),
            last_name: last.to_string(),
            score,
        }
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}, {}", self.score, self.last_name, self.first_name)
    }
}

fn create_records(lines: &[String]) -> Vec<Record> {
    let mut records = Vec::new();

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        match words.as_slice() {
            [f, l, s, ..]
                if let Ok(score) = s.parse::<u8>()
                    && (0..=100).contains(&score) =>
            {
                records.push(Record::new(f, l, score));
            }
            _ => {}
        }
    }

    records
}

fn read_stdin() -> io::Result<Vec<String>> {
    io::stdin().lines().collect()
}

fn read_file(file: &str) -> io::Result<Vec<String>> {
    if let Ok(f) = File::open(file) {
        io::BufReader::new(f).lines().collect()
    } else {
        eprintln!("{} doesn't exist", file);
        process::exit(1);
    }
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    let result = if !args.is_empty() {
        read_file(&args[0])
    } else {
        read_stdin()
    };

    if let Ok(lines) = result {
        let mut records = create_records(&lines);
        records.sort_by(|a, b| {
            b.score
                .cmp(&a.score) // sort score descending
                .then_with(|| a.last_name.cmp(&b.last_name)) // sort last names ascending
                .then_with(|| a.first_name.cmp(&b.first_name))
        });
        for x in records {
            println!("{x}");
        }
    }
}
