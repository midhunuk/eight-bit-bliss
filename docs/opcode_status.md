## 6502 Opcode Implementation Tracker

### Addressing Mode Abbreviations

| Abbreviation | Name                | Description |
|--------------|---------------------|-------------|
| IMM          | Immediate           | Operand is a constant value |
| ZP           | Zero Page           | Address in first 256 bytes (0x0000–0x00FF) |
| ZP,X         | Zero Page,X         | Zero page address offset by X register |
| ZP,Y         | Zero Page,Y         | Zero page address offset by Y register |
| ABS          | Absolute            | Full 16-bit address |
| ABS,X        | Absolute,X          | Absolute address offset by X register |
| ABS,Y        | Absolute,Y          | Absolute address offset by Y register |
| IND          | Indirect            | Address read from memory (used by JMP) |
| IND,X        | Indexed Indirect    | Zero page pointer + X, then dereferenced |
| IND,Y        | Indirect Indexed    | Zero page pointer dereferenced, then + Y |
| ACC          | Accumulator         | Operation performed on accumulator register |
| IMP          | Implied             | No operand (implicit in instruction) |
| REL          | Relative            | Signed offset for branch instructions |

| Opcode | Implemented | Tested | Addressing Modes Done |
|--------|------------|--------|------------------------|
| ADC | ✅ | ✅ | IMM, ZP, ZP,X, ABS, ABS,X, ABS,Y, IND,X, IND,Y |
| AND | ❌ | ❌ |  |
| ASL | ❌ | ❌ |  |
| BCC | ❌ | ❌ |  |
| BCS | ❌ | ❌ |  |
| BEQ | ❌ | ❌ |  |
| BIT | ❌ | ❌ |  |
| BMI | ❌ | ❌ |  |
| BNE | ❌ | ❌ |  |
| BPL | ❌ | ❌ |  |
| BRK | ❌ | ❌ |  |
| BVC | ❌ | ❌ |  |
| BVS | ❌ | ❌ |  |
| CLC | ❌ | ❌ |  |
| CLD | ❌ | ❌ |  |
| CLI | ❌ | ❌ |  |
| CLV | ❌ | ❌ |  |
| CMP | ❌ | ❌ |  |
| CPX | ❌ | ❌ |  |
| CPY | ❌ | ❌ |  |
| DEC | ❌ | ❌ |  |
| DEX | ❌ | ❌ |  |
| DEY | ❌ | ❌ |  |
| EOR | ❌ | ❌ |  |
| INC | ❌ | ❌ |  |
| INX | ✅ | ✅ | IMP |
| INY | ❌ | ❌ |  |
| JMP | ❌ | ❌ |  |
| JSR | ❌ | ❌ |  |
| LDA | ✅ | ✅ | IMM, ZP, ZP,X, ABS, ABS,X, ABS,Y, IND,X, IND,Y |
| LDX | ✅ | ✅ | IMM, ZP, ZP,Y, ABS, ABS,Y |
| LDY | ✅ | ✅ | IMM, ZP, ZP,X, ABS, ABS,X |
| LSR | ❌ | ❌ |  |
| NOP | ❌ | ❌ |  |
| ORA | ❌ | ❌ |  |
| PHA | ❌ | ❌ |  |
| PHP | ❌ | ❌ |  |
| PLA | ❌ | ❌ |  |
| PLP | ❌ | ❌ |  |
| ROL | ❌ | ❌ |  |
| ROR | ❌ | ❌ |  |
| RTI | ❌ | ❌ |  |
| RTS | ❌ | ❌ |  |
| SBC | ❌ | ❌ |  |
| SEC | ❌ | ❌ |  |
| SED | ❌ | ❌ |  |
| SEI | ❌ | ❌ |  |
| STA | ❌ | ❌ |  |
| STX | ❌ | ❌ |  |
| STY | ❌ | ❌ |  |
| TAX | ✅ | ✅ | IMP |
| TAY | ❌ | ❌ |  |
| TSX | ❌ | ❌ |  |
| TXA | ❌ | ❌ |  |
| TXS | ❌ | ❌ |  |
| TYA | ❌ | ❌ |  |