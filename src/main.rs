// ============================================================================
// DO NOT MODIFY THIS FILE!
// IT WILL BE OVERWRITTEN EVERY TIME THE SOURCE IS BUILT!
// ============================================================================


mod cli;
pub use cli::RunPart;
mod dispatch;
use dispatch::aoc;

fn main() {
    cli::main()
}

pub fn run(year: u16, day: u8, part: crate::RunPart) {
    match year {
        2022 => y2022::run_day(day, part),
        _ => panic!("Invalid year: no solutions have been written for {}", year),
            }
        }
pub mod y2022;
