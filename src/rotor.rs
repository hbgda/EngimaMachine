use std::collections::HashSet;
use crate::utils::*;

pub enum RotorVersion {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    Gamma,
    Beta
}

#[derive(Debug)]
pub struct Rotor {
    _id: String,
    _key_map: String,
    _step_chars: HashSet<char>,
    _ring_setting: u8,
    _current_position: u8,
}

impl Rotor {
    
    pub fn cypher_char_and_increment(&mut self, source: char) -> char {
        self.increment_position();
        self.get_cyphered_char(source)
    }

    pub fn increment_position(&mut self) {
        self._current_position = (self._current_position + 1) % 26
    }

    pub fn get_cyphered_char(&self, source: char) -> char {
        // println!("\n| Source: {source}");
        
        let char_u8: u8 = char_to_u8(source);
        let index: usize = ((char_u8 + self._current_position) % 26) as usize;
        // println!("| Char: {char_u8} Index: {index}");

        let cyphered: char = self._key_map.as_bytes()[index] as char;
        // println!("| Cyphered: {cyphered}\n");
        cyphered
    }
}

impl Rotor {
    pub fn should_step(&self, position: char) -> bool {
        self._step_chars.contains(&position)
    }

    pub fn get_position(&self) -> char {
        u8_to_char(self._current_position)
    }

    pub fn set_ring_setting(&mut self, setting: u8) {
        self._ring_setting = setting
    }
}

impl Rotor {
    pub fn new(id: String, key_map: String, step_chars: HashSet<char>, ring_setting: char, position: char) -> Self {

        if !ring_setting.is_alphabetic() {
            eprintln!("ring_setting invalid, {ring_setting} not alphabetic");
            panic!();
        }
        if !position.is_alphabetic() {
            eprintln!("position invalid, {position} not alphabetic");
            panic!();
        }

        Rotor {
            _id: id,
            _key_map: key_map,
            _step_chars: step_chars,
            _ring_setting: (ring_setting as u8 - b'A') % 26,
            _current_position: (position as u8 - b'A') % 26
        }
    }
}

impl Rotor {
    pub fn from_version(version: RotorVersion, ring_setting: char, position: char) -> Self {
        match version {
            RotorVersion::I => {
                Rotor::new(
                    String::from("I"),      String::from("EKMFLGDQVZNTOWYHXUSPAIBRCJ"),     HashSet::from(['R']),       ring_setting, position
                )
            },
            RotorVersion::II => {
                Rotor::new(
                    String::from("II"),     String::from("AJDKSIRUXBLHWTMCQGZNPYFVOE"),     HashSet::from(['F']),       ring_setting, position 
                )
            }
            RotorVersion::III => {
                Rotor::new(
                    String::from("III"),    String::from("BDFHJLCPRTXVZNYEIWGAKMUSQO"),     HashSet::from(['W']),       ring_setting, position 
                )
            },
            RotorVersion::IV => {
                Rotor::new(
                    String::from("IV"),     String::from("ESOVPZJAYQUIRHXLNFTGKDCMWB"),     HashSet::from(['K']),       ring_setting, position 
                )
            },
            RotorVersion::V => {
                Rotor::new(
                    String::from("V"),      String::from("VZBRGITYUPSDNHLXAWMJQOFECK"),     HashSet::from(['A']),       ring_setting, position
                )
            },
            RotorVersion::VI => {
                Rotor::new(
                    String::from("VI"),     String::from("JPGVOUMFYQBENHZRDKASXLICTW"),     HashSet::from(['A', 'N']),  ring_setting, position
                )
            },
            RotorVersion::VII => {
                Rotor::new(
                    String::from("VII"),    String::from("NZJHGRCXMYSWBOUFAIVLPEKQDT"),     HashSet::from(['A', 'N']),  ring_setting, position
                )
            },
            RotorVersion::VIII => {
                Rotor::new(
                    String::from("VIII"),   String::from("FKQHTLXOCBJSPDZRAMEWNIUYGV"),     HashSet::from(['A', 'N']),  ring_setting, position
                )
            },

            RotorVersion::Beta => todo!(),
            RotorVersion::Gamma => todo!(),
        }
    }
}

