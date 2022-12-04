use std::{
    ffi::OsString,
    fs::{self, File},
    io::{self, Write},
};

use glob;

macro_rules! write_to_file {
    ($file:expr, $($arg:tt)*) => {
        if let Err(e) = writeln!($file.as_ref().unwrap(), $($arg)*) {
            println!("cargo:warning=Failed writing \"{}\" to file: \"{:?}\": {:?}", format!($($arg)*), $file, e);
        }
    };
}

const HEADER: &str = r"// ============================================================================
// DO NOT MODIFY THIS FILE!
// IT WILL BE OVERWRITTEN EVERY TIME THE SOURCE IS BUILT!
// ============================================================================
";

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src");
    let mut prev_year = OsString::new();
    let mut file: Option<File> = None;
    let mut years: Vec<String> = Vec::new();
    let mut days: Vec<String> = Vec::new();
    for path in glob::glob("./src/y*/d*")
        .expect("Failed to read glob pattern")
        .filter_map(|p| p.map(|p| p.with_extension("")).ok())
    {
        let mut comp = path.components();
        let y_name = comp.nth(1).unwrap().as_os_str();
        if prev_year != y_name {
            write_run_day(&file, &mut days);
            file = fs::File::create(path.parent().unwrap().join("mod.rs")).ok();
            prev_year = y_name.to_owned();
            years.push(y_name.to_str().unwrap().to_string());
            write_to_file!(file, "{}", HEADER);
        }
        let d_name = comp.next().unwrap().as_os_str().to_str().unwrap();
        days.push(d_name.to_string());
        write_to_file!(file, "pub mod {};", d_name);
    }
    write_run_day(&file, &mut days);
    if let Err(e) = write_dispatcher(years) {
        println!("cargo:warning=Failed writing to \"main.rs\": {:?}", e);
    }
}

fn write_run_day(file: &Option<File>, days: &mut Vec<String>) {
    if days.len() > 0 {
        if file.is_some() {
            write_to_file!(
                file,
                "\npub fn run_day(day: u8, part: crate::RunPart) {{\n    match day {{"
            );
            for day in days.iter() {
                write_to_file!(
                    file,
                    "        {} => {}::main(part),",
                    (&day[1..]).parse::<u8>().unwrap(),
                    day
                );
            }
            write_to_file!(file, "        _ => panic!(\"Invalid day: {{}} does not have a solution\", day),\n    }}\n}}");
        }
        *days = Vec::new();
    }
}

fn write_dispatcher(years: Vec<String>) -> Result<(), io::Error> {
    if years.len() > 0 {
        let mut f = fs::File::create("./src/main.rs").unwrap();
        write!(
            f,
            r"{}

mod cli;
pub use cli::RunPart;

fn main() {{
    cli::main()
}}

pub fn run(year: u16, day: u8, part: crate::RunPart) {{
    match year {{
",
            HEADER
        )?;
        for year in years.iter() {
            writeln!(
                f,
                "        {} => {}::run_day(day, part),",
                (&year[1..]).parse::<u16>().unwrap(),
                year
            )?;
        }
        writeln!(
            f,
            r#"        _ => panic!("Invalid year: no solutions have been written for {{}}", year),
            }}
        }}"#
        )?;
        for year in years {
            writeln!(f, "pub mod {};", year)?;
        }
    }
    Ok(())
}
