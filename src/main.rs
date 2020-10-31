use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;

const FILEPATH: &str = "jrnii.txt";

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Starting a new jrnii? Type in a note about your day!");
    }
    let file_path = Path::new(FILEPATH);

    let mut open_opts = match OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path) {
            Err(why) => match why.kind() {
                ErrorKind::NotFound => match File::create(file_path) {
                    Ok(fc) => fc,
                    Err(e) => panic!("Error creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Error opening the file: {:?}", other_error)
                },
            },
            Ok(opts) => {opts}
        };

    args.remove(0);

    let write_str = args.join(" ");
    if let Err(e) = writeln!(open_opts, "{}", write_str) {
        eprintln!("could nor write to file: {}", e)
    }
    // if second arg starts with -, then it is a flag, else it is new entry
}
