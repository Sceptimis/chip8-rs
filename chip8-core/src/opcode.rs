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
        (0x0, 0x0, 0xE, 0x0) => Some(Opcode::Clear),
        (0x0, 0x0, 0xE, 0xE) => Some(Opcode::Return),
        (0x1, _, _, _) => Some(Opcode::Jump((opcode & 0x0FFF) as u8)),
        (0x2, _, _, _) => Some(Opcode::Call((opcode & 0x0FFF) as u8)),
        (0x3, x, _, _) => Some(Opcode::SkipIfVxEqualsNn(x, (opcode & 0x00FF) as u8)),
        (0x4, x, _, _) => Some(Opcode::SkipIfVxNotEqualsNn(x, (opcode & 0x00FF) as u8)),
        (0x5, x, y, 0x0) => Some(Opcode::SkipIfVxEqualsVy(x, y)),
        (0x6, x, _, _) => Some(Opcode::StoreNnInVx(x, (opcode & 0x00FF) as u8)),
        (0x7, x, _, _) => Some(Opcode::AddNnToVx(x, (opcode & 0x00FF) as u8)),
        (0x8, x, y, 0x0) => Some(Opcode::SetVyInVx(x, y)),
        (0x8, x, y, 0x1) => Some(Opcode::Or(x, y)),
        (0x8, x, y, 0x2) => Some(Opcode::And(x, y)),
        (0x8, x, y, 0x3) => Some(Opcode::Xor(x, y)),
        (0x8, x, y, 0x4) => Some(Opcode::Add(x, y)),
        (0x8, x, y, 0x5) => Some(Opcode::Sub(x, y)),
        (0x8, x, y, 0x6) => Some(Opcode::ShiftRight(x, y)),
        (0x8, x, y, 0x7) => Some(Opcode::ReverseSub(x, y)),
        (0x8, x, y, 0xE) => Some(Opcode::ShiftLeft(x, y)),
        (0x9, x, y, 0x0) => Some(Opcode::SkipIfVxNotEqualsVy(x, y)),
        (0xA, _, _, _) => Some(Opcode::StoreNnnInI((opcode & 0x0FFF) as u8)),
        (0xB, _, _, _) => Some(Opcode::JumpAddV0((opcode & 0x0FFF) as u8)),
        (0xC, x, _, _) => Some(Opcode::SetVxRand(x, (opcode & 0x00FF) as u8)),
        (0xD, x, y, n) => Some(Opcode::DrawSprite(x, y, n)),
        (0xE, x, 0x9, 0xE) => Some(Opcode::SkipIfKeyDown(x)),
        (0xE, x, 0xA, 0x1) => Some(Opcode::SkipIfKeyNotDown(x)),
        (0xF, x, 0x0, 0x7) => Some(Opcode::StoreDelayInVx(x)),
        (0xF, x, 0x0, 0xA) => Some(Opcode::WaitKeyDownStore(x)),
        (0xF, x, 0x1, 0x5) => Some(Opcode::SetDelayToVx(x)),
        (0xF, x, 0x1, 0x8) => Some(Opcode::SetSoundToVx(x)),
        (0xF, x, 0x1, 0xE) => Some(Opcode::AddVxToI(x)),
        (0xF, x, 0x2, 0x9) => Some(Opcode::SetSpriteIFromVx(x)),
        (0xF, x, 0x3, 0x3) => Some(Opcode::StoreBCD(x)),
        (0xF, x, 0x3, 0x3) => Some(Opcode::CopyRegisters(x)),
        (0xF, x, 0x3, 0x3) => Some(Opcode::FillRegisters(x)),
        _ => {
            println!("Opcode {} not implemented", opcode);
            None
        }
    }
}
