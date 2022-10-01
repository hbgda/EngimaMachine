use std::collections::{HashSet, HashMap};
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
    _key_map: HashMap<char, char>,
    _step_chars: HashSet<char>,
    _ring_setting: u8,      // 0-25 | A-Z
    _current_position: u8,  // 0-25 | A-Z
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
        let offset_char = add_char(source, self._current_position + self._ring_setting);

        match self._key_map.get(&offset_char) {
            Some(mapped) => subtract_char(*mapped, self._current_position + self._ring_setting),
            None => source
        }
    }

    pub fn get_cyphered_char_reflect(&self, source: char) -> char {
        let offset_char = add_char(source, self._current_position + self._ring_setting);

        match self._key_map.iter().find(|&pair| pair.1 == &offset_char) {
            Some(mapped) => subtract_char(*mapped.0, self._current_position + self._ring_setting),
            None => {
                eprintln!("Couldn't find char {offset_char}");
                source
            }
        }
    }
}

impl Rotor {
    pub fn should_step_next(&self) -> bool {
        self._step_chars.contains(
            &u8_to_char(self._current_position)
        )
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

        let mut _rotor = Rotor {
            _id: id,
            _key_map: HashMap::new(),
            _step_chars: step_chars,
            _ring_setting: (ring_setting as u8 - b'A') % 26,
            _current_position: (position as u8 - b'A') % 26
        };

        for (index, mapped) in key_map.chars().enumerate() {
            let _origin_char: char = u8_to_char(index as u8);
            _rotor._key_map.insert(_origin_char, mapped);
            println!("Origin: {_origin_char} Mapped: {mapped}");
        }

        _rotor
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

