use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use dirs::home_dir;
use chrono::prelude::*;

const DIR_NAME: &str = ".jrnii";
const FILE_NAME: &str = "jrnii.txt";

fn main() {
    let mut parent_path = home_dir().unwrap();
    parent_path.push(DIR_NAME);   // file stored under $HOME/.jrnii 

    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Starting a new jrnii? Type in a note about your day!");
        return;
    } 

    if args[1] == "-r" {    // read local file 
        let local_path = Path::new(FILE_NAME);
        let mut file = match File::open(&local_path) {
            Err(why) => match why.kind() {
                ErrorKind::NotFound => {
                    println!("No local jrnii yet, enter a note after the keyword 'jrnii' to start one");
                    return;
                },
                _ => panic!("could not open local jrnii because: {}", why),
            },
            Ok(f) => f,
        };
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("has file but could not read because {}", why),
            Ok(_) => print!("{}", s),
        }

    } else if args[1] == "-j" { // write to global journal
        let local = Local::now();
        let date_file_name = local.format("%Y-%m-%d.txt").to_string();
        let file_path = parent_path.join(date_file_name);

        let mut open_opts = match OpenOptions::new()
            .write(true)
            .append(true)
            .open(&file_path) {
                Err(why) => match why.kind() {  // missing both dir and file or just file
                    ErrorKind::NotFound => match fs::create_dir(&parent_path) { // try making dir
                        Ok(_) => File::create(&file_path).unwrap(),
                        Err(dir_why) => match dir_why.kind() {  // already has dir, need file
                            ErrorKind::AlreadyExists => File::create(&file_path).unwrap(),
                            dir_other => panic!("Error creating directory: {:?}", dir_other),
                        },
                    },
                    other_error => panic!("Error opening the file: {:?}", other_error)
                },
                Ok(opts) => {opts}
            };

        args.remove(0);
        args.remove(0);
        writeln!(open_opts, "{}", args.join(" ")).unwrap();

    } else {
        let mut open_opts = match OpenOptions::new()
            .write(true)
            .append(true)
            .open(FILE_NAME) {
                Err(why) => match why.kind() {
                    ErrorKind::NotFound => File::create(FILE_NAME).unwrap(),
                    other_error => panic!("Error opening the file: {:?}", other_error)
                },
                Ok(opts) => {opts}
            };
        args.remove(0);
        writeln!(open_opts, "{}", args.join(" ")).unwrap();
    }
    
    // if second arg starts with -, then it is a flag, else it is new entry
}
