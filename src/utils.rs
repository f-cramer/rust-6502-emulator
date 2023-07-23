pub fn combine(lsb: u8, msb: u8, offset: u8) -> u16 {
    let without_offset = ((msb as u16) << 8) + (lsb as u16);
    without_offset.wrapping_add(offset as u16)
}
