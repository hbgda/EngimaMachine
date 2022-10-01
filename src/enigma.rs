use crate::plugboard::Plugboard;
use crate::rotor::{Rotor, RotorVersion};
use crate::reflector::{Reflector, ReflectorVersion};

pub struct Enigma {
    _rotors: Vec<Rotor>,    // Rotors stored in reverse for convenience, index 0 is right most rotor.
    _reflector: Reflector,
    _plugboard: Plugboard
}

impl Enigma {
    pub fn encrypt_char(&mut self, source: char) -> char {
        let mut _current_cyphered_char: char = source.to_ascii_uppercase();

        _current_cyphered_char = self._plugboard.get_char(_current_cyphered_char);


        for i in 0..self._rotors.len() {
            self.increment_rotors();
            _current_cyphered_char = self._rotors[i].get_cyphered_char(_current_cyphered_char);   
        }

        _current_cyphered_char = self._reflector.get_char(_current_cyphered_char);

        for i in (0..self._rotors.len()).rev() {
            _current_cyphered_char = self._rotors[i].get_cyphered_char(_current_cyphered_char);
        }

        _current_cyphered_char = self._plugboard.get_char(_current_cyphered_char);

        _current_cyphered_char
    }

    pub fn increment_rotors(&mut self) {
        for i in 0..self._rotors.len() {
            if i == 0 {
                self._rotors[i].increment_position();
            }

            // If next rotor exists and current rotor is at turnover point, incrememnt next rotor.
            if i < self._rotors.len() - 1 && self._rotors[i].should_step_next() {
                println!("Stepping next rotor.");
                self._rotors[i + 1].increment_position();
            }
            else {
                break;
            }
        }
    }
}

impl Enigma {
    pub fn new(rotors: Vec<(u8, char, char)>, reflector: ReflectorVersion, plugboard: &[(char, char)]) -> Self {
        let mut _enigma = Enigma {
            _rotors: Vec::new(),
            _reflector: Reflector::from_version(reflector),
            _plugboard: Plugboard::new(plugboard)
        };

        for _rotor_info in rotors {
            _enigma._rotors.push({
                let mut _rotor: Rotor = Rotor::from_version({
                    let _rotor_num: u8 = _rotor_info.0;
                    match _rotor_num {
                        0 => RotorVersion::I,
                        1 => RotorVersion::II,
                        2 => RotorVersion::III,
                        3 => RotorVersion::IV,
                        4 => RotorVersion::V,
                        5 => RotorVersion::VI,
                        6 => RotorVersion::VII,
                        7 => RotorVersion::VIII,
                        _ => {
                            eprintln!("Invalid Rotor {_rotor_num}, skipping.");
                            continue;
                        }
                    }
                }, _rotor_info.1.to_ascii_uppercase(), _rotor_info.2.to_ascii_uppercase());
                println!("{:#?}", _rotor);
                _rotor
            })
        }

        _enigma
    }
}
