/*
Like its precursor, the 6800, the 6502 has very few registers. The 6502's registers include one 8-bit accumulator register (A), two 8-bit index registers (X and Y), 
7 processor status flag bits (P; from bit 7 to bit 0 these are the negative (N), overflow (V), reserved, break (B), decimal (D), interrupt disable (I), zero (Z) and carry (C) flag), 
an 8-bit stack pointer (S), and a 16-bit program counter (PC).

instructions: https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA

*/

pub struct CPU {
    pub ra: u8,
    pub rx: u8,
    pub ry: u8,
    // pub y: u8,
    pub status: u8, // so there are a total of 7 status flags. 0bNVBDICZ0 (last bit is always unused i believe)
    // pub sp: u8,
    pub pc: u16,
    memory: [u8; 0xFFFF]
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
   Immediate,
   ZeroPage,
   ZeroPage_X,
   ZeroPage_Y,
   Absolute,
   Absolute_X,
   Absolute_Y,
   Indirect_X,
   Indirect_Y,
   NoneAddressing,
}

impl CPU {

    pub fn new() -> Self {
        CPU {
            ra: 0,
            rx: 0,
            ry: 0,
            status: 0,
            pc: 0,
            memory: [0; 0xFFFF]
        }
    }

    fn get_op_addr(&mut self, mode: &AddressingMode) -> u16 {

        match mode {
            // taken from https://bugzmanov.github.io/nes_ebook/chapter_3_2.html - will re-implement when not feeling lazy
            AddressingMode::Immediate => self.pc,
 
            AddressingMode::ZeroPage  => self.read(self.pc) as u16,
           
            AddressingMode::Absolute => self.read_u16(self.pc),
         
            AddressingMode::ZeroPage_X => {
                let pos = self.read(self.pc);
                let addr = pos.wrapping_add(self.rx) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.read(self.pc);
                let addr = pos.wrapping_add(self.ry) as u16;
                addr
            }
 
            AddressingMode::Absolute_X => {
                let base = self.read_u16(self.pc);
                let addr = base.wrapping_add(self.rx as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.read_u16(self.pc);
                let addr = base.wrapping_add(self.ry as u16);
                addr
            }
 
            AddressingMode::Indirect_X => {
                let base = self.read(self.pc);
 
                let ptr: u8 = (base as u8).wrapping_add(self.rx);
                let lo = self.read(ptr as u16);
                let hi = self.read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.read(self.pc);
 
                let lo = self.read(base as u16);
                let hi = self.read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.ry as u16);
                deref
            }
          
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    // little endian stuff??
    fn read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.read(pos) as u16;
        let hi = self.read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }
 
    fn write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.write(pos, lo);
        self.write(pos + 1, hi);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000.. (0x8000 + program.len())].copy_from_slice(&program[..]); // ?????
        self.write_u16(0xFFFC, 0x8000);
    }

    pub fn load_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn reset(&mut self) {
        self.ra = 0;
        self.rx = 0;
        self.status = 0;
        self.pc = self.read_u16(0xFFFC);
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

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_addr(mode);
        let value = self.read(addr);
        
        self.ra = value;
        self.update_flags(self.ra);
        self.pc += 1;
    }

    fn tax(&mut self) {
        self.rx = self.ra;
        self.update_flags(self.ra)
    }

    fn inx(&mut self) {
        self.rx = self.rx.wrapping_add(1);
        self.update_flags(self.rx);
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read(self.pc);
            self.pc += 1;

            match opcode {
                // break
                0x00 => return,
                // lda modes
                0xA9 => self.lda(&AddressingMode::Immediate),
                0xA5 => self.lda(&AddressingMode::ZeroPage),
                0xB5 => self.lda(&AddressingMode::ZeroPage_X),
                0xAD => self.lda(&AddressingMode::Absolute),
                0xBD => self.lda(&AddressingMode::Absolute_X),
                0xB9 => self.lda(&AddressingMode::Absolute_Y),
                0xA1 => self.lda(&AddressingMode::Indirect_X),
                0xB1 => self.lda(&AddressingMode::Indirect_Y), 
                // unimplemented addressing
                0xAA => self.tax(),
                0xe8 => self.inx(),
                _ => todo!()
            }
        }
    }
}