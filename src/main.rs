#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

mod args;
mod parser;
use args::*;
use chrono;
use parser::*;
use std::time::Instant;

fn main() {
    let args = Arguments::parse();

    // println!("Filename: {}", args.filename);
    // println!("Path: {}", args.path);
    // let filenamepath: String = args.path + "/" + &args.filename;
    // println!("Full filename and path: {}", filenamepath);

    match args.action {
        ActionType::Extract(ref tags) => {
            println!(
                "Processing the following file: {}",
                args.filename.to_string_lossy().to_owned()
            );
            println!("Request to extract the following Tags: {:?}", tags);
        }
        ActionType::Filter(FilterCommand {
            ref criteria,
            ref relation,
        }) => {
            let rela: &str = relation.as_ref().unwrap();
            println!(
                "Request to filter for the following criteria: {:?} with relation: '{}'",
                criteria, rela
            );
        }
    }

    // Start the timer
    let start = Instant::now();

    let mut fparser = parser::Parser::new(&args);
    fparser.process_file();

    // Stop the timer
    let duration = start.elapsed();

    println!("File processing completed in {:?} ", duration);
}
