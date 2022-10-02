pub mod rotor;
pub mod enigma;
pub mod utils;
pub mod reflector;
pub mod plugboard;

use enigma::Enigma;
use rotor::RotorVersion;
use reflector::ReflectorVersion;


fn main() {
    let mut enigma: Enigma = Enigma::new(
                                vec![(RotorVersion::I, 'A', 'Z'), (RotorVersion::II, 'A', 'B'), (RotorVersion::III, 'A', 'A')], 
                                ReflectorVersion::B, 
                                &[]);
    let test_char = 'A';
    for _ in 0..126 {
        let _cyphered = enigma.encrypt_char(test_char);
        println!("Source: {test_char} | Cyphered: {_cyphered}");
    }
    let rotor_state = enigma.get_rotor_state();
    println!("Final State: {rotor_state}")
}