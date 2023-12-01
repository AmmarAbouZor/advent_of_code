mod day_01;

pub fn run() {
    run_day(1);
}

fn run_day(day: u8) {
    match day {
        1 => day_01::run(),
        _ => unreachable!("day not implemented"),
    }
}
