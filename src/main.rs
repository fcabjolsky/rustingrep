use std::collections::HashSet;

use clap::Parser;
use regex::Regex;

//TODO: GREP into the file directory

#[derive(Debug, Clone)]
struct Options {
    ctx_lines: usize,
    print_line_numbers: bool,
}

#[derive(Debug, Clone)]
struct SearchEng {
    source: Vec<String>,
    opts: Options,
    tags: HashSet<usize>,
}

impl SearchEng {
    /// creates a new seaerch engine for the file_path with options
    fn new_from_file(file_path: &str, options: Options) -> Self {
        let file = std::fs::read_to_string(file_path).expect("can't read the file");
        let content: Vec<String> = file.split("\n").map(str::to_string).collect();
        SearchEng {
            source: content,
            opts: options,
            tags: HashSet::new(),
        }
    }

    /// searches the pattern to_search in the current search engine 
    fn search(&mut self, to_search: &str) {
        let pattern = Regex::new(to_search).expect("can't create the patter");

        for (i, line) in self.source.iter().enumerate() {
            if let Some(_) = pattern.find(line) {
                    self.tags.insert(i);
            }
        }

    }
    
    /// prints the resulsts found by the search engine
    fn print_results(&self) {
        let mut tags: Vec<_> = self.tags.iter().collect();
        tags.sort();

        for tag in tags {
            let lower_ctx = (*tag).saturating_sub(self.opts.ctx_lines);
            let upper_ctz = std::cmp::min(*tag + self.opts.ctx_lines, self.source.len() - 1);

            for i in lower_ctx..=upper_ctz {
                let mut prefix = "";
                let mut line_number = "".to_string();
                if i == *tag {
                    prefix = ">>";
                }
                if self.opts.print_line_numbers {
                    line_number = (i + 1).to_string();

                }

                println!("{} {} {}", prefix, line_number, self.source[i]);
            }
            println!("--------");
        }

    }
}

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
    ctx_lines: usize,

    #[arg(short = 'l', long, default_value = "false")]
    print_line_numbers: bool,

}

fn main() {
    let args = Args::parse();

    let mut search_eng = SearchEng::new_from_file(
        &args.input,
        Options {
            ctx_lines: args.ctx_lines,
            print_line_numbers: args.print_line_numbers,
        },
    );
    search_eng.search(&args.pattern);
    search_eng.print_results();
}

