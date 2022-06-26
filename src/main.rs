use std::{ env, process, fs };
use colored::*;
use regex::Regex; 

struct Arguments {
    target: String,
    replacement: String,
    input_file: String,
    output_file: String,
}

fn print_usage() {
    println!("{} - quickly replace a string with another in a file", "qrep".green().italic());
    println!("Usage: qrep <target string> <replacement string> <input file> <output file>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{} wrong number of arguments, expected 4, got {}", "Error:".red().bold(), args.len());
        process::exit(1);
    }

    Arguments { 
        target: args[0].clone(),
        replacement: args[1].clone(),
        input_file: args[2].clone(), 
        output_file: args[3].clone()
    }
}

fn replace(input: &str, target: &str, replacement: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(input, replacement).into_owned())
}


fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.input_file) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{} failed to read input file '{}': {}", "Error:".red().bold(), args.input_file, err);
            process::exit(1);
        }
    };

    let result = match replace(&data, &args.target, &args.replacement) {
        Ok(v) => v.to_owned(),
        Err(err) => {
            eprintln!("{} failed to replace with regular expression '{}' and replacement '{}': {}", "Error:".red().bold(), args.target, args.replacement, err);
            process::exit(1);
        }
    };

    if let Err(err) = fs::write(&args.output_file, &result) {
        eprintln!("{} failed to write data to output file '{}': {}", "Error:".red().bold(), args.output_file, err);
        process::exit(1);
    }

}
