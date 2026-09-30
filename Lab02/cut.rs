use std::env;
use std::io;

const NUM_ARGS: usize = 2;

#[derive(Debug)]
enum Range {
    Single(usize),         // N
    From(usize),           // N-
    To(usize),             // -M
    Between(usize, usize), // N-M
}

impl Range {
    fn parse(input: &str) -> Option<Range> {
        if input.is_empty() {
            return None;
        }

        match input.split_once('-') {
            // N
            None => {
                let number = parse_positive(input)?;
                Some(Range::Single(number))
            }

            Some((left, right)) => match (left, right) {
                // N-
                (start, "") => {
                    let start = parse_positive(start)?;
                    Some(Range::From(start))
                }
                // -M
                ("", end) => {
                    let end = parse_positive(end)?;
                    Some(Range::To(end))
                }
                // N-M
                (start, end) => {
                    let start = parse_positive(start)?;
                    let end = parse_positive(end)?;

                    if start > end {
                        return None;
                    }

                    Some(Range::Between(start, end))
                }
            },
        }
    }

    fn contains(&self, n: usize) -> bool {
        match self {
            Range::Single(x) => n == *x,
            Range::From(x) => n >= *x,
            Range::To(x) => n <= *x,
            Range::Between(x, y) => n >= *x && n <= *y,
        }
    }
}

fn parse_argument(input: &str) -> Option<(char, Vec<Range>)> {
    let (mode, range_spec) = if let Some(spec) = input.strip_prefix("-c") {
        ('c', spec)
    } else {
        let spec = input.strip_prefix("-f")?;
        ('f', spec)
    };

    if range_spec.is_empty() {
        return None;
    }

    let ranges = parse_ranges(range_spec)?;

    Some((mode, ranges))
}

fn parse_positive(input: &str) -> Option<usize> {
    match input.parse::<usize>() {
        Ok(number) if number > 0 => Some(number),
        _ => {
            println!("ranges must contain positive integers");
            None
        }
    }
}

fn parse_ranges(input: &str) -> Option<Vec<Range>> {
    let mut ranges: Vec<Range> = Vec::new();

    for range in input.split(',') {
        let range = Range::parse(range)?;
        ranges.push(range);
    }

    Some(ranges)
}

fn select_characters(line: &str, ranges: &[Range]) -> String {
    line.chars()
        .enumerate()
        .filter_map(|(index, character)| {
            let position = index + 1;

            if ranges.iter().any(|range| range.contains(position)) {
                Some(character)
            } else {
                None
            }
        })
        .collect()
}

fn select_fields(line: &str, ranges: &[Range]) -> String {
    line.split('\t')
        .enumerate()
        .filter_map(|(index, field)| {
            let position = index + 1;

            if ranges.iter().any(|range| range.contains(position)) {
                Some(field)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\t")
}

fn process_input(mode: char, ranges: &[Range]) {
    let stdin = io::stdin();

    for line_result in stdin.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("failed to read line: {e}");
                return;
            }
        };

        let output = match mode {
            'c' => select_characters(&line, ranges),
            'f' => select_fields(&line, ranges),
            _ => unreachable!(),
        };

        println!("{output}");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != NUM_ARGS {
        println!("usage: {} -c|-f ranges", args[0]);
        return;
    }

    let (mode, ranges) = match parse_argument(&args[1]) {
        Some(result) => result,
        None => {
            println!("invalid argument");
            return;
        }
    };

    process_input(mode, &ranges);
}
