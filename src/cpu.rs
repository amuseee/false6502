/*
Like its precursor, the 6800, the 6502 has very few registers. The 6502's registers include one 8-bit accumulator register (A), two 8-bit index registers (X and Y), 
7 processor status flag bits (P; from bit 7 to bit 0 these are the negative (N), overflow (V), reserved, break (B), decimal (D), interrupt disable (I), zero (Z) and carry (C) flag), 
an 8-bit stack pointer (S), and a 16-bit program counter (PC).

instructions: https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA

*/

pub struct CPU {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub status: u8, // so there are a total of 7 status flags. 0bNVBDICZ0 (last bit is always unused i believe)
    pub sp: u8,
    pub pc: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            status: 0,
            pc: 0,
        }
    }
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.pc = 0;

        loop {
            let opcode = program[self.pc as usize];
            self.pc += 1;
            match opcode {
                0xA9 => {
                    // lda - https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA
                    let param = program[self.pc as usize];
                    self.pc += 1;
                    self.a = param;

                    // know your binary logic kids!
                    if self.a == 0 {
                        self.status = self.status || 0b00000010;
                    } else {
                        self.status = self.statuf && 0b11111101; // unsets the "Zero" flag
                    }

                    // negaive flag, set if bit 7 of a is set/is a 1
                    if self.a & 0b10000000 == 1 {
                        self.status = self.status || 0b10000000;
                    } else {
                        // unset
                        self.status = self.status & 0b01111111;
                    }
                }
                _ => todo!()
            }
        }
    }
}