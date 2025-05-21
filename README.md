# Waverune
Tool for converting [Wave2](https://github.com/Meisaka/MeiVM2/) hex bytecode into runes.

Supports:
- Basic hex to rune conversion
- Conversion of [w2s binary files](https://github.com/zeb-hicks/wave2_assembler).
- Compress runs of zeroes
- Compress arbitrary values
- Sparse word high/low alignment
- Output chat commands for convenience

Todo:
- Add a deflate mode for rune -> hex conversion
- Clean this garbage fire of a codebase up

### Usage:
```
waverune [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input file

Options:
  -b, --binary           Read input file as Wave2 binary format
  -c, --chat             Output as chat command
  -o, --output <OUTPUT>  Output file path
  -h, --help             Print help
  -V, --version          Print version
```

### Example conversion:

```
0000 0000 0000 0000 ffff ffff ffff ffff
dead beef cafe f00d 0042 0000 0000 0001
```
Becomes:
```
ᛈᚠᛟᛟᛟᛟᛃᚱᛜᛞᛖᛜᛗᛞᛞᛟᛚᛖᛟᛞᛟᚺᚺᛜᚺᚺᛈᛁᛁᚠᚾ×
```

## Install

Make sure rust is installed, and then run:
```
cargo install --git https://github.com/zeb-hicks/waverune
```
