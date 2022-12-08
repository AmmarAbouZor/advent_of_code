mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

pub fn run() {
    run_day(5);
}

fn run_day(day: u8) {
    match day {
        1 => day_01::run(),
        2 => day_02::run(),
        3 => day_03::run(),
        4 => day_04::run(),
        5 => day_05::run(),
        _ => panic!("day not implemented"),
    }
}
