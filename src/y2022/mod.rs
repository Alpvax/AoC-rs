// ============================================================================
// DO NOT MODIFY THIS FILE!
// IT WILL BE OVERWRITTEN EVERY TIME THE SOURCE IS BUILT!
// ============================================================================

pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;

pub fn run_day(day: u8, part: crate::RunPart) {
    match day {
        1 => d01::main(part),
        2 => d02::main(part),
        3 => d03::main(part),
        4 => d04::main(part),
        5 => d05::main(part),
        _ => panic!("Invalid day: {} does not have a solution", day),
    }
}
