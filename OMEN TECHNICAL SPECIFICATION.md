# OMEN TECHNICAL SPECIFICATION

## INTRODUCTION

This specification defines the main design of the theoretical OMEN programming language. 

OMEN is a non-typed, procedural programming language designed to be as unsafe as possible. It is intended to be used on punch cards as a computer science project.
In OMEN, data is code, and code is data. This means that there is no distinction between the two, and they can be used interchangeably.

## GLOSSARY

- Address - A 32-bit*10 unsigned integer that represents a location in memory. Addresses go from 0x000000000..0x9FFFFFFF6
- Referral - A 32-bit unsigned integer that represents its own address and the next 9 addresses in memory. Referrals are used to reference data and code in memory. Referral ranges are able to be used through the period character (example: 0x00000000.0x00000009, or 0x00000000 0x2E 0x00000009 as an operand), and when used treat the entire range as a single referral.
- Instruction - A 8-bit unsigned integer that represents a specific operation to be performed by the computer.
- Data - A sequence of bits that can be interpreted as either code or data, depending on the context in which it is used.
- WRAM - Work RAM, a type of memory that is used for temporary storage of data and code during program execution.
- VRAM - Video RAM, a type of memory that is used for storing data related to graphics and display.
- CRAM - Color RAM, a type of memory that is used for storing data related to colors and palettes.
- OAM - Object Attribute Memory, a type of memory that is used for storing data related to sprites and objects.

## AMEN

AMEN is the CISC assembly language for OMEN. It is a low-level language that is used to write code that can be executed by the computer. AMEN instructions are represented as 8-bit unsigned integers, and they can be used to perform various operations on data and referrals.

If values are of different primitives (e.g, u80 vs u160), smaller numbers are padded with preceding zeros.

Operands 00 - 05 use the u8th number first (e.g., ADD 3 \[%stack: 35, 34, 23\] = 23 + 34 + 35 in this order).

### LIST OF INSTRUCTIONS

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


## SYNTAX

0xXXXXXXXX: \[block\] {} - Defines a referral or referral range at the specified address, with optional control flow instructions and a block of code or data enclosed in curly braces.

### BLOCKS

A block is a sequence of instructions and data that can be executed or referenced as a unit. Blocks are defined within curly braces {} and can contain any valid AMEN instructions, data, and referrals.

#### LIST OF BLOCK TYPES
- loop - A block that is intended to be executed repeatedly indefinitely.

- while - A block that is intended to be executed repeatedly while a certain condition is true. The condition is evaluated at the beginning of the block, and if it is true, the block is executed again.

- dowhile - A block that is intended to be executed repeatedly while a certain condition is true. The condition is evaluated at the end of the block, and if it is true, the block is executed again.

- repeat - A block that is intended to be executed a certain number of times. The number of iterations is specified at the beginning of the block.

- for - A block that is intended to be executed a certain number of times, with an index variable that is incremented or decremented each iteration. The number of iterations and the initial value of the index variable are specified at the beginning of the block.

- if - A block that is intended to be executed if a certain condition is true. The condition is evaluated at the beginning of the block, and if it is true, the block is executed.

- else - A block that is intended to be executed if the condition of the preceding if block is false. The else block must immediately follow an if block and will be executed if the condition of that if block is false.

- wram - A block that defines a referral or referral range in WRAM at the specified address, with optional control flow instructions and a block of code or data enclosed in curly braces.

- vram - A block that defines a referral or referral range in VRAM at the specified address, with optional control flow instructions and a block of code or data enclosed in curly braces.

- cram - A block that defines a referral or referral range in CRAM at the specified address, with optional control flow instructions and a block of code or data enclosed in curly braces.

- oam - A block that defines a referral or referral range in OAM at the specified address, with optional control flow instructions and a block of code or data enclosed in curly braces.

- no block (omit block type entirely) - A block that defines a referral or referral range at the specified address independent of any block (no control flow block or data block type). The block of code or data is enclosed in curly braces.

- print - A block that prints the contents of the block as tiles. The data must be structured as such: \[tile\]\[palette\], \[tile\]\[palette], ... where tile is a 32-bit unsigned integer representing the tile index in VRAM, and palette is a 32-bit unsigned integer representing the palette index in CRAM. The block of data is enclosed in curly braces.

- amen - A block that contains raw AMEN instructions and data. The block of code and data is enclosed in curly braces.

### OPERATORS
- \+ - Used for addition in arithmetic expressions.
- \- - Used for subtraction in arithmetic expressions.
- \* - Used for multiplication in arithmetic expressions.
- / - Used for division in arithmetic expressions.
- % - Used for modulus in arithmetic expressions. Can also be used to denote registers (e.g., %gp1, %cursorX, etc).
- = - Used for assignment.
- == - Used for equality comparison in conditions.
- != - Used for inequality comparison in conditions.
- \> - Used for greater than comparison in conditions.
- < - Used for less than comparison in conditions.
- \>= - Used for greater than or equal to comparison in conditions.
- <= - Used for less than or equal to comparison in conditions.
- && - Used for logical AND in conditions.
- || - Used for logical OR in conditions.
- ! - Used for logical NOT in conditions.
- ^^ - Used for logical XOR in conditions.

### HASH PROTOCOL
- \#rdefine \[ref\] as \[iden\] - Defines a referral as an identifier that can be used in place of the referral in code. The identifier must be unique and cannot be the same as any other identifier or keyword.

- \#include \[file\] - Includes the contents of another file at the location of the include directive. The file must be a valid OMEN source file and can contain any valid OMEN code, including referrals, blocks, and instructions.

- \#adefine \[addr\] as \[iden\] - Defines an address as an identifier that can be used in place of the address in code. The identifier must be unique and cannot be the same as any other identifier or keyword.

- \#cdef \[code\] as \[alias\] - Defines a string of code as a one word alias that must be prefixed by the \$ sign.

## MEMORY MODEL

The OMEN Machine uses a memory model comprised of four main types of memory and a set of registers. The four main types of memory are main memory, WRAM, VRAM, and OAM. 

Main memory is used for storing code and data that is not related to graphics or sound. 

WRAM is used for temporary storage of data and code during program execution. 

VRAM is used for storing data related to graphics and display, including tile data, palette data, and sprite data. 

OAM is used for storing data related to sprites and objects.

Each memory type functions on an address bus where each address references 10 bytes of data. This means that each address can store a 80-bit value, which can be interpreted as either code or data depending on the context in which it is used.

Registers are used for temporary storage of data and code during program execution. Each register can store a 32-bit value, which can be interpreted as either code or data depending on the context in which it is used.

The main registers are as follows:
- %cursorX - Used for storing the X coordinate for drawing operations.
- %cursorY - Used for storing the Y coordinate for drawing operations.
- %Interrupt - Used for storing the interrupt code when an interrupt occurs.
- %KeyboardPress - Used for storing the referral to be called when a keyboard press interrupt occurs.
- %KeyboardRelease - Used for storing the referral to be called when a keyboard release interrupt occurs
- %VBlank - Used for storing the referral to be called when a VBlank interrupt occurs.
- %HBlank - Used for storing the referral to be called when an HBlank interrupt occurs.
- %gp1 - General purpose register 1.
- %gp2 - General purpose register 2.
- %gp3 - General purpose register 3.
- %gp4 - General purpose register 4.
- %gp5 - General purpose register 5.
- %gp6 - General purpose register 6.
- %gp7 - General purpose register 7.
- %gp8 - General purpose register 8.
- %gp9 - General purpose register 9.
- %gp10 - General purpose register 10.
- %gp11 - General purpose register 11.
- %gp12 - General purpose register 12.
- %gp13 - General purpose register 13.
- %gp14 - General purpose register 14.
- %gp15 - General purpose register 15.
- %gp16 - General purpose register 16.
- %gp17 - General purpose register 17.
- %gp18 - General purpose register 18.
- %gp19 - General purpose register 19.
- %gp20 - General purpose register 20.
- %gp21 - General purpose register 21.
- %gp22 - General purpose register 22.
- %gp23 - General purpose register 23.
- %gp24 - General purpose register 24.
- %gp25 - General purpose register 25.
- %gp26 - General purpose register 26.
- %gp27 - General purpose register 27.
- %gp28 - General purpose register 28.
- %gp29 - General purpose register 29.
- %gp30 - General purpose register 30.
- %gp31 - General purpose register 31.

The integer values of each register are as follows:
- %cursorX = 0x00000000
- %cursorY = 0x00000001
- %Interrupt = 0x00000002
- %KeyboardPress = 0x00000003
- %KeyboardRelease = 0x00000004
- %VBlank = 0x00000005
- %HBlank = 0x00000006
- %gp1 = 0x00000007
- %gp2 = 0x00000008
- %gp3 = 0x00000009
- %gp4 = 0x0000000A
- %gp5 = 0x0000000B
- %gp6 = 0x0000000C
- %gp7 = 0x0000000D
- %gp8 = 0x0000000E
- %gp9 = 0x0000000F
- %gp10 = 0x00000010
- %gp11 = 0x00000011
- %gp12 = 0x00000012
- %gp13 = 0x00000013
- %gp14 = 0x00000014
- %gp15 = 0x00000015
- %gp16 = 0x00000016
- %gp17 = 0x00000017
- %gp18 = 0x00000018
- %gp19 = 0x00000019
- %gp20 = 0x0000001A
- %gp21 = 0x0000001B
- %gp22 = 0x0000001C
- %gp23 = 0x0000001D
- %gp24 = 0x0000001E
- %gp25 = 0x0000001F
- %gp26 = 0x00000020
- %gp27 = 0x00000021
- %gp28 = 0x00000022
- %gp29 = 0x00000023
- %gp30 = 0x00000024
- %gp31 = 0x00000025

## INTERRUPTS

The interrupts of the OMEN system work using different signals and an %Interrupt register.

When an interrupt occurs, the processor stops, logs an integer value into the %Interrupt register, does any other required tasks, and checks a corresponding register based on the %Interrupt register. It will then CALL that register. 

{
    example: 
        KeyboardPressInterrupt 
        => 0x//////// in %Interrupt 
        => check %Interrupt 
        => go to %KeyboardPress 
        => CALL %KeyboardPress 
}

The list of interrupts are as follows:

- KeyboardPressInterrupt       %KeyboardPress
- KeyboardReleaseInterrupt     %KeyboardRelease
- VBlankInterrupt              %VBlank
- HBlankInterrupt              %HBlank

Interrupt CALLs must be loaded beforehand via PUSH/LOAD and STRR.

## DISPLAY

The screen is made up of 40x32 tiles, and each tile is an 8x8 pixel image. This means that the screen can display a total of 320x256 pixels. The color of each pixel is determined by a palette, which is a set of colors that can be used to color the tiles on the screen.

Tiles are 32 byte structs in VRAM that represent an 8x8 pixel image. Every 8 bytes represents a row of pixels, and each pixel is represented by 4 bits that refer to a color in the palette. This means that each tile can represent an image with up to 16 colors. The palette data is stored in CRAM, and each palette is a set of 16 colors, where each color is represented as a 16 bit unsigned integer in the format 0bRRRRRGGGGGGBBBBB, where R is the red component, G is the green component, and B is the blue component of the color.

The display system works on a set of interrupts (VBlank and HBlank), a 40x32 tile screen, OAM, and VRAM.

The DRAW instruction is used to draw tiles from VRAM onto the screen at the %cursorX and %cursorY coordinates. The data for the DRAW instruction is specified as a referral or referral range that points to tile and palette data in VRAM. The tile data is used to determine which tiles to draw, and the palette data is used to determine the colors of those tiles.

Tile data is structured as follows: \[tile\]\[palette\]\[layer\], \[tile\]\[palette\]\[layer\], ... where tile is a 32-bit unsigned integer representing the tile index in VRAM, and palette is a 32-bit unsigned integer representing the palette index in CRAM. 

Tile IDs are based on an offset of 00000000 - AFFFFFFF in VRAM, and palette IDs are based on an offset of B0000000 - FFFFFFFF in CRAM. This means that tile ID 0 is at 00000000, 1 is at 00000020, 2 is at 00000040, etc. and palette ID 0 is at B0000000, 1 is at B0000020, 2 is at B0000040, etc.

The tile section refers to the id of the tile in VRAM, and the palette section refers to the id of the palette in CRAM. The layer section is used to determine the order in which tiles are drawn.

Layer numbers are as shown:
- 0000 => background layer 0 (bottom layer)
- 0100 => middleground
- 0200 => foreground (top layer)
- 0300 => object layer (sprites, text, etc). The bottom 16 bits determine the priority of the object on the object layer (e.g., 0300 is lower than 0301, so 0301 is drawn over 0300)

To measure this, a seperate place in memory called OAM is used. An OAM struct looks as such: \[x:2\]\[y:2\]\[TID:4\]\[PID:4\]\[FLIP:1\]\[PRIORITY:1\]\[PADDING:2\]
x and y refer to the coordinates of the sprite,
TID refers to the tile id in VRAM,
PID refers to the palette id in CRAM,
FLIP refers to whether the sprite is flipped horizontally or vertically (00 => no flip, 01 => horizontal flip, 02 => vertical flip, 03 => both flips),
PRIORITY refers to the priority of the sprite on the object layer (00 => lowest priority, FF => highest priority)
PADDING is just unused bits to fill out the struct to 16 bytes.

## SOUND

The sound system works using the PLAY and PLRW instructions and sound structs in main memory. When a PLAY or PLRW instruction is executed, the processor reads the sound data from the specified referral and plays it through a sound CPU. The sound data is structured as follows: \[note: 1\]\[duration: 1\].

Notes are represented as 8 bit unsigned integers, where the ones digit represents the note in the chromatic scale (0 => C, 1 => C#, 2 => D, 3 => D#, 4 => E, 5 => F, 6 => F#, 7 => G, 8 => G#, 9 => A, 10 => A#, 11 => B) and the tens digit represents the octave (0-7). For example, middle C would be represented as 100, C#4 would be represented as 101, D4 would be represented as 102, etc.

PLAY and PLRW are not blocking instructions, meaning that the processor will continue executing the next instructions while the sound is playing. This allows for multiple sounds to be played simultaneously and for sounds to be played while other operations are being performed.

## KEYBOARD

The keypressing system works using the stack and two types of interrupts: KeyboardPressInterrupt and KeyboardReleaseInterrupt.

When a key is pressed, the CPU stops and immediately pushes the integer value of the key pressed onto the stack. It then logs the integer value of the %KeyboardPress register (0x00000003) into the %Interrupt register. It then checks the %Interrupt register, sees that it is a KeyboardPressInterrupt, and jumps to the referral stored in the %KeyboardPress register, calling it as a function. The same process occurs for key releases, but with the %KeyboardRelease register instead.

If two keys are pressed at the same time, the CPU will handle both interrupts in the order they were received. This means that if key A and key B are pressed simultaneously, the CPU will first handle the interrupt for key A, then handle the interrupt for key B. The same applies for key releases.
