pub mod rotor;
pub mod enigma;
pub mod utils;
pub mod reflector;
pub mod plugboard;

use crate::enigma::Enigma;
use crate::reflector::ReflectorVersion;


fn main() {
    let mut enigma: Enigma = Enigma::new(
                                vec![(0, 'A', 'Z')], 
                                ReflectorVersion::B, 
                                &[]);
                    
    for _ in 0..4 {
        let _cyphered = enigma.encrypt_char('A');
        println!("Source: A | Cyphered: {_cyphered}");
    }
}