mod tokens;
mod word;

use std::path::PathBuf;
use crate::word::Word;

use clap::Parser;
use tokens::{parse_word_groups, WordGroup, WordGroupConstructor, WordReader};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    input: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn runes_from_words(words: Vec<Word>) -> String {
    let mut output = String::new();
    let mut witer = words.iter();

    while match witer.next() {
        Some(word) => {
            true
        },
        None => false,
    }{}

    output
}

fn main() {
    let args = Args::parse();

    let input = std::fs::read_to_string(&args.input).expect("Failed to read input file");
    let mut words: Vec<Word> = Vec::new();
    let mut word_buffer: String = String::new();
    let mut output = String::new();

    for line in input.lines() {
        for c in line.chars() {
            match c {
                'a'..='f' | 'A'..='F' | '0'..='9' => {
                    word_buffer.push(c);
                }
                _ => ()
            }
            if word_buffer.len() == 4 {
                let word = Word::new(u16::from_str_radix(&word_buffer, 16).unwrap());
                words.push(word);
                word_buffer.clear();
            }
        }
    }

    let mut group_builder = WordGroupConstructor::new(words);
    if let Some(groups) = group_builder.construct() {
        for group in groups {
            output.push_str(&group.to_string())
        }
    }
    
    if let Some(output_path) = args.output {
        std::fs::write(output_path, output).expect("Failed to write output file");
    } else {
        println!("{}", output);
    }
}
