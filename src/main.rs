pub mod rotor;
pub mod enigma;
pub mod utils;
pub mod reflector;
pub mod plugboard;

use crate::enigma::Enigma;
use crate::reflector::ReflectorVersion;


fn main() {
    let mut enigma: Enigma = Enigma::new(
                                vec![(0, 'Z', 'Z'), (0, 'Z', 'A')], 
                                ReflectorVersion::B, 
                                &[('A', 'C')]);
    for _ in 0..4 {
        let _cyphered = enigma.encrypt_char('A');
        println!("Source: A | Cyphered: {_cyphered}");
    }
}