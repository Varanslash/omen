use std::fs;
use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let code = fs::read_to_string(argv[1].clone()).expect("Unable to read file");
    let mut codeout: Vec<u8> = Vec::new();
    for line in code.lines() {
        let sline = line.split_whitespace().collect::<Vec<&str>>();
        if sline.is_empty() { continue; }
        match sline[0].to_uppercase().as_str() {
            "NOP" => {
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Every instruction is 10 bytes long, so NOP is just 10 zero bytes.
            }
            "ADD" => {
                codeout.push(0x01);
                codeout.push(sline[1].parse::<u8>().expect("Error/ADD: Invalid value")); // u8
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "SUB" => {
                codeout.push(0x02);
                codeout.push(sline[1].parse::<u8>().expect("Error/SUB: Invalid value")); // u8
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "MUL" => {
                codeout.push(0x03);
                codeout.push(sline[1].parse::<u8>().expect("Error/MUL: Invalid value")); // u8
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "DIV" => {
                codeout.push(0x04);
                codeout.push(sline[1].parse::<u8>().expect("Error/DIV: Invalid value")); // u8
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
            }
            "MOD" => {
                codeout.push(0x05);
                codeout.push(sline[1].parse::<u8>().expect("Error/MOD: Invalid value")); // u8
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
            }
            "JMP" => {
                codeout.push(0x10);
                codeout.extend_from_slice(&sline[1].parse::<u32>().expect("Error/JMP: Invalid value").to_le_bytes()); // Code Referral
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "JPIF" => {
                codeout.push(0x11);
                codeout.extend_from_slice(&sline[1].parse::<u32>().expect("Error/JPIF: Invalid value").to_le_bytes()); // Code Referral
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "CALL" => {
                codeout.push(0x12);
                codeout.extend_from_slice(&sline[1].parse::<u32>().expect("Error/CALL: Invalid value").to_le_bytes()); // Code Referral
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "CLIF" => {
                codeout.push(0x13);
                codeout.extend_from_slice(&sline[1].parse::<u32>().expect("Error/CLIF: Invalid value").to_le_bytes()); // Code Referral
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "RET" => {
                codeout.push(0x14);
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "GOTO" => {
                codeout.push(0x15);
                codeout.extend_from_slice(&sline[1].parse::<u64>().expect("Error/GOTO: Invalid value").to_le_bytes()); // Absolute address
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "GTIF" => {
                codeout.push(0x16);
                codeout.extend_from_slice(&sline[1].parse::<u64>().expect("Error/GTIF: Invalid value").to_le_bytes()); // Absolute address
                codeout.push(0x00); // Padding to make the instruction 10 bytes long.
            }
            "JPFF" => {
                codeout.push(0x17);
                codeout.extend_from_slice(&sline[1].parse::<u32>().expect("Error/JPFF: Invalid value").to_le_bytes()); // Code Referral
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "PUSH" => {
                codeout.push(0xC0);
                codeout.extend_from_slice(&sline[1].parse::<u64>().expect("Error/PUSH: Invalid value").to_le_bytes()); // Const
                codeout.push(0x00); // Padding to make the instruction 10 bytes long.
            }
            "LOAD" => {
                codeout.push(0xC1);
                codeout.extend_from_slice(&sline[1].parse::<u32>().expect("Error/LOAD: Invalid value").to_le_bytes()); // Data Referral
                match sline.get(2) {
                    Some(_) => {
                        codeout.extend_from_slice(&sline[2].parse::<u32>().expect("Error/LOAD: Invalid value").to_le_bytes()); // Code Referral
                    }
                    None => {
                        codeout.extend_from_slice(&sline[1].parse::<u32>().expect("Error/LOAD: Invalid value").to_le_bytes()); // Padding to make the instruction 10 bytes long.
                    }
                }
                codeout.push(0x00); // Padding to make the instruction 10 bytes long.
            }
            "STR" => {
                codeout.push(0xC2);
                let flag = match sline[1].to_uppercase().as_str() {
                    "MAIN" => 0x00,
                    "WRAM" => 0x01,
                    "VRAM" => 0x02,
                    "OAM" => 0x03,
                    _ => panic!("Error/STR: Invalid flag"),
                };
                codeout.push(flag);
                codeout.extend_from_slice(&sline[2].parse::<u32>().expect("Error/STR: Invalid value").to_le_bytes()); // Data Referral, but placement and not extraction
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "STRR" => {
                codeout.push(0xC3);
                codeout.push(sline[1].parse::<u8>().expect("Error/STRR: Invalid value")); // Register number
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "DREF" => {
                codeout.push(0xC4);
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "LODR" => {
                codeout.push(0xC5);
                codeout.push(sline[1].parse::<u8>().expect("Error/LODR: Invalid value")); // Register number
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "CMP" => {
                codeout.push(0xE0);
                let condition = match sline[1].to_uppercase().as_str() {
                    "EQ" => 0x00,
                    "NE" => 0x01,
                    "GT" => 0x02,
                    "LT" => 0x03,
                    "GE" => 0x04,
                    "LE" => 0x05,
                    _ => panic!("Error/CMP: Invalid condition"),
                };
                codeout.push(condition);
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "NOT" => {
                codeout.push(0xE1);
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "AND" => {
                codeout.push(0xE2);
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "OR" => {
                codeout.push(0xE3);
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "XOR" => {
                codeout.push(0xE4);
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "XNOR" => {
                codeout.push(0xE5);
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "NOR" => {
                codeout.push(0xE6);
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "NAND" => {
                codeout.push(0xE7);
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "DRAW" => {
                codeout.push(0xF0);
                codeout.extend_from_slice(&sline[1].parse::<u32>().expect("Error/DRAW: Invalid value").to_le_bytes()); // Data Referral
                match sline.get(2) {
                    Some(_) => {
                        codeout.extend_from_slice(&sline[2].parse::<u32>().expect("Error/DRAW: Invalid value").to_le_bytes());
                    }
                    None => {
                        codeout.extend_from_slice(&sline[1].parse::<u32>().expect("Error/DRAW: Invalid value").to_le_bytes());
                    }
                }
                codeout.push(0x00); // Padding to make the instruction 10 bytes long.
            }
            "INPT" => {
                codeout.push(0xF1);
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "WAIT" => {
                codeout.push(0xF2);
                codeout.extend_from_slice(&sline[1].parse::<u16>().expect("Error/WAIT: Invalid value").to_le_bytes());
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "PLAY" => {
                codeout.push(0xF3);
                codeout.extend_from_slice(&sline[1].parse::<u32>().expect("Error/PLAY: Invalid value").to_le_bytes());
                match sline.get(2) {
                    Some(_) => {
                        codeout.extend_from_slice(&sline[2].parse::<u32>().expect("Error/PLAY: Invalid value").to_le_bytes());
                    }
                    None => {
                        codeout.extend_from_slice(&sline[1].parse::<u32>().expect("Error/PLAY: Invalid value").to_le_bytes());
                    }
                }
                codeout.push(0x00); // Padding to make the instruction 10 bytes long.
            }
            "PLRW" => {
                codeout.push(0xF4);
                codeout.extend_from_slice(&sline[1].parse::<u32>().expect("Error/PLRW: Invalid value").to_le_bytes());
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "DI" => {
                codeout.push(0xF5);
                codeout.push(sline[1].parse::<u8>().expect("Error/DI: Invalid value"));
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "EI" => {
                codeout.push(0xF6);
                codeout.push(sline[1].parse::<u8>().expect("Error/EI: Invalid value"));
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "DIS" => {
                codeout.push(0xF7);
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "EIS" => {
                codeout.push(0xF8);
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            "INT" => {
                codeout.push(0xF9);
                codeout.push(sline[1].parse::<u8>().expect("Error/INT: Invalid value"));
                codeout.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Padding to make the instruction 10 bytes long.
            }
            _ => panic!("Error: Invalid instruction {}", sline[0]),
        }
    }
}

/*

00 - NOP - Does nothing.
01 - ADD u8 - Adds all values up to u8 slots deep into the stack and pushes the result back onto the stack.
02 - SUB u8 - Subtracts all values up to u8 slots deep into the stack and pushes the result back.
03 - MUL u8 - Multiplies all values up to u8 slots deep into the stack and pushes the result back.
04 - DIV u8 - Divides all values up to u8 slots deep into the stack and pushes the result back.
05 - MOD u8 - Computes the modulus of all values up to u8 slots deep into the stack and pushes the result back.
10 - JMP ref - Jumps to the referral specified.
11 - JPIF ref - Jumps to the referral specified if the top of the stack is 0x01.
12 - CALL ref - Calls the referral specified, pushing the current address onto the stack as a return address.
13 - CLIF ref - Calls the referral specified if the top of the stack is 0x01, pushing the current address onto the stack as a return address.
14 - RET - Returns from the current function, popping the return address from the stack and jumping to it.
15 - GOTO address - Jumps to the specified address.
16 - GTIF address - Jumps to the specified address if the top of the stack is 0x01. 
17 - JPFF ref - Jumps to the referral specified if the top of the stack is 0x00.
C0 - PUSH data - Pushes the specified data onto the stack.
C1 - LOAD ref - Loads the data at the referral specified onto the stack.
C2 - STR flag ref - Stores the top of the stack at the referral specified, in the zone specified by the flag. Flags are as follows: 00 => main memory, 01 => WRAM, 02 => VRAM (includes color ram), 03 => OAM
C3 - STRR reg - Stores the top of the stack at the specified register.
C4 - DREF - Pops the top of the stack and treats the bottom 32 bits as a referral, loading the data at that referral onto the stack.
C5 - LODR - Loads a register's value onto the stack.
E0 - CMP condition - Pops the top two values from the stack and compares them according to the specified condition (e.g., equal, not equal, greater than, less than), pushing the result (0x01 for true, 0x00 for false) back 
onto the stack.
E1 - NOT - Pops the top of the stack, performs a bitwise NOT operation on it, and pushes the result back onto the stack.
E2 - AND - Pops the top two values from the stack, performs a bitwise AND operation on them.
E3 - OR - Pops the top two values from the stack, performs a bitwise OR operation on them, and pushes the result back onto the stack.
E4 - XOR - Pops the top two values from the stack, performs a bitwise XOR operation on them, and pushes the result back onto the stack.
E5 - XNOR - Pops the top two values from the stack, performs a bitwise XNOR operation on them, and pushes the result back onto the stack.
E6 - NOR - Pops the top two values from the stack, performs a bitwise NOR operation on them, and pushes the result back onto the stack.
E7 - NAND - Pops the top two values from the stack, performs a bitwise NAND operation on them, and pushes the result back onto the stack.
F0 - DRAW ref - Takes all tile/palette data from the specified referral or referral range and draws it at the %cursorX and %cursorY values.
F1 - INPT - Reads input from the user and pushes it onto the stack.
F2 - WAIT u16 - Waits for a specific interrupt or scanline, specified by the 16-bit unsigned integer operand, then handles the interrupt. The 16 bit interrupts are as follows: 0x00// => an HBlank scanline, 0x01// => 
VBlank, 0x02// => Any HBlank
F3 - PLAY ref - Plays the sound stored at the ref its operand points to. (ex: PLAY 0x00001000 \[0x00001000: { 0x80000000 }\] => plays sound at 80000000)
F4 - PLRW ref - Plays the sound stored at the ref. (ex: PLRW 0x00001000 => plays sound at 00001000)
F5 - DI int - Disables a certain interrupt.
F6 - EI int - Enables a certain interrupt.
F7 - DIS - Disables all interrupts.
F8 - EIS - Enables all interrupts.
F9 - INT - Forces a certain interrupt. 

*/