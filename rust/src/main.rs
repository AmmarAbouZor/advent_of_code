mod utls;
mod year_15;
mod year_16;
mod year_20;
mod year_21;
mod year_22;
mod year_23;
mod year_24;

fn main() {
    run_year("24");
}

fn run_year(year: &str) {
    match year {
        "15" => year_15::run(),
        "16" => year_16::run(),
        "22" => year_22::run(),
        "21" => year_21::run(),
        "20" => year_20::run(),
        "23" => year_23::run(),
        "24" => year_24::run(),
        _ => unreachable!("year not implemented"),
    }
}
