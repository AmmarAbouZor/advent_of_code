use std::{
    fs,
    io::{self, BufRead},
};

// I can't use this macro because I don't save the input files within the git repository. In case I
// clone the repo on a new setup I'll get compiler error on each missing file even the code doesn't
// pass through this method

/// Include the input file for the given year and day and return it as &'static str.
#[macro_export]
macro_rules! include_input {
    {$year:literal, $day:literal} => {{
        include_str!(concat!("../../input/year_", $year, "/day_", $day, ".txt"))
    }};
}

pub fn read_lines_from_file(path: &str) -> Vec<String> {
    let file = fs::File::open(path).unwrap();

    io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect()
}

pub fn get_input_path(year: &str, day: &str) -> String {
    format!("input/year_{year}/day_{day}.txt")
}

pub fn read_text_from_file(year: &str, day: &str) -> String {
    let path = get_input_path(year, day);

    fs::read_to_string(path).unwrap()
}
