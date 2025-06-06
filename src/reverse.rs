use crate::word::Word;


pub fn reverse_write(omnom: String) -> Vec<Word> {
    let mut val: u16 = 0;
    let mut addr: u16 = 0;
    let order = [12u32,8,4,0];
    let unorder = [0, 12u32,8,4,0];
    let mut ofs_index = 0u32;
    let mut last_written = 0u16;
    let mut bytes: [u16; 0x1000] = [0; 0x1000];
    let mut high: u16 = 0;

    for bite in omnom.chars() {
        if let Some(hex_val) = match bite {
            '0'..='9' => Some((bite as u8 - b'0') as u16),
            // abcdef
            'A'..='F' => Some((bite as u8 - b'A' + 10) as u16),
            'a'..='f' => Some((bite as u8 - b'a' + 10) as u16),
            // uvwxyz
            'U'..='Z' => Some((bite as u8 - b'U' + 10) as u16),
            'u'..='z' => Some((bite as u8 - b'u' + 10) as u16),
            // skip forward in the address space
            // by N
            'ᚢ' => {
                let ofs = unorder[ofs_index as usize];
                val >>= ofs;
                if val == 0 { val = 1; }
                addr = addr.wrapping_add(val) % 0x1000;
                ofs_index = 0;
                val = 0;
                None
            }
            // skip forward a word without writting
            // and without affecting input
            'ᚨ' => {
                addr = addr.wrapping_add(1);
                None
            }
            // write 0 words, N times
            'ᚠ' => {
                let ofs = unorder[ofs_index as usize];
                val >>= ofs;
                if val == 0 { val = 1; }
                while val > 0 {
                    // vmproc.write_priv(addr, 0);
                    bytes[addr as usize] = 0;
                    high = high.max(addr);
                    addr = addr.wrapping_add(1) % 0x1000;
                    val -= 1;
                }
                ofs_index = 0;
                val = 0;
                None
            }
            // repeat the "last written" value 1 or N times
            'ᚱ' => {
                let ofs = unorder[ofs_index as usize];
                val >>= ofs;
                if val == 0 { val = 1; }
                while val > 0 {
                    // vmproc.write_priv(addr, last_written);
                    bytes[addr as usize] = last_written;
                    high = high.max(addr);
                    addr = addr.wrapping_add(1) % 0x1000;
                    val -= 1;
                }
                ofs_index = 0;
                val = 0;
                None
            }
            // right align and write current value
            '×' => {
                let ofs = unorder[ofs_index as usize];
                val >>= ofs;
                last_written = val;
                // vmproc.write_priv(addr, val);
                bytes[addr as usize] = last_written;
                high = high.max(addr);
                addr = addr.wrapping_add(1) % 0x1000;
                ofs_index = 0;
                val = 0;
                None
            },
            // left align and write current value
            'ᚲ' => {
                last_written = val;
                // vmproc.write_priv(addr, val);
                bytes[addr as usize] = last_written;
                high = high.max(addr);
                addr = addr.wrapping_add(1) % 0x1000;
                ofs_index = 0;
                val = 0;
                None
            }
            // alternate hex data, 0-F equiv
            // ᚺᚾ ᛁᛃ ᛈᛇ ᛉᛊ ᛏᛒ ᛖᛗ ᛚᛜ ᛞᛟ
            'ᚺ' => Some(0), 'ᚾ' => Some(1), 'ᛁ' => Some(2), 'ᛃ' => Some(3),
            'ᛈ' => Some(4), 'ᛇ' => Some(5), 'ᛉ' => Some(6), 'ᛊ' => Some(7),
            'ᛏ' => Some(8), 'ᛒ' => Some(9), 'ᛖ' => Some(10), 'ᛗ' => Some(11),
            'ᛚ' => Some(12), 'ᛜ' => Some(13), 'ᛞ' => Some(14), 'ᛟ' => Some(15),
            _ => None
        } {
            let ofs = order[ofs_index as usize];
            val |= hex_val << ofs;
            ofs_index += 1;
        }
        if ofs_index >= 4 {
            last_written = val;
            // vmproc.write_priv(addr, val);
            bytes[addr as usize] = last_written;
            high = high.max(addr);
            val = 0;
            addr += 1;
            ofs_index = 0;
        }
    }
    if ofs_index > 0 {
        // vmproc.write_priv(addr, val);
        bytes[addr as usize] = val;
        high = high.max(addr);
    }

    // println!("{:?}", bytes);

    // Return bytes[0..high] as a Vector
    let mut result = Vec::new();
    for i in 0..=high {
        result.push(Word::from(bytes[i as usize]));
    }
    result
}
