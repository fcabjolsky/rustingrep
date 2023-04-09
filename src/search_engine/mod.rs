use std::collections::HashSet;

use regex::Regex;

use crate::Options;

#[derive(Debug, Clone)]
pub struct SearchEng {
    source: Vec<String>,
    opts: Options,
    tags: HashSet<usize>,
}

impl SearchEng {
    /// Creates a new seaerch engine for the file_path with options
    pub fn new_from_file(file_path: &str, options: Options) -> Self {
        let file = std::fs::read_to_string(file_path).expect("can't read the file");
        let content: Vec<String> = file.split("\n").map(str::to_owned).collect();
        SearchEng {
            source: content,
            opts: options,
            tags: HashSet::new(),
        }
    }

    /// Searches the pattern to_search in the current search engine
    pub fn search(&mut self, to_search: &str) {
        let pattern = Regex::new(to_search).expect("can't create the patter");

        for (i, line) in self.source.iter().enumerate() {
            if let Some(_) = pattern.find(line) {
                self.tags.insert(i);
            }
        }
    }

    /// Prints the resulsts found by the search engine
    pub fn print_results(&self) {
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

#[cfg(test)]
mod test {
    use super::SearchEng;
    use super::Options;

    #[test]
    fn search_for_lorem_in_test_file_finds_366_results() {
        let mut search_eng = SearchEng::new_from_file(
            "./test.txt",
            Options {
                ctx_lines: 0,
                print_line_numbers: true,
            },
        );
        search_eng.search("lorem");
        assert_eq!(search_eng.tags.len(), 61);
    }

    #[test]
    fn search_for_felipe_in_test_file2_finds_2_results() {
        let mut search_eng = SearchEng::new_from_file(
            "./test2.txt",
            Options {
                ctx_lines: 0,
                print_line_numbers: true,
            },
        );
        search_eng.search("Felipe");
        assert_eq!(search_eng.tags.len(), 2);
    }

    #[test]
    fn search_for_testing_in_test_file2_finds_1_results() {
        let mut search_eng = SearchEng::new_from_file(
            "./test2.txt",
            Options {
                ctx_lines: 0,
                print_line_numbers: true,
            },
        );
        search_eng.search("testing");
        assert_eq!(search_eng.tags.len(), 1);
    }
}
