// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
use types::{Address, Vregister};




let mut RAM: [u8; 4096] = [0; 4096];
// 4KB of RAM
// The first 512 bytes are unused (0x0 to 0x1FF) 
// Most CHIP8 programs start at 0x200, but some begin at 0x600

// Registers
let mut V: [u8; 16] = [0; 16]; // V registers 0x0 through 0xf
let mut I: u16 = 0; // I register
let mut PC: u16 = 0; // Program counter
let mut SP: u8 = 0; // Stack pointer
let mut stack: [u16, 16] = [0; 16];
let mut DT: u16; // Delay timer register
let mut ST: u16; // Sound timer register

enum Instruction {
    SYS,                      // 0nnn - SYS addr: Jump to nnn 
    CLS,                      // 00e0 - CLS: Clears the display
    RET,                      // 00ee - RET: Returns from a subroutine
    JP(Address),              // 1nnn - JP addr: Jumps to nnn
    CALL(Address),            // 2nnn - CALL addr: Calls subroutine at nnn 
    SE(Vregister, u8),        // 3xkk - SE Vx, byte: skips next instruction if Vx == kk 
    SNE(Vregister, u8),       // 4xkk - SNE Vx, byte: skips next instruction if Vx != kk 
    SE(Vregister, Vregister), // 5xy0 - SE Vx, Vy: skips next instruction if Vx == Vy
    LD(Vregister, u8),        // 6xkk - LD Vx, Vy: set Vx = kk  
    ADD(Vregister, u8),       // 7xkk - ADD Vx, byte: Set Vx = Vx + kk
    LD(Vregister, Vregister), // 8xy0 - LD Vx, Vy: set Vx = Vy
    OR(Vregister, Vregister), // 8xy1 - OR Vx, Vy:
}

// Display
let mut display: [[bool; 64]; 32] = [[0; 64]; 32];

fn main() {
    println!("Hello, world!");
}