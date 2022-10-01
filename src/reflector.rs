use std::{collections::HashMap};

pub enum ReflectorVersion {
    B,
    C,
    BD,
    CD,
    None
}

pub struct Reflector {
    _id: String,
    _key_map: HashMap<char, char>
}

impl Reflector {
    pub fn get_char(&self, source: char) -> char {
        match self._key_map.get(&source) {
            Some(mapped) => *mapped,
            None => source
        }
    }
}

impl Reflector {
    pub fn new(id: String, key_map: &[(char, char)]) -> Self {
        let mut _reflector: Reflector = Reflector {
             _id: id, 
             _key_map: HashMap::new()
        };

        for key_pair in key_map {
            if _reflector._key_map.contains_key(&key_pair.0) || _reflector._key_map.contains_key(&key_pair.1) {
                eprintln!("Invalid value passed for key_map {:#?}, key already exists.", key_pair)
            }
            _reflector._key_map.insert(key_pair.0, key_pair.1);
            _reflector._key_map.insert(key_pair.1, key_pair.0);
        }

        _reflector
    }
}

impl Reflector {
    pub fn from_version(version: ReflectorVersion) -> Self {
        match version {
            ReflectorVersion::B => {
                Reflector::new(String::from("B"), &[('A', 'Y'), ('B', 'R'), ('C', 'U'), ('D', 'H'), 
                                                                ('E', 'Q'), ('F', 'S'), ('G', 'L'), ('I', 'P'), 
                                                                ('J', 'X'), ('K', 'N'), ('M', 'O'), ('T', 'Z'), 
                                                                ('V', 'W')])
            },
            ReflectorVersion::C => {
                Reflector::new(String::from("C"), &[('A', 'F'), ('B', 'V'), ('C', 'P'), ('D', 'J'),
                                                                ('E', 'I'), ('G', 'O'), ('H', 'Y'), ('K', 'R'),
                                                                ('L', 'Z'), ('M', 'X'), ('N', 'W'), ('T', 'Q'),
                                                                ('S', 'U')])
            }
            ReflectorVersion::BD => todo!(),
            ReflectorVersion::CD => todo!(),
            ReflectorVersion::None => {
                Reflector::new(String::from("None"), &[])
            },
        }
    }
}