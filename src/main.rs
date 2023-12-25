// simple 6502 emulator written in rust
// https://web.archive.org/web/20210429110213/http://www.obelisk.me.uk/6502/

struct cpu {
    pc: u16, // program counter
    sp: u16, // stack pointer
    a: u8, // registers
    x: u8,
    y: u8,
    /*
        TODO: implement processor status flags - bitfields
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
    fn new() -> Self {
        cpu {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
        }
    }

    fn reset(&mut self)
    {
        // https://www.c64-wiki.com/wiki/Reset_(Process)
        self.pc = 0xFFFC;
        self.sp = 0x0100;
        self.a = 0;
        self.x = 0;
        self.y = 0;
    }
}

fn main() {
    let mut emu = cpu::new();
    emu.reset();
}