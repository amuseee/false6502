/*
Like its precursor, the 6800, the 6502 has very few registers. The 6502's registers include one 8-bit accumulator register (A), two 8-bit index registers (X and Y), 
7 processor status flag bits (P; from bit 7 to bit 0 these are the negative (N), overflow (V), reserved, break (B), decimal (D), interrupt disable (I), zero (Z) and carry (C) flag), 
an 8-bit stack pointer (S), and a 16-bit program counter (PC).

instructions: https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA

*/

pub struct CPU {
    pub a: u8,
    pub x: u8,
    // pub y: u8,
    pub status: u8, // so there are a total of 7 status flags. 0bNVBDICZ0 (last bit is always unused i believe)
    // pub sp: u8,
    pub pc: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            x: 0,
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
                        self.status = self.status | 0b00000010;
                    } else {
                        self.status = self.status & 0b11111101; // unsets the "Zero" flag
                    }

                    // negaive flag, set if bit 7 of a is set/is a 1
                    if self.a & 0b10000000 == 1 {
                        self.status = self.status | 0b10000000;
                    } else {
                        // unset
                        self.status = self.status & 0b01111111;
                    }
                }
                0x00 => {
                    return;
                    // break doesnt need flags set since the program just quits
                }
                0xAA => {
                    self.x = self.a;

                    if self.x == 0 {
                        self.status = self.status | 0b10000000;
                    } else {
                        self.status = self.status & 0b11111101;
                    }

                    // neg flag
                    if self.x & 0b10000000 == 1 {
                        self.status = self.status | 0b10000000;
                    } else {
                        self.status = self.status & 0b01111111;
                    }
                }
                _ => todo!()
            }
        }
    }
}

#[cfg(test)]
mod test {
   use super::*;
 
   #[test]
   fn test_0xa9_lda_immediate_load_data() {
       let mut cpu = CPU::new();
       cpu.interpret(vec![0xa9, 0x05, 0x00]);
       assert_eq!(cpu.a, 0x05);
       assert!(cpu.status & 0b0000_0010 == 0b00);
       assert!(cpu.status & 0b1000_0000 == 0);
   }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.a = 10;
        cpu.interpret(vec![0xaa, 0x00]);
  
        assert_eq!(cpu.x, 10)
    }
}