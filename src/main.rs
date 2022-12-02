mod dispatch;
pub use dispatch::RunPart;

 fn main() {
    dispatch::main()
}

pub mod y2022;

pub fn run(year: u16, day: u8, part: crate::RunPart) {
    match year {
        2022 => y2022::run_day(day, part),
        _ => panic!("Invalid year: no solutions have been written for {}", year),
    }
}
