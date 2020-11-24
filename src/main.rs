use std::env;
use dirs::home_dir;

fn main() {
    let mut parent_path = home_dir().unwrap();
    parent_path.push(jrnii::DIR_NAME);   // file stored under $HOME/.jrnii 

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Starting a new jrnii? Type in a note about your day!");
        return;
    } 

    match args[1].as_str() {
        "-j" => jrnii::write_global(&parent_path, args),
        "-r" => jrnii::read_global_from_date(&parent_path, args),
        "-rl" => jrnii::read_local(),
        "-rt" => jrnii::read_global_today(&parent_path),
        _ => jrnii::write_local(args),
    }
}
