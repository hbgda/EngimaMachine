use std::collections::HashMap;

pub struct Plugboard {
    _key_map: HashMap<char, char>
}

impl Plugboard {
    pub fn get_char(&self, source: char) -> char {
        match self._key_map.get(&source) {
            Some(mapped) => *mapped,
            None => source,
        }
    }
}

impl Plugboard {
    pub fn new(key_map: &[(char, char)]) -> Self {
        let mut _plugboard: Plugboard = Plugboard {
            _key_map: HashMap::new()
        };

        for key_pair in key_map {
            if _plugboard._key_map.contains_key(&key_pair.0) || _plugboard._key_map.contains_key(&key_pair.1) {
                eprintln!("Invalid value passed for key_map {:#?}, key already exists.", key_pair)
            }
            _plugboard._key_map.insert(key_pair.0, key_pair.1);
            _plugboard._key_map.insert(key_pair.1, key_pair.0);
        }

        _plugboard
    }
}