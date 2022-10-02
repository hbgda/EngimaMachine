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
    
    // Increment current position
    pub fn increment_position(&mut self) {
        self._current_position = (self._current_position + 1) % 26
    }

    // Cypher the given char, accounting for ring setting and offset.
    // In an actual enigma, the ring setting refers to the offset of the internal wiring of each Rotor.
    pub fn cypher_char(&self, source: char) -> char {

        // Offset source char by adding the current position and the ring setting, add_char will account for any overflow.
        let offset_char = add_char(source, self._current_position + self._ring_setting);

        // If the resulting offset_char exists in the enigmas key map return its associated value, this would be whatever the letter cyphers to normally
        // If the character does not have an associated value just return itself, this shouldn't happen.
        match self._key_map.get(&offset_char) {
            Some(mapped) => subtract_char(*mapped, self._current_position + self._ring_setting),
            None => source
        }
    }

    // Similar to cypher_char, this instead cyphers in the opposite direction
    pub fn reflect_char(&self, source: char) -> char {
        let offset_char = add_char(source, self._current_position + self._ring_setting);

        // Find the first value equal to offset_char, the associated key will then be what is needed.
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

    // Return true if the Rotor is at a point where a subsequent Rotor should increment.
    // In an actual enigma machine there would be a notch that triggers the ratchet teeth of the left Rotor.
    pub fn should_step_next(&self) -> bool {
        self._step_chars.contains(
            &u8_to_char(self._current_position)
        )
    }

    pub fn get_position(&self) -> char {
        u8_to_char(self._current_position)
    }
}

impl Rotor {
    pub fn new(id: String, key_map: String, step_chars: HashSet<char>, ring_setting: char, position: char) -> Self {

        // Check if ring_setting and position are valid alphabetical characters
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
            _ring_setting: char_to_u8(ring_setting),
            _current_position: char_to_u8(position)
        };

        // Iterate over characters in the key_map String and create a HashMap entry from its index and value
        // index is converted to an alphabetical character from A-Z depending on its value, e.g. 0 = A
        // To be clear a String is being used here for no particular reason other than it's what I used in the inital stages of mapping the program and I cba to change it
        for (index, mapped) in key_map.chars().enumerate() {
            let _origin_char: char = u8_to_char(index as u8);
            _rotor._key_map.insert(_origin_char, mapped);
        }

        _rotor
    }
}

impl Rotor {

    // Pre-configured Rotor versions
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

