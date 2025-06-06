use std::fmt::Display;

use crate::word::Word;

pub fn char_to_rune(c: char) -> Option<char> {
    match c {
        '0' => Some('ᚺ'),
        '1' => Some('ᚾ'),
        '2' => Some('ᛁ'),
        '3' => Some('ᛃ'),
        '4' => Some('ᛈ'),
        '5' => Some('ᛇ'),
        '6' => Some('ᛉ'),
        '7' => Some('ᛊ'),
        '8' => Some('ᛏ'),
        '9' => Some('ᛒ'),
        'a' | 'A' => Some('ᛖ'),
        'b' | 'B' => Some('ᛗ'),
        'c' | 'C' => Some('ᛚ'),
        'd' | 'D' => Some('ᛜ'),
        'e' | 'E' => Some('ᛞ'),
        'f' | 'F' => Some('ᛟ'),
        '*' => Some('ᚱ'),
        'z' => Some('ᚠ'),
        '<' => Some('ᚲ'),
        '>' => Some('×'),
        _=> None,
    }
}

#[allow(unused)]
pub fn rune_to_char(c: char) -> Option<char> {
    match c {
        'ᚺ' => Some('0'),
        'ᚾ' => Some('1'),
        'ᛁ' => Some('2'),
        'ᛃ' => Some('3'),
        'ᛈ' => Some('4'),
        'ᛇ' => Some('5'),
        'ᛉ' => Some('6'),
        'ᛊ' => Some('7'),
        'ᛏ' => Some('8'),
        'ᛒ' => Some('9'),
        'ᛖ' => Some('a'),
        'ᛗ' => Some('b'),
        'ᛚ' => Some('c'),
        'ᛜ' => Some('d'),
        'ᛞ' => Some('e'),
        'ᛟ' => Some('f'),
        'ᚱ' => Some('*'),
        'ᚠ' => Some('z'),
        'ᚲ' => Some('<'),
        '×' => Some('>'),
        _=> None,
    }
}

pub fn string_to_rune(s: &str) -> String {
    let mut output = String::new();
    for c in s.chars() {
        let rune = char_to_rune(c);
        match rune {
            Some(r) => output.push(r),
            None => output.push(c),
        }
        // output.push(c);
    }
    output
}

#[allow(unused)]
pub fn to_word(s: &str) -> Result::<Word, String> {
    let mut word = 0;
    let mut i = 0;
    for c in s.chars() {
        if let Some(r) = rune_to_char(c) {
            match r {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    word <<= 4;
                    word |= r.to_digit(16).unwrap() as u16;
                    i += 1;
                }
                _ => {}
            }
        }
    }
    if i == 4 {
        Ok(Word::new(word))
    } else {
        Err(format!("Invalid word length: {}", i))
    }
}

#[allow(unused)]
pub fn rune_to_string(s: String) -> Result::<String, String> {
    let words: Vec<[char; 4]> = Vec::new();
    let mut output = String::new();
    let mut buffer: [char; 4] = ['0'; 4];
    let mut i = 0;
    for c in s.chars() {
        if let Some(r) = rune_to_char(c) {
            match r {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    buffer[i] = r;
                    i += 1;
                    if i == 4 {
                        for j in 0..4 {
                            output.push(buffer[j] as char);
                        }
                        i = 0;
                    }
                }
                'z' => {
                    if i == 0 {
                        output.push_str("0000");
                    } else {

                    }
                    let num = buffer.iter().take(i).collect::<String>();
                    if let Ok(num) = u16::from_str_radix(&num, 16) {
                        for _ in 0..num {
                            output.push_str("0000");
                        }
                    }
                }
                '*' => {
                    if i > 0 && words.len() > 0 {
                        let num = words.last().unwrap().iter().collect::<String>();
                        let num = Word::from(num);
                    } else {

                    }
                }
                _ => {}
            }
        }
    }
    Ok(output)
}

#[derive(Debug, Clone, Copy)]
pub enum WordGroup {
    #[allow(unused)]
    Skip,
    #[allow(unused)]
    SkipChain(usize),
    Zero,
    ZeroChain(usize),
    Word(Word),
    WordChain(Word, usize),
    LowNibble(u16, Option<usize>),
    LowByte(u16, Option<usize>),
    HighByte(u16, Option<usize>),
    HighNibble(u16, Option<usize>),
}

impl Display for WordGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hex = match self {
            WordGroup::Zero => "1z".to_string(),
            WordGroup::ZeroChain(count) => format!("{:x}z", count),
            WordGroup::Word(word) => format!("{:04x}", word.value()),
            WordGroup::WordChain(word, count) => format!("{:04x}{}*", word.value(), count),
            WordGroup::LowNibble(word, None) => format!("{:01x}>", word),
            WordGroup::LowByte(word, None) => format!("{:02x}>", word),
            WordGroup::HighByte(word, None) => format!("{:02x}<", word >> 8),
            WordGroup::HighNibble(word, None) => format!("{:01x}<", word >> 12),
            WordGroup::LowNibble(word, Some(count)) => format!("{:01x}>{}*", word, count),
            WordGroup::LowByte(word, Some(count)) => format!("{:02x}>{}*", word, count),
            WordGroup::HighByte(word, Some(count)) => format!("{:02x}<{}*", word >> 8, count),
            WordGroup::HighNibble(word, Some(count)) => format!("{:01x}<{}*", word >> 12, count),
        };
        write!(f, "{}", string_to_rune(&hex))
    }
}

#[test]
fn test_word_groups() {
    assert_eq!(string_to_rune(&WordGroup::ZeroChain(3).to_string()), "ᛃᚠ");
    assert_eq!(string_to_rune(&WordGroup::Word(Word::new(0xdead)).to_string()), "ᛜᛞᛖᛜ");
    assert_eq!(string_to_rune(&WordGroup::HighNibble(0x5000, None).to_string()), "ᛇᚲ");
    assert_eq!(string_to_rune(&WordGroup::LowNibble(0xf, None).to_string()), "ᛟ×");
    assert_eq!(string_to_rune(&WordGroup::HighByte(0x3300, None).to_string()), "ᛃᛃᚲ");
    assert_eq!(string_to_rune(&WordGroup::LowByte(0x42, None).to_string()), "ᛈᛁ×");
    assert_eq!(string_to_rune(&WordGroup::HighNibble(0x5000, Some(4)).to_string()), "ᛇᚲᛈᚠ");
    assert_eq!(string_to_rune(&WordGroup::LowNibble(0xf, Some(2)).to_string()), "ᛟ×ᛁᚠ");
    assert_eq!(string_to_rune(&WordGroup::HighByte(0x3300, Some(6)).to_string()), "ᛃᛃᚲᛉᚠ");
    assert_eq!(string_to_rune(&WordGroup::LowByte(0x42, Some(5)).to_string()), "ᛈᛁ×ᛇᚠ");
}

#[test]
fn test_word_group_reader() {
    let words = vec![
        Word::new(0x0000),
        Word::new(0x0001),
        Word::new(0x0001),
        Word::new(0x0002),
        Word::new(0x0002),
        Word::new(0x0002),
    ];

    let mut reader = WordReader {
        words: words.clone(),
        index: 0,
    };

    assert_eq!(reader.next(), Some(Word::new(0x0000)));
    assert_eq!(reader.peek(), Some(Word::new(0x0001)));
    assert_eq!(reader.count_ahead(|w| w == Word::new(0x0001)), 2);
}

pub struct WordReader {
    words: Vec<Word>,
    index: usize,
}

impl WordReader {
    pub fn new(words: Vec<Word>) -> Self {
        WordReader { words, index: 0 }
    }

    pub fn next(&mut self) -> Option<Word> {
        if self.index >= self.words.len() {
            return None;
        }

        let word = &self.words[self.index];
        self.index += 1;

        return Some(*word);
    }

    pub fn advance(&mut self, count: usize) {
        self.index += count;
    }

    #[allow(unused)]
    pub fn peek(&self) -> Option<Word> {
        if self.index >= self.words.len() {
            return None;
        }

        Some(self.words[self.index])
    }

    pub fn count_ahead<F: Fn(Word) -> bool>(&self, test: F) -> usize {
        let mut count = 0;
        let mut i = self.index;

        while i < self.words.len() {
            if test(self.words[i]) {
                count += 1;
            } else {
                break;
            }
            i += 1;
        }

        count
    }
}

pub struct WordGroupConstructor {
    reader: WordReader,
    groups: Vec<WordGroup>,
}

impl WordGroupConstructor {
    pub fn new(words: Vec<Word>) -> Self {
        WordGroupConstructor { reader: WordReader::new(words.clone()), groups: Vec::new() }
    }

    pub fn construct(&mut self) -> Option<Vec<WordGroup>> {
        while let Some(word) = self.reader.next() {
            let repeats = self.reader.count_ahead(|w| w == word);
            let repeat = if repeats > 0 { Some(repeats) } else { None };
            let w = word.value();
            let a = if (w & 0xf000) == w { 0b1000 } else { 0 };
            let b = if (w & 0x0f00) == w { 0b0100 } else { 0 };
            let c = if (w & 0x00f0) == w { 0b0010 } else { 0 };
            let d = if (w & 0x000f) == w { 0b0001 } else { 0 };
            let bits = a | b | c | d;
            self.groups.push(match bits {
                0b1000 => WordGroup::HighNibble(word.value() as u16, repeat),
                0b1100 => WordGroup::HighByte(word.value() as u16, repeat),
                0b0011 => WordGroup::LowByte(word.value() as u16, repeat),
                0b0001 => WordGroup::LowNibble(word.value() as u16, repeat),
                _ => {
                    match w {
                        0 => match repeats {
                            0 => WordGroup::Zero,
                            _ => WordGroup::ZeroChain(repeats + 1),
                        },
                        _ => match repeats {
                            0 => WordGroup::Word(word),
                            _ => WordGroup::WordChain(word, repeats),
                        }
                    }
                },
            });
            self.reader.advance(repeats);
        }

        Some(self.groups.clone())
    }
}


#[test]
fn test_vector() {
    let v = Word::new(0xdead);
    assert_eq!(v.to_string(), "dead");
}

#[test]
fn test_runes() {
    let rune = char_to_rune('a');
    assert_eq!(rune, Some('ᛖ'));
    let runes = "This is a test. 1234567890";
    let output = string_to_rune(runes);
    assert_eq!(output, "ᛖᛞᚾᛁᛃᛈᛇᛉᛊᛏᛒᚺ");
}

#[test]
fn test_alignments() {
    let input: Vec<Word> = vec![0x3000,0x0110,0x8006,0xf].into_iter().map(Word::new).collect();
    let output: String = format!("{:04x}", input[0].value());
    assert_eq!(output, "ᛃᚲᚺᚾᚾᚺᛏᚺᚺᛉᛟ×");
}

#[test]
fn test_zeroes() {
    let input: Vec<Word> = vec![0,0,0,0].into_iter().map(Word::new).collect();
    let output: String = format!("{:04x}", input[0].value());
    assert_eq!(output, "ᛈᚠ");
}

#[test]
fn test_repeats() {
    let input: Vec<Word> = vec![0xffff,0xffff,0xffff,0xffff].into_iter().map(Word::new).collect();
    let output: String = format!("{:04x}", input[0].value());
    assert_eq!(output, "ᛟᛟᛟᛟᛃᚱ");

    let input: Vec<Word> = vec![0x348c,0x348c,0x348c,0x348c].into_iter().map(Word::new).collect();
    let output: String = format!("{:04x}", input[0].value());
    assert_eq!(output, "ᛃᛈᛏᛚᛃᚱ");
}
