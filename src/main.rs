mod utls;
mod year_15;
mod year_16;
mod year_22;

fn main() {
    run_year(22);
}

fn run_year(year: u8) {
    match year {
        15 => year_15::run(),
        16 => year_16::run(),
        22 => year_22::run(),
        _ => panic!("year not implemented"),
    }
}
