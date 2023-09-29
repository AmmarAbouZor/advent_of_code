mod utls;
mod year_15;
mod year_16;
mod year_20;
mod year_21;
mod year_22;

fn main() {
    run_year("20");
}

fn run_year(year: &str) {
    match year {
        "15" => year_15::run(),
        "16" => year_16::run(),
        "22" => year_22::run(),
        "21" => year_21::run(),
        "20" => year_20::run(),
        _ => unreachable!("year not implemented"),
    }
}
