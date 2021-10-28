use structopt::StructOpt;
use std::io::{prelude::*, BufReader};
use std::fs::File;

#[derive(StructOpt)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let file = File::open(&args.path).expect("Could not read file.");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.contains(&args.pattern) {
                    println!("{}", line)
                }
            },
            Err(e) => println!("Error parsing line: {:?}", e),
        }
    }
}

