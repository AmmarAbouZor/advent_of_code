mod day_01;
mod day_02;
mod day_03;
mod day_04;

pub fn run() {
    run_day(4);
}

fn run_day(day: u8) {
    match day {
        1 => day_01::run(),
        2 => day_02::run(),
        3 => day_03::run(),
        4 => day_04::run(),
        _ => panic!("day not implemented"),
    }
}
