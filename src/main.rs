use core::{fmt, panic};
use std::{
    env::args,
    fmt::{format, Display},
    fs,
};

#[derive(Debug, PartialEq, Eq)]
enum CLIOption {
    Field(u32),
    File(String),
}

fn parse_options(args: &Vec<String>) -> Vec<CLIOption> {
    let mut options = Vec::<CLIOption>::new();

    for arg in args {
        if arg.starts_with("-f") {
            if let Ok(field_num) = arg
                .strip_prefix("-f")
                .expect("Inside if it must start with '-f'")
                .parse::<u32>()
            {
                options.push(CLIOption::Field(field_num))
            }
        } else {
            options.push(CLIOption::File(arg.to_owned()))
        }
    }

    options
}

#[derive(Debug, PartialEq, Eq)]
struct Table {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    fn get_row(self, index: u32) -> Vec<String> {
        self.rows[index as usize].clone()
    }

    fn get_col(&self, index: u32) -> Table {
        let mut data: Table = Table::default();
        data.columns = vec![self.columns[index as usize].clone()];

        for row in &self.rows {
            data.rows.push(vec![row[index as usize].clone()]);
        }

        data
    }
}

impl Default for Table {
    fn default() -> Self {
        Self {
            columns: vec![],
            rows: vec![],
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for col in &self.columns {
            write!(f, "{}\t", col)?;
        }
        write!(f, "\n")?;
        for row in &self.rows {
            for val in row {
                write!(f, "{}\t", val)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn parse_tsv(filename: &str) -> Table {
    let raw = fs::read_to_string(filename).unwrap_or_else(|e| panic!("Couldn't read file: {}", e));
    let mut data: Table = Table::default();
    let mut lines = raw.lines().into_iter().peekable();

    let columns: Vec<String> = lines
        .peek()
        .expect("Lines should not be empty")
        .split("\t")
        .map(|x| x.to_owned())
        .collect();
    data.columns = columns;

    for line in lines.skip(1) {
        let row: Vec<String> = line.split("\t").map(|x| x.to_owned()).collect();
        data.rows.push(row);
    }

    data
}

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let options: Vec<CLIOption> = parse_options(&args);

    let mut filename: Option<String> = None;
    let mut fields: Vec<u32> = Vec::new();
    for option in options {
        match option {
            CLIOption::File(x) => filename = Some(x),
            CLIOption::Field(y) => fields.push(y - 1),
        }
    }

    if filename.is_none() {
        panic!("No filename specified");
    }

    let data: Table = parse_tsv(&filename.unwrap());

    for field in fields {
        let result: Table = data.get_col(field);
        println!("{}", result);
    }
}

mod tests {
    use crate::{parse_options, parse_tsv, CLIOption, Table};

    #[test]
    fn no_options_passed() {
        let args: Vec<String> = Vec::new();
        let options = parse_options(&args);

        assert!(options.is_empty());
    }

    #[test]
    fn field_option_passed() {
        let args: Vec<String> = vec!["-f2".into()];
        let target_options: Vec<CLIOption> = vec![CLIOption::Field(2)];

        let options = parse_options(&args);

        assert_eq!(options, target_options);
    }

    #[test]
    fn parse_sample_tsv() {
        let filename: String = "sample.tsv".into();
        let target_data: Table = Table {
            columns: vec!["f0", "f1", "f2", "f3", "f4"]
                .into_iter()
                .map(|x| x.to_owned())
                .collect(),
            rows: vec![],
        };
        let data: Table = parse_tsv(&filename);

        assert_eq!(data, target_data);
    }
}
