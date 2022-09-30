pub fn char_to_u8(source: char) -> u8{
    source as u8 - b'A'
}

pub fn u8_to_char(source: u8) -> char {
    (b'A' + (source % 26)) as char
}