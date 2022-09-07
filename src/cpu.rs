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
}
