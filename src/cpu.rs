/*
Like its precursor, the 6800, the 6502 has very few registers. The 6502's registers include one 8-bit accumulator register (A), two 8-bit index registers (X and Y), 
7 processor status flag bits (P; from bit 7 to bit 0 these are the negative (N), overflow (V), reserved, break (B), decimal (D), interrupt disable (I), zero (Z) and carry (C) flag), 
an 8-bit stack pointer (S), and a 16-bit program counter (PC).

instructions: https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA

*/

pub struct CPU {
    pub ra: u8,
    pub rx: u8,
    // pub y: u8,
    pub status: u8, // so there are a total of 7 status flags. 0bNVBDICZ0 (last bit is always unused i believe)
    // pub sp: u8,
    pub pc: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            ra: 0,
            rx: 0,
            status: 0,
            pc: 0,
        }
    }

    fn update_flags(&mut self, result: u8) { // takes in a register value and updates the status flags based on that
        // sets/unsets zero flag
        if result == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }

        // sets/unsets negative flag
        if result & 0b1000_0000 == 1 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        } 
    }

    fn lda(&mut self, param: u8) {
        self.ra = param;
        self.update_flags(self.ra);
    }

    fn tax(&mut self) {
        self.rx = self.ra;
        self.update_flags(self.ra)
    }

    fn inx(&mut self) {
        self.rx = self.rx.wrapping_add(1);
        self.update_flags(self.rx);
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        loop {
            let opcode = program[self.pc as usize];
            self.pc += 1;
            match opcode {
                0xA9 => {
                    // lda - https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA
                    let param = program[self.pc as usize];
                    self.pc += 1;

                    self.lda(param);
                }

                0x00 => return,
                
                0xAA => self.tax(),
                
                0xe8 => self.inx(),
                
                _ => todo!()
            }
        }
    }
}