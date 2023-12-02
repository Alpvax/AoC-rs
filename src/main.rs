// ============================================================================
// DO NOT MODIFY THIS FILE!
// IT WILL BE OVERWRITTEN EVERY TIME THE SOURCE IS BUILT!
// ============================================================================

mod cli;
pub use cli::RunPart;
mod dispatch;
#[allow(unused_imports)]
use dispatch::*;
mod dispatcher;
#[allow(unused_imports)]
use dispatcher::*;

fn main() {
    cli::main()
}

pub fn run(year: u16, day: u8, part: crate::RunPart) {
    match year {
        #[cfg(feature = "y2022")]
        2022 => y2022::run_day(day, part),
        #[cfg(feature = "y2023")]
        2023 => y2023::run_day(day, part),
        _ => panic!("Invalid year: no solutions have been written for {}", year),
    }
}
#[cfg(feature = "y2022")]
pub mod y2022;
#[cfg(feature = "y2023")]
pub mod y2023;
