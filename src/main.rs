mod tokens;
mod word;
mod diff;
mod reverse;

use std::{io::Read, path::PathBuf};
use clap_stdin::{FileOrStdin, StdinError};
use word::Word;

use clap::Parser;
use tokens::WordGroupConstructor;

// use crate::tokens::{char_to_rune, WordGroup};
use crate::reverse::reverse_write;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(default_value = "-")]
    input: FileOrStdin<Vec<u8>>,
    /// Read input file as Wave2 binary format
    #[arg(short, long, default_value_t = false)]
    binary: bool,
    /// Output as chat command
    #[arg(short, long, default_value_t = false)]
    chat: bool,

    /// Reverse runic encoding into the raw bytes as written
    #[arg(short, long, default_value_t = false)]
    reverse: bool,

    /// Reverse encode the runes into a human readable representation
    #[arg(short='R', long="read",  default_value_t = false)]
    read_runes: bool,

    // #[arg(short, long)]
    // diff: Option<PathBuf>,

    /// Output file path
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), StdinError> {
    let args = Args::parse();

    let mut reader = args.input.into_reader()?;

    let words;
    let mem_words;
    let code_words;

    if args.read_runes {
        let mut runes = String::new();
        reader.read_to_string(&mut runes)?;

        for c in runes.chars() {
            print!("{}", match c {

                _ => "?"
            });
        }
    }
    if args.reverse {
        let mut runes = String::new();
        reader.read_to_string(&mut runes)?;

        let words = reverse_write(runes);

        for chunk in words.chunks(16) {
            let values: Vec<String> = chunk.iter().map(|word| word.to_string()).collect();
            println!("{}", values.join(", "));
        }
        return Ok(())
    }

    if args.binary {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let file = parse_binary_file(buffer).expect("Failed to parse binary file");

        let size = match (file.memory_start, file.code_start) {
            (0, 0) => 0,
            (_, 0) => file.memory.len(),
            (_, _) => 0x40 + file.code.len()
        };
        let mut bytes = Vec::new();
        bytes.resize(size, 0);

        for i in 0..file.memory.len() {
            bytes[i] = file.memory[i];
        }
        for i in 0..file.code.len() {
            bytes[i + 0x40] = file.code[i];
        }

        words = binary_to_words(bytes);
        mem_words = binary_to_words(file.memory);
        code_words = binary_to_words(file.code);
    } else {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;

        words = hex_to_words(input.clone());
        mem_words = hex_to_words(input.clone());
        code_words = Vec::new();
    }

    let mut output = String::new();

    // TODO: Implement decompiling runes

    // TODO: Implement binary diffing
    // if let Some(diff_path) = args.diff {
    //     let diff_input = std::fs::read_to_string(diff_path).expect("Failed to read diff file");
    //     let mut diff_words: Vec<Word> = Vec::new();

    // }

    if args.chat && mem_words.len() > 0 {
        // Format output as chat comman !vm clear write <mem> ! code <code> ! restart
        let mut mem_builder = WordGroupConstructor::new(mem_words);
        output.push_str("!vm clear write ");
        if let Some(groups) = mem_builder.construct() {
            for group in groups {
                output.push_str(&group.to_string())
            }
        }
        if code_words.len() > 0 {
            output.push_str(" ! code ");
            let mut code_builder = WordGroupConstructor::new(code_words);
            if let Some(groups) = code_builder.construct() {
                for group in groups {
                    output.push_str(&group.to_string())
                }
            }
        }
        output.push_str(" ! restart");
    } else if args.chat {
        // Assume the input starts at 0x0
        output.push_str("!vm clear write ");
        let mut group_builder = WordGroupConstructor::new(words);
        if let Some(groups) = group_builder.construct() {
            for group in groups {
                output.push_str(&group.to_string())
            }
        }
        output.push_str(" ! restart");
    } else {
        // Output raw runes
        let mut group_builder = WordGroupConstructor::new(words);
        if let Some(groups) = group_builder.construct() {
            for group in groups {
                output.push_str(&group.to_string())
            }
        }
    }

    if let Some(output_path) = args.output {
        std::fs::write(output_path, output).expect("Failed to write output file");
    } else {
        println!("{}", output);
    }
    Ok(())
}

struct BinaryFile {
    pub header: Vec<u8>,
    pub memory_start: usize,
    pub code_start: usize,
    pub memory: Vec<u8>,
    pub code: Vec<u8>,
}

fn parse_binary_file(bytes: Vec<u8>) -> Result<BinaryFile, String> {
    let mut file = BinaryFile {
        header: Vec::new(),
        memory: Vec::new(),
        memory_start: 0,
        code_start: 0,
        code: Vec::new(),
    };

    // Check header
    if bytes.len() < 7 {
        return Err("Malformed header, file too short.".to_string());
    }

    let magic = &bytes[0..4];
    if magic != b"MWvm" {
        return Err("Invalid magic number.".to_string());
    }

    let version = &bytes[4];

    match version {
        0 | 1 => {
            let mem_start = bytes[5] as usize;
            let code_start = bytes[6] as usize;

            let mem_size = match (mem_start, code_start) {
                (0, _) => 0,
                (_, 0) => bytes.len() - mem_start,
                (_, _) => code_start - mem_start
            };
            let code_size = match (mem_start, code_start) {
                (_, 0) => 0,
                (_, _) => bytes.len() - code_start
            };

            file.header = bytes[0..7].to_vec();

            file.memory = bytes[mem_start..mem_start + mem_size].to_vec();
            file.code = bytes[code_start..code_start + code_size].to_vec();
            file.memory_start = mem_start;
            file.code_start = code_start;
        },
        2 => {
            todo!();
        },
        _ => return Err(format!("Unsupported version {version}.")),
    }

    Ok(file)
}

#[test]
fn test_bin_file_loader() {
    let bytes = b"MWvm\x01\x00\x00";
    let file = parse_binary_file(bytes.to_vec()).unwrap();
    assert_eq!(file.header, b"MWvm\x01\x00\x00");
    assert_eq!(file.memory, b"");
    assert_eq!(file.code, b"");

    let bytes = b"MWvm\x01\x07\x08\x01\x02";
    let file = parse_binary_file(bytes.to_vec()).unwrap();
    assert_eq!(file.header, b"MWvm\x01\x07\x08");
    assert_eq!(file.memory, b"\x01");
    assert_eq!(file.code, b"\x02");

    let bytes = b"MWvm\x01\x07\x0b12345678";
    let file = parse_binary_file(bytes.to_vec()).unwrap();
    assert_eq!(file.header, b"MWvm\x01\x07\x0b");
    assert_eq!(file.memory, b"1234");
    assert_eq!(file.code, b"5678");
}

fn hex_to_words(hex_string: String) -> Vec<Word> {
    let mut words = Vec::new();

    let hex_string = hex_string.trim().replace(" ", "").replace("\n", "");

    for i in (0..hex_string.len()).step_by(4) {
        let word_str = &hex_string[i..(i + 4).min(hex_string.len())];
        let word = Word::new(u16::from_str_radix(word_str, 16).unwrap());
        words.push(word);
    }

    words
}

#[test]
fn test_hex_to_words() {
    let hex_string = "12345678".to_string();
    let words = hex_to_words(hex_string);
    assert_eq!(words.len(), 2);
    assert_eq!(words[0].value(), 0x1234);
    assert_eq!(words[1].value(), 0x5678);
}

fn binary_to_words(input: Vec<u8>) -> Vec<Word> {
    let mut words = Vec::new();

    for chunk in input.chunks_exact(2) {
        let word = Word::new(u16::from_be_bytes([chunk[0], chunk[1]]));
        words.push(word);
    }

    words
}

#[test]
fn test_binary_to_words() {
    let input = vec![0x12, 0x34, 0x56, 0x78];
    let words = binary_to_words(input);
    assert_eq!(words.len(), 2);
    assert_eq!(words[0].value(), 0x1234);
    assert_eq!(words[1].value(), 0x5678);
}
