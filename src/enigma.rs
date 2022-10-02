use crate::plugboard::Plugboard;
use crate::rotor::{Rotor, RotorVersion};
use crate::reflector::{Reflector, ReflectorVersion};

pub struct Enigma {
    _rotors: Vec<Rotor>,    // Rotors stored in reverse for convenience, index 0 is right most rotor.
    _reflector: Reflector,
    _plugboard: Plugboard
}

impl Enigma {

    pub fn encrypt_string(&mut self, source: &String) -> String {
        source.chars()
            .map(|_char| self.encrypt_char(_char)).collect()
    }

    pub fn encrypt_char(&mut self, source: char) -> char {
        
        if !source.is_alphabetic() {
            return source;
        }

        // Convert to uppercase
        let mut cyphered_char: char = source.to_ascii_uppercase();

        // First route char through plugboard
        cyphered_char = self._plugboard.get_char(cyphered_char);

        // Increment rotors if necessary
        self.increment_rotors();

        // Iterate over each rotor and cypher the current state of cyphered_char,
        // An enigma machine will go from right to left, in this case the rotors are stored in the reverse order for convenience
        for rotor in self._rotors.iter() {
            cyphered_char = rotor.cypher_char(cyphered_char);   
        }

        // Route char through reflector
        cyphered_char = self._reflector.get_char(cyphered_char);

        // Whatever comes from the reflector is then sent back through the rotors and cyphered in reverse
        for rotor in self._rotors.iter().rev() {
            cyphered_char = rotor.reflect_char(cyphered_char);
        }

        // Finally, route the cyphered_char through the plugboard again to get the end encrypted char
        cyphered_char = self._plugboard.get_char(cyphered_char);

        cyphered_char
    }

    pub fn increment_rotors(&mut self) {
        for i in 0..self._rotors.len() {
            
            // First rotor will increment on each key press, so always incrememnt if i is 0 
            if i == 0 {
                self._rotors[i].increment_position();
            }

            // If next rotor exists and current rotor is at turnover point, increment next rotor.
            if i < self._rotors.len() - 1 && self._rotors[i].should_step_next() {
                self._rotors[i + 1].increment_position();
            }
            else {
                break;
            }
        }
    }

    // Get the position of each rotor from left to right
    pub fn get_rotor_state(&self) -> String {
        self._rotors.iter().map(|rot| rot.get_position()).rev().collect()
    }
}

impl Enigma {
    pub fn new(rotors: Vec<(RotorVersion, char, char)>, reflector: ReflectorVersion, plugboard: &[(char, char)]) -> Self {
        // Create template Enigma instance
        let mut _enigma = Enigma {
            _rotors: Vec::new(),
            _reflector: Reflector::from_version(reflector),
            _plugboard: Plugboard::new(plugboard)
        };

        // Create Rotors from the configurations given
        for _rotor_info in rotors {
            // Push Rotor to the Enigma instance
            _enigma._rotors.push({
                let mut _rotor: Rotor = Rotor::from_version(
                    _rotor_info.0, 
                    _rotor_info.1.to_ascii_uppercase(), 
                    _rotor_info.2.to_ascii_uppercase());
                _rotor
            })
        }

        _enigma
    }
}
