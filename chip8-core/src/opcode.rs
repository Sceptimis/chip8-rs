/*
 * Information regarding these opcodes can be found at:
 * https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#instructions
 * https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set
 *
 * Instruction 0NNN is not implemented on purpose.
 */
pub enum Opcode {
    // 00E0 Clears the screen.
    Clear,
    // 00EE Returns from a subroutine.
    Return,
    // 1NNN Jumps to NNN.
    Jump(u8),
    // 2NNN Executes subroutine starting at NNN.
    Call(u8),
    // 3XNN Skips if VX equals NN.
    SkipIfVxEqualsNn(u8, u8),
    // 4XNN Skips if VX is not equal to NN.
    SkipIfVxNotEqualsNn(u8, u8),
    // 5XY0 Skips if VX is equal to VY.
    SkipIfVxEqualsVy(u8, u8),
    // 6XNN Stores NN in VX.
    StoreNnInVx(u8, u8),
    // 7XNN Adds NN to VX.
    AddNnToVx(u8, u8),
    // 8XY0 Stores VY in VX.
    SetVyInVx(u8, u8),
    // 8XY1 Sets VX to VX OR VY.
    Or(u8, u8),
    // 8XY2 Sets VX to VX AND VY.
    And(u8, u8),
    // 8XY3 Sets VX to VX XOR VY.
    Xor(u8, u8),
    // 8XY4 Adds VY to VX, Sets VF to 01 if carry, otherwise 00.
    Add(u8, u8),
    // 8XY5 Subtracts VY from VX, Sets VF to 00 if borrow, otherwise 01.
    Sub(u8, u8),
    // 8XY6 Stores VY bitshifted right by one in VX. Sets VF to least significant bit before shift.
    ShiftRight(u8, u8),
    // 8XY7 Sets VX to VY minus VX. Sets VF to 00 if borrow, otherwise 01.
    ReverseSub(u8, u8),
    // 8XYE Stores VY bitshifted left by one in VX. Sets VF to most significant bit before shift.
    ShiftLeft(u8, u8),
    // 9XY0 Skips if VX is not equal to VY.
    SkipIfVxNotEqualsVy(u8, u8),
    // ANNN Store NNN in I.
    StoreNnnInI(u8),
    // BNNN Same as Jump (1NNN) plus V0.
    JumpAddV0(u8),
    // CXNN Set VX to (rand & NN).
    SetVxRand(u8, u8),
    // DXYN Draw sprite at (VX, VY) with N height of data in I. Sets VF to 01 if any pixels get
    // unset, otherwise 00.
    DrawSprite(u8, u8, u8),
    // EX9E Skip if key corresponding to VX is pressed.
    SkipIfKeyDown(u8),
    // EXA1 Skip if key corresponding to VX is not pressed.
    SkipIfKeyNotDown(u8),
    // FX07 Store delay timer's value in VX.
    StoreDelayInVx(u8),
    // FX0A Wait for a keypress, then store result in VX.
    WaitKeyDownStore(u8),
    // FX15 Set delay timer to value in VX.
    SetDelayToVx(u8),
    // FX18 Set soun timer to value in VX.
    SetSoundToVx(u8),
    // FX1E Add VX to I.
    AddVxToI(u8, u8),
    // FX29 Set I to adress of sprite data corresponding to VX.
    SetSpriteIFromVx(u8),
    // FX33 Stores the binary-coded decimal equiavlent of VX at I, I+1, and I+2.
    StoreBCD(u8),
    // FX55 Stores V0 to VX in ram starting at I. I is set to I+X+1 afterwards.
    CopyRegisters(u8),
    // FX65 Fill V0 to VX with values starting at I. I is set to I+X+1 afterwards.
    FillRegisters(u8),
}

pub fn decode_instruction(opcode: u16) -> Option<Opcode> {
    // It's easier to split the nibble using a tuple.
    let nibble = (
        (opcode & 0xF000 >> 12) as u8,
        (opcode & 0x0F00 >> 8) as u8,
        (opcode & 0x00F0 >> 4) as u8,
        (opcode & 0x000F) as u8,
    );

    // Decode opcode.
    match nibble {
        _ => None,
    }
}
