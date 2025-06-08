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
use crate::{reverse::reverse_write, tokens::{string_to_rune, WordGroup}};

#[allow(unused)]
const ANSI_RESET: &str = "\x1B[0m";
#[allow(unused)]
const ANSI_WHITE: &str = "\x1B[97m";
#[allow(unused)]
const ANSI_GREY: &str = "\x1B[37m";
#[allow(unused)]
const ANSI_BLUE: &str = "\x1B[34m";

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

    /// Colorize output
    #[arg(short = 'C', long = "color", default_value_t = false)]
    color: bool,

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
                'ᚺ' => "0", 'ᚾ' => "1", 'ᛁ' => "2", 'ᛃ' => "3",
                'ᛈ' => "4", 'ᛇ' => "5", 'ᛉ' => "6", 'ᛊ' => "7",
                'ᛏ' => "8", 'ᛒ' => "9", 'ᛖ' => "a", 'ᛗ' => "b",
                'ᛚ' => "c", 'ᛜ' => "d", 'ᛞ' => "e", 'ᛟ' => "f",
                'ᚱ' => "*", 'ᚠ' => "z",
                'ᚲ' => "<", '×' => ">",
                'ᚢ' => "_",
                '\n' | ' ' => "",
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
            (_, _) => 0x80 + file.code.len()
        };
        let mut bytes = Vec::new();
        bytes.resize(size, 0);

        for i in 0..file.memory.len() {
            bytes[i] = file.memory[i];
        }
        for i in 0..file.code.len() {
            bytes[i + 0x80] = file.code[i];
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

    // TODO: Implement decompiling runes

    // TODO: Implement binary diffing
    // if let Some(diff_path) = args.diff {
    //     let diff_input = std::fs::read_to_string(diff_path).expect("Failed to read diff file");
    //     let mut diff_words: Vec<Word> = Vec::new();

    // }

    let mut output = String::new();

    let mem_groups = WordGroupConstructor::new(mem_words).construct();
    let code_groups = WordGroupConstructor::new(code_words).construct();

    const CHUNK_LIMIT: usize = 64;

    if args.chat {
        if words.len() > CHUNK_LIMIT {
            // Split long sequences into multiple commands.
            let mut offset = 0;
            let mut first = true;
            for chunk in words.chunks(CHUNK_LIMIT) {
                if !first { output += "\n"; }
                let mut chunk = chunk.to_vec();
                if first {
                    // If writing 0 to PC, write 0x40 instead.
                    if chunk[0x3d].value() == 0 {
                        chunk[0x3d] = Word::new(0x40);
                    }
                }
                let mut ctor = WordGroupConstructor::new(chunk);
                let groups = ctor.construct().unwrap();
                output += &write_command(first, false, offset, Some(words_to_string(groups, args.color)), None);
                offset += ctor.word_count;
                first = false;
            }
            output += " ! restart";
        } else {
            output = match (mem_groups, code_groups) {
                (Some(mem), None) => write_command(true, true, 0, Some(words_to_string(mem, args.color)), None),
                (None, Some(code)) => write_command(true, true, 0, None, Some(words_to_string(code, args.color))),
                (Some(mem), Some(code)) => write_command(true, true, 0, Some(words_to_string(mem, args.color)), Some(words_to_string(code, args.color))),
                _ => output
            }
        }
    } else {
        let all_groups = WordGroupConstructor::new(words).construct().unwrap();
        output = words_to_string(all_groups, args.color);
    }

    if let Some(output_path) = args.output {
        std::fs::write(output_path, output).expect("Failed to write output file");
    } else {
        println!("{}", output);
    }
    Ok(())
}

fn words_to_string(words: Vec<WordGroup>, color: bool) -> String {
    let mut out = String::new();
    let mut bright = true;
    for group in words {
        if color { out += if bright { ANSI_WHITE } else { ANSI_BLUE }; }
        out += group.to_string().as_str();
        if color { out += ANSI_RESET; }
        bright = !bright;
    }
    out
}

fn write_command(clear: bool, reset: bool, offset: u16, mem: Option<String>, code: Option<String>) -> String {
    let mut out = String::new();

    out += if clear { "!vm clear " }
           else     { "!vm " };

    match (offset, mem, code) {
        (0, Some(mem), Some(code)) => {
            out += "write ";
            out += &mem;
            out += " ! code ";
            out += &code;
        }
        (0, Some(mem), None) => {
            out += "write ";
            out += &mem;
        }
        (0x40, Some(code), None) |
        (0, None, Some(code)) => {
            out += "code ";
            out += &code;
        }
        (offset, Some(mem), None) => {
            out += "write ";
            out += make_rune_offset(offset).as_str();
            out += &mem;
        }
        _ => {} // ???
    }

    if reset { out += " ! reset" }

    out
}

#[test]
fn text_write_command() {
    assert_eq!(write_command(true,  false, 0,     Some("ᚾᛁᛃᛈ".to_string()), None), "!vm clear write ᚾᛁᛃᛈ");
    assert_eq!(write_command(true,  false, 0,     None, Some("ᚾᛁᛃᛈ".to_string())), "!vm clear code ᚾᛁᛃᛈ");
    assert_eq!(write_command(true,  true,  0,     Some("ᚾᛁᛃᛈ".to_string()), None), "!vm clear write ᚾᛁᛃᛈ ! reset");
    assert_eq!(write_command(true,  true,  0,     None, Some("ᚾᛁᛃᛈ".to_string())), "!vm clear code ᚾᛁᛃᛈ ! reset");
    assert_eq!(write_command(true,  true,  0,     Some("1234".to_string()), Some("5678".to_string())), "!vm clear write 1234 ! code 5678 ! reset");
    assert_eq!(write_command(true,  true,  0x40,  Some("1234".to_string()), None), "!vm clear code 1234 ! reset");
    assert_eq!(write_command(true,  true,  0x200, Some("ᚾᛁᛃᛈ".to_string()), None), "!vm clear write ᛁᚺᚺᚢᚾᛁᛃᛈ ! reset");
}

fn make_rune_offset(offset: u16) -> String {
    let mut off = offset;
    let mut out = String::new();
    while off > 0 {
        let diff = off.min(0xfff);
        out += string_to_rune(format!("{:0x}ᚢ", diff).as_str()).as_str();
        off -= diff;
    }
    out
}

#[test]
fn test_make_rune_offset() {
    assert_eq!(make_rune_offset(0x000), "");
    assert_eq!(make_rune_offset(0x001), "ᚾᚢ");
    assert_eq!(make_rune_offset(0x00f), "ᛟᚢ");
    assert_eq!(make_rune_offset(0x010), "ᚾᚺᚢ");
    assert_eq!(make_rune_offset(0x100), "ᚾᚺᚺᚢ");
    assert_eq!(make_rune_offset(0x123), "ᚾᛁᛃᚢ");
    assert_eq!(make_rune_offset(0x2000), "ᛟᛟᛟᚢᛟᛟᛟᚢᛁᚢ");
}

#[allow(unused)]
struct MemoryChunk {
    pub words: Vec<WordGroup>,
    pub offset: u16,
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
