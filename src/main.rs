// simple 6502 emulator written in rust
// https://web.archive.org/web/20210429110213/http://www.obelisk.me.uk/6502/

use std::arch::x86_64::_CMP_FALSE_OQ;

struct cpu {
    PC: u16, // program counter
    SP: u16, // stack pointer
    A: u8, // registers
    X: u8,
    Y: u8,
    C: bool, // 1 bit processor flags, can use bool
    Z: bool,
    I: bool,
    D: bool,
    B: bool,
    V: bool,
    N: bool,
}

impl cpu {
    fn new() -> Self {
        cpu {
            PC: 0,
            SP: 0,
            A: 0,
            X: 0,
            Y: 0,
            C: false,
            Z: false,
            I: false,
            D: false,
            B: false,
            V: false,
            N: false,
        }
    }

    fn reset(&mut self)
    {
        // https://www.c64-wiki.com/wiki/Reset_(Process)
        self.PC = 0xFFFC;
        self.SP = 0x0100;
        self.A = 0;
        self.X = 0;
        self.Y = 0;
        self.C = false;
        self.Z = false;
        self.I = false;
        self.D = false;
        self.B = false;
        self.V = false;
        self.N = false;
    }
}

fn main() {
    let mut emu = cpu::new();
    emu.reset();
}