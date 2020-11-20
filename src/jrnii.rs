use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use chrono::prelude::*;

pub const DIR_NAME: &str = ".jrnii";
pub const FILE_NAME: &str = "jrnii.txt";

pub fn read_local() {
    let local_path = Path::new(FILE_NAME);
    let mut file = match File::open(&local_path) {
        Err(why) => match why.kind() {
            ErrorKind::NotFound => {
                println!("no local jrnii to read");
                return;
            },
            _ => panic!("could not open local jrnii: {}", why),
        },
        Ok(f) => f,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("file found but cannot read: {}", why),
        Ok(_) => print!("{}", s),
    }
}

pub fn write_global(parent_path: & std::path::PathBuf, args: Vec<String>) {
    let local = Local::now();
    let date_file_name = local.format("%Y-%m-%d.txt").to_string();
    let file_path = parent_path.join(date_file_name);
    let mut args = args;

    let mut open_opts = match OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path) {
            Err(why) => match why.kind() {  // missing both dir and file or just file
                ErrorKind::NotFound => match fs::create_dir(&parent_path) { // try making dir
                    Ok(_) => File::create(&file_path).unwrap(),
                    Err(dir_why) => match dir_why.kind() {  // already has dir, need file
                        ErrorKind::AlreadyExists => File::create(&file_path).unwrap(),
                        _ => panic!("error creating directory: {}", dir_why),
                    },
                },
                _ => panic!("error opening the file: {}", why)
            },
            Ok(opts) => {opts}
        };

    args.remove(0);
    args.remove(0);
    writeln!(open_opts, "{}", args.join(" ")).unwrap();
}

pub fn write_local(args: Vec<String>) {
    let mut args = args;
    let mut open_opts = match OpenOptions::new()
        .write(true)
        .append(true)
        .open(FILE_NAME) {
            Err(why) => match why.kind() {
                ErrorKind::NotFound => File::create(FILE_NAME).unwrap(),
                _ => panic!("Error opening the file: {}", why)
            },
            Ok(opts) => {opts}
        };
    args.remove(0);
    writeln!(open_opts, "{}", args.join(" ")).unwrap();
}