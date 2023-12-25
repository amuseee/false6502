// simple 6502 emulator written in rust
// https://web.archive.org/web/20210429110213/http://www.obelisk.me.uk/6502/

struct cpu {
    pc: u8, // program counter
    sp: u8, // stack pointer
    a: u16, // registers
    x: u16,
    y: u16,
}

impl cpu {
    fn new() -> cpu {
        cpu {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
        }
    }
}

fn main() {

}