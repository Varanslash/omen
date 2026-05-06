hello everyone :3

it took a while for this new upload because of school and stuff, but here we go

i am working on a library for the OMEN language. not a standard library, but closer to manufacturer provided code (like IBM’s fortran library back in fortran 1 and 2). it will be implemented from E0000000 - FEFFFFFF, and some code for interrupt handling that happens in FF000000.

I also developed two new systems:

The first is a keypressing system, where each key has a 1 byte numerical value (00 - FF) that corresponds to itself. when pressed, it will trigger an interrupt where the processor has to stop, save the key id onto the stack, and CALL FF000000 (should this be changed?). Interrupt handling code (that you write at that address) handles keypresses.

the second is a music system - i have bytes that correspond to each key on a piano. the least significant byte (0 - C) tells the note, and the most significant byte tells the octave. music is played by incrementing through note struct data manually and playing the note via a PLAY instruction that takes a referral and plays the referral address inside the first referral (example: PLAY 00001000, if 00001000 has the integer 80000000, the note at 80000000 is played). There’s also PLAY_RAW that plays the specified referral (PLAY_RAW 00001000 plays 00001000)

finally, i was able to get the video system designed. i don’t feel like summarizing so here’s an explanation i gave someone:

“ok so

tiles are drawn to the screen using the PRINT instruction and this cool thing called vram

vram is split into two parts: the tile space and the palette space

tiles take up vram addresses 00000000 to AFFFFFFF

palettes take B0000000 to FFFFFFFF

each tile is a 32 byte struct composed of 8 32bit integers

each of these integers correspond to 8 pixels, ranging from 0 - F in color

so 0x0FFFFFF0 would be 0, 15, 15, 15, 15, 15, 15, 0

each palette is a 32 bit struct composed on 16 16bit integers

each of these integers is a 16bit color

they go from 0 - F in order

you draw tiles by using the print instruction pointing to a referral that holds your tile and palette data

each tile and palette has an id based on offset from 00000000/B0000000, which is what you use

example: PRINT 0x10000000, where at 10000000 is 0x00000000 (tile) and 0x00000001 (palette)

you can do this with as many tiles as you need

when it comes to background, foreground, middleground, and sprites, PRINT has a flag for that

PRINTing using flag 00XX makes it background

01XX is middle

and 02XX is foreground

03XX is sprites, where XX tells you the priority of the sprite

higher number = higher priority, where sprites are drawn on top of each other

to measure this, we use a seperate memory zone called OAM

an OAM struct looks like this => [x:2][y:2][TID:4][PID:4][FLIP:1][PRIORITY:1][PADDING:2], where numbers represent bytes taken

x and y are the sprite's top left pixel offset from the top left pixel of the screen

tid is the tile id it is

and pid is the palette id

flipping is either 0x00, 0x01, 0x02, or 0x03

0 is normal, 1 is mirrored, 2 is vertical mirror, and 3 is both

priority goes from 00 to FF

padding is so we have a nice round number like 16 instead of a brutish number like 14”

with all of this in mind, there are three milestones left:

- make the spec

- make the provided library

- make a small game or usable program using some of these features

man we’ve come so far

goodbye everyone :3
