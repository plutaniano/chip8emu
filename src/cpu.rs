const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const DISPLAY_HEIGHT: usize = 64;
const DISPLAY_WIDTH: usize = 32;

struct CPU {
    ram: [u8; RAM_SIZE],
    V: [u8; 16],
    I: u16,
    PC: u16,
    SP: u16,
    stack: [u16; STACK_SIZE],
    DT: u16,
    ST: u16,
}

impl CPU {
    pub fn new() -> Self {
        Processor {
            ram: [0; RAM_SIZE],
            V: [u8; 0],
            I: 0,
            PC: 0x200,
            SP: 0,
            stack: [0; STACK_SIZE],
            DT: 0,
            ST: 0,
        }
    }

    pub fn run_instruction(&mut self, nibbles: u16) {
        let a = nibbles & 0xF000 >> 12 as u8;
        let b = nibbles & 0x0F00 >> 08 as u8;
        let c = nibbles & 0x00F0 >> 04 as u8;
        let d = nibbles & 0x000F >> 00 as u8;

        match (a, b, c, d) {
            (0x0, 0x0, 0xE, 0x0) => self.run_00e0(),
            (0x0, 0x0, 0xE, 0xE) => self.run_00ee(),
            (0x0, _, _, _) => self.run_0nnn(a + b + c),
            (0x1, _, _, _) => self.run_1nnn(a + b + c),
            (0x2, _, _, _) => self.run_2nnn(a + b + c),
            (0x3, _, _, _) => self.run_3xkk(b, c + d),
            (0x4, _, _, _) => self.run_4xkk(b, c + d),
            (0x5, _, _, 0x0) => self.run_5xk0(b, c),
            (0x6, _, _, _) => self.run_6xkk(b, c + d),
            (0x7, _, _, _) => self.run_7xkk(b, c + d),
            (0x8, _, _, 0x0) => self.run_8xy0(b, c),
            (0x8, _, _, 0x1) => self.run_8xy1(b, c),
            (0x8, _, _, 0x2) => self.run_8xy2(b, c),
            (0x8, _, _, 0x3) => self.run_8xy3(b, c),
            (0x8, _, _, 0x4) => self.run_8xy4(b, c),
            (0x8, _, _, 0x5) => self.run_8xy5(b, c),
            (0x8, _, _, 0x6) => self.run_8xy6(b, c),
            (0x8, _, _, 0x7) => self.run_8xy7(b, c),
            (0x8, _, _, 0xE) => self.run_8xyE(b, c),
            (0x9, _, _, 0x0) => self.run_9xy0(b, c),
            (0xA, _, _, _) => self.run_Annn(b + c + d),
            (0xB, _, _, _) => self.run_Bnnn(b + c + d),
            (0xC, _, _, _) => self.run_Cnnn(b, c + d),
            (0xD, _, _, _) => self.run_Dnnn(b, c, d),
            (0xB, _, _, _) => self.run_Annn(b + c + d),
        }
    }
}
