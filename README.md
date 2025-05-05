# Waverune
Tool for converting [WaveVM](https://github.com/Meisaka/MeiVM2/) hex bytecode into runes.

Supports:
- Basic hex to rune conversion
- Repeat runs of zeroes
- Repeat arbitrary values
- Sparse word high/low alignment

Todo:
- Add a deflate mode for rune -> hex conversion
- Clean this garbage fire of a codebase up

### Usage:
`waverune <input_file> [-o <output_file>]`

### Example conversion:

```
0000 0000 0000 0000 ffff ffff ffff ffff
dead beef cafe f00d 0042 0000 0000 0001
```
Becomes:
```
ᛈᚠᛟᛟᛟᛟᛃᚱᛜᛞᛖᛜᛗᛞᛞᛟᛚᛖᛟᛞᛟᚺᚺᛜᚺᚺᛈᛁᛁᚠᚾ×
```