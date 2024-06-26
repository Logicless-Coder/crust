use core::{fmt, panic};
use std::{
    env::args,
    fs,
    io::{stdin, Read},
};

#[derive(Debug, PartialEq, Eq)]
enum CLIOption {
    Fields(Vec<u32>),
    File(String),
    Delimiter(String),
}

fn parse_options(args: &Vec<String>) -> Vec<CLIOption> {
    let mut options = Vec::<CLIOption>::new();

    for arg in args {
        if arg.starts_with("-f") {
            let mut delim = ",";
            if arg.contains(" ") && !arg.contains(delim) {
                delim = " ";
            }
            let field_nums: Vec<u32> = arg
                .strip_prefix("-f")
                .expect("Inside if it must start with '-f'")
                .split(delim)
                .map(|x| {
                    x.parse::<u32>()
                        .unwrap_or_else(|_| panic!("Invalid field specified"))
                })
                .collect();
            options.push(CLIOption::Fields(field_nums));
        } else if arg.starts_with("-d") {
            let delimiter = arg
                .strip_prefix("-d")
                .expect("Inside if it must start with '-d'");
            options.push(CLIOption::Delimiter(delimiter.to_owned()))
        } else if !arg.starts_with("-") {
            options.push(CLIOption::File(arg.to_owned()))
        }
    }

    options
}

#[derive(Debug, PartialEq, Eq)]
struct Table {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
    delimiter: String,
}

impl Table {
    fn get_cols(&self, indices: Vec<u32>) -> Table {
        let mut data: Table = Table::default();
        data.delimiter = self.delimiter.clone();
        for index in &indices {
            data.columns.push(self.columns[*index as usize].clone());
        }

        for row in &self.rows {
            let mut res_row: Vec<String> = Vec::new();
            for index in &indices {
                res_row.push(row[*index as usize].clone());
            }
            data.rows.push(res_row);
        }

        data
    }
}

impl Default for Table {
    fn default() -> Self {
        Self {
            columns: vec![],
            rows: vec![],
            delimiter: "\t".into(),
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cols = self.columns.clone().into_iter().peekable();
        while let Some(col) = cols.next() {
            print!("{}", col);
            if !cols.peek().is_none() {
                print!("{}", self.delimiter);
            }
        }
        write!(f, "\n")?;
        let mut rows = self.rows.clone().into_iter().peekable();
        while let Some(row) = rows.next() {
            let mut vals = row.into_iter().peekable();
            while let Some(val) = vals.next() {
                print!("{}", val);
                if !vals.peek().is_none() {
                    print!("{}", self.delimiter);
                }
            }
            if !rows.peek().is_none() {
                print!("\n");
            }
        }
        Ok(())
    }
}

fn parse_tsv(raw: String, delimiter: &String) -> Table {
    let mut data: Table = Table::default();
    data.delimiter = delimiter.clone();
    let mut lines = raw.lines().into_iter().peekable();

    let columns: Vec<String> = lines
        .peek()
        .expect("Lines should not be empty")
        .split(delimiter)
        .map(|x| x.to_owned())
        .collect();
    data.columns = columns;

    for line in lines.skip(1) {
        let row: Vec<String> = line.split(delimiter).map(|x| x.to_owned()).collect();
        data.rows.push(row);
    }

    data
}

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let options: Vec<CLIOption> = parse_options(&args);

    let mut filename: Option<String> = None;
    let mut fields: Vec<u32> = Vec::new();
    let mut delimiter: String = "\t".into();
    for option in options {
        match option {
            CLIOption::File(x) => filename = Some(x),
            CLIOption::Fields(x) => fields = x.iter().map(|y| y - 1).collect(),
            CLIOption::Delimiter(x) => delimiter = x,
        }
    }

    let mut raw: String = String::new();
    match filename {
        Some(x) => {
            raw = fs::read_to_string(x).unwrap_or_else(|e| panic!("Couldn't read file: {}", e))
        }
        None => {
            stdin()
                .read_to_string(&mut raw)
                .expect("Should be able to read from stdin");
        }
    }

    let data: Table = parse_tsv(raw, &delimiter);

    let result: Table = data.get_cols(fields);
    println!("{}", result);
}
