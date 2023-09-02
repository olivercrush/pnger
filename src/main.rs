use std::env;
use std::process;

use pnger::Args;
use pnger::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let args = Args::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //println!("in_file_path : {0}", args.in_file_path);
    //println!("out_file_path : {0}", args.out_file_path);

    run(args);
}