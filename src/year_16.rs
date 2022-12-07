mod day_01;
mod day_02;
mod day_03;

pub fn run() {
    run_day(3);
}

fn run_day(day: u8) {
    match day {
        1 => day_01::run(),
        2 => day_02::run(),
        3 => day_03::run(),
        _ => panic!("day not implemented"),
    }
}
