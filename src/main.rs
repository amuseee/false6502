// simple 6502 emulator written in rust
// https://web.archive.org/web/20210429110213/http://www.obelisk.me.uk/6502/

use std::fmt::Debug;
use bitfield::bitfield;

struct cpu {
    pc: u16, // program counter
    sp: u8, // stack pointer
    a: u8, // registers
    x: u8,
    y: u8,

    // processor status flags - bitfields
   bitfield!{
        pub struct Statusflags(u8);

   } 
    /*
        c++ implen of bitfield status flags
        unsigned char C : 1;
        unsigned char Z : 1;
        unsigned char I : 1;
        unsigned char D : 1;
        unsigned char B : 1;
        unsighed char V : 1;
        unsigned char N : 1;
    */
}

impl cpu {

}

fn main() {

}