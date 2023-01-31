mod utls;
mod year_15;
mod year_16;

fn main() {
    run_year("16");
}

fn run_year(year: &str) {
    match year {
        "15" => year_15::run(),
        "16" => year_16::run(),
        _ => panic!("year not implemented"),
    }
}
