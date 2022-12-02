pub mod d01;
pub mod d02;

pub fn run_day(day: u8, part: crate::RunPart) {
    match day {
        1 => d01::main(part),
        2 => d02::main(part),
        _ => panic!("Invalid day: {} does not have a solution", day),
    }
}
