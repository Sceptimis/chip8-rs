// CHIP-8 uses 64x32 pixels, SUPER-CHIP uses 128x64.
// Colour for display is monochromatic.
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
const FONT_ADDR_START: usize = 0x000;
const FONT_ADDR_END: usize = 0x1FF;
const PROGRAM_ADDR_START: usize = 0x200;

pub struct CPU {
    ram: [u8; 4096],                             // 4KB of "RAM".
    vram: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT], // 64x32 pixels.
    pc: u16,                                     // Program Counter.
    i: u16,              // Index Register, use to point at the current instruction in memory.
    stack: [u16; 16],    // Used when calling subroutines, an returning from them.
    delay: u8,           // Delay Timer. Decrements @ 60Hz until 0.
    sound: u8,           // Sound Timer. Same as delay timer, gives off a beep when above 0.
    registers: [u8; 16], // General purpose registers V0-VF.
}

impl CPU {
    pub fn new() -> Self {
        let mut ram = [0; 4096];

        // Copy font into the first 512 bytes.
        assert!(FONT.len() <= (FONT_ADDR_END - FONT_ADDR_START));
        ram[0..FONT.len()].copy_from_slice(&FONT);

        CPU {
            ram,
            vram: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            pc: 0,
            i: 0,
            stack: [0; 16],
            delay: 0,
            sound: 0,
            registers: [0; 16],
        }
    }

    /*
     * Programs are loaded into memory starting at address 0x200.
     */
    pub fn load_program(&mut self, prog: &[u8]) {
        self.ram[PROGRAM_ADDR_START..PROGRAM_ADDR_START + prog.len()].copy_from_slice(&prog);
        self.pc = PROGRAM_ADDR_START as u16;
    }

    /*
     * Read instruction that `pc` is pointing to, then increment `pc` by two.
     */
    fn fetch_instruction(&mut self) -> u16 {
        assert!(self.pc <= 4094);

        // Bitshift left, then bitwise OR two successive u8 into a single u16.
        let pc = self.pc as usize;
        let opcode = (self.ram[pc] as u16) << 8 | (self.ram[pc + 1] as u16);
        self.pc += 2;

        opcode
    }

    fn execute_instruction() {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {}
