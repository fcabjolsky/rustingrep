use std::collections::HashSet;

use clap::Parser;
use regex::Regex;

//TODO: GREP into the file directory

/// searches for patterns
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Pattern to search
    #[arg(short, long)]
    pattern: String,

    ///Input file
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "0")]
    ctx_lines: String,
}

fn main() {
    let args = Args::parse();
    let ctx_lines = str::parse::<usize>(&args.ctx_lines).unwrap_or_default();
    let pattern = Regex::new(&args.pattern).unwrap();
   
    let file = std::fs::read_to_string(args.input).unwrap();
    let content: Vec<&str> = file.split("\n").collect();

    let mut tags: HashSet<usize> = HashSet::new();

    for (i, line) in content.iter().enumerate() {
        let result = pattern.find(*line);
        match result {
            Some(_) => {
                tags.insert(i);
            }
            None => (),
        }
    }

    let mut tags: Vec<_> = tags.iter().collect();
    tags.sort();

    for tag in tags{
        let lower_ctx = (*tag).saturating_sub(ctx_lines);
        let upper_ctz = std::cmp::min (*tag + ctx_lines, content.len() - 1);
        
        for i in lower_ctx..=upper_ctz {
            let mut prefix = "";
            if i == *tag {
                prefix = ">>";
            }
            println!("{} {} {}", prefix, i + 1, content[i]);
        }
        println!("--------");
    }
}

