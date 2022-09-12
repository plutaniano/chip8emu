const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const DISPLAY_HEIGHT: usize = 64;
const DISPLAY_WIDTH: usize = 32;

struct CPU {
    ram: [u8; RAM_SIZE],
    v: [u8; 16],
    i: u16,
    pc: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    dt: u16,
    st: u16,
}


impl CPU {
    pub fn new() -> Self {
        Processor {
            ram: [0; RAM_SIZE],
            v: [u8; 0],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; STACK_SIZE],
            dt: 0,
            st: 0,
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
            (0xA, _, _, _) => self.run_annn(b + c + d),
            (0xB, _, _, _) => self.run_bnnn(b + c + d),
            (0xC, _, _, _) => self.run_cxkk(b, c + d),
            (0xD, _, _, _) => self.run_dxyn(b, c, d),
            (0xE, _, 0x9, 0xE) => self.run_ex9e(b),
            (0xE, _, 0xA, 0x1) => self.run_exa1(b),
            (0xF, _, 0x0, 0x7) => self.run_fx07(b),
            (0xF, _, 0x0, 0xA) => self.run_fx0a(b),
            (0xF, _, 0x1, 0x5) => self.run_fx15(b),
            (0xF, _, 0x1, 0x8) => self.run_fx18(b),
            (0xF, _, 0x1, 0xE) => self.run_fx1e(b),
            (0xF, _, 0x2, 0x9) => self.run_fx29(b),
            (0xF, _, 0x3, 0x3) => self.run_fx33(b),
            (0xF, _, 0x5, 0x5) => self.run_fx55(b),
            (0xF, _, 0x6, 0x5) => self.run_fx65(b),
            _ => 
        }
    }

    // 0nnn - SYS addr
    // Jump to a machine code routine at nnn.
    fn run_0nnn(&self, addr: u8) -> {}
    
    // 00e0 - CLS
    // Clear the display.
    fn run_00e0(&mut self) -> () {
        self.vram.iter_mut().for_each(|m| *m = 0);
    }

    // 1nnn - JP addr
    // Jump to location nnn.
    fn run_1nnn(&mut self, addr: u8) -> () {
        self.pc = addr;
    }

    // 2nnn - CALL addr
    // Call subroutine at nnn.
    fn run_2nnn(&mut self, addr: u8) -> () {
        self.sp += 1;
        self.stack.push(self.pc);
        self.pc = addr;
    }

    // 3xkk - SE Vx, byte
    // Skip next instruction if Vx = kk.
    fn run_3xkk(&mut self, x: u8, byte: u8) -> () {
        if self.v[x] == byte {
            self.pc += 1;
        }
    }

    // 4xkk - SNE Vx, byte
    // Skip next instruction if Vx != kk.
    fn run_4xkk(&mut self, x: u8, byte: u8) -> () {
        if self.v[x] != byte {
            self.pc += 1;
        }
    }

    // 5xy0 - SE Vx, Vy
    // Skip next instruction if Vx == Vy
    fn run_5xy0(&mut self, x: u8, y: u8) -> () {
        if self.v[x] == self.v[y] {
            self.pc += 1;
        }
    }

    // 6xkk - LD Vx, byte
    // Set Vx = Vx + kk.
    fn run_6xkk(&mut self, x: u8, byte: u8) -> () {
        self.v[x] = byte;
    }

    // 7xkk - ADD Vx, byte
    // Set Vx = Vx + kk.
    fn run_7xkk(&mut self, x: u8, byte: u8) -> () {
        self.v[x] += byte;
    }

    // 8xy0 - LD Vx, Vy
    // Set Vx = Vy
    fn run_8xy0(&mut self, x: u8, y: u8) -> () {
        self.v[x] = self.v[y];
    }

    // 8xy2 - AND Vx, Vy
    // Set Vx = Vx AND Vy
    fn run_8xy2(&mut self, x: u8, y: u8) -> () {
        self.v[x] = self.v[x] & self.v[y];
    }

    // 8xy3 - XOR Vx, Vy
    // Set Vx = Vx XOR Vy
    fn run_8xy3(&mut self, x: u8, y: u8) -> () {
        self.v[x] = self.v[x] ^ self.v[y];
    }

    // 8xy4 - ADD Vx, Vy
    // Set Vx = Vx + Vy
    fn run_8xy4(&mut self, x: u8, y: u8) -> () {
        let result: u16 = x + y;
        self.v[x] = result as u8;
        self.v[0xf] = if result > 0xff { 1 } else { 0 };
    }

    // 8xy5 - SUB Vx, Vy
    // Set Vx = Vx - Vy, set VF = NOT borrow
    fn run_8xy5(&mut self, x: u8, y: u8) -> () {
        self.v[0xf] = if self.v[x] > self.v[y] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
    }

    // 
}
