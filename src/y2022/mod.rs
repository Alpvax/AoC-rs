// ============================================================================
// DO NOT MODIFY THIS FILE!
// IT WILL BE OVERWRITTEN EVERY TIME THE SOURCE IS BUILT!
// ============================================================================

pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;
pub mod d06;
pub mod d07;
pub mod d08;
pub mod d09;
pub mod d10;
pub mod d11;
pub mod d12;

pub fn run_day(day: u8, part: crate::RunPart) {
    match day {
        1 => d01::main(part),
        2 => d02::main(part),
        3 => d03::main(part),
        4 => d04::main(part),
        5 => d05::main(part),
        6 => d06::main(part),
        7 => d07::main(part),
        8 => d08::main(part),
        9 => d09::main(part),
        10 => d10::main(part),
        11 => d11::main(part),
        12 => d12::main(part),
        _ => panic!("Invalid day: {} does not have a solution", day),
    }
}
