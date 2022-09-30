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

        for i in key_map {
            if _plugboard._key_map.contains_key(&i.0) || _plugboard._key_map.contains_key(&i.1) {
                eprintln!("Invalid value passed for key_map {:#?}, key already exists.", i)
            }
            _plugboard._key_map.insert(i.0, i.1);
            _plugboard._key_map.insert(i.1, i.0);
        }

        _plugboard
    }
}