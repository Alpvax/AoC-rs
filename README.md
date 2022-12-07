# AoC-rs
Rust implementation of Advent of Code solutions

To run, copy your input to `../input/<yyyy>/<dd>.txt` (the `input` directory should be on the same level as this repo) and recompile with cargo.
In future, the paths will not be hard-coded

## CLI:
```
Usage: aoc [[year] day] [-p 1|2]

Arguments:
  [year]  Specify the year of the solution to run. Defaults to the current year
  [day]   Specify the day of the solution to run. Defaults to today. Must be specified if year is.

Options:
  -p, --part <part>  Specify the part of the solution to run. If not specified, both parts are run.
  -h, --help         Print help information
  -V, --version      Print version information
```
