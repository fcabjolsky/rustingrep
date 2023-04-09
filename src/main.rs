mod search_engine;
use search_engine::SearchEng;

use clap::Parser;

//TODO: GREP into the file directory

#[derive(Debug, Clone)]
pub struct Options {
    ctx_lines: usize,
    print_line_numbers: bool,
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

    ///Number of context lines to print 
    #[arg(short, long, default_value = "0")]
    ctx_lines: usize,

    ///Print the line numbers along side the results
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

