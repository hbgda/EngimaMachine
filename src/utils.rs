pub fn char_to_u8(source: char) -> u8{
    source as u8 - b'A'
}

pub fn u8_to_char(source: u8) -> char {
    (b'A' + (source % 26)) as char
}

pub fn add_char(source: char, plus: u8) -> char {
    (b'A' + ((char_to_u8(source) + plus) % 26)) as char
}

pub fn subtract_char(source: char, minus: u8) -> char {
    (b'A' + ((char_to_u8(source) + (26 - minus)) % 26)) as char

}