pub mod rotor;
pub mod enigma;
pub mod utils;
pub mod reflector;
pub mod plugboard;

use enigma::Enigma;
use rotor::RotorVersion;
use reflector::ReflectorVersion;

// TODO: User configurable Enigma
// TODO: Wasm interface

fn main() {
    let mut enigma: Enigma = Enigma::new(
                                vec![(RotorVersion::I, 'A', 'Z'), (RotorVersion::II, 'A', 'A'), (RotorVersion::III, 'A', 'A')], 
                                ReflectorVersion::B, 
                                &[]);

    let mut input: String = String::new();

    println!("Enter text to encrypt:");

    std::io::stdin().read_line(&mut input)
        .expect("Failed to read input.");

    input = input.trim().to_string();
    
    let output: String = enigma.encrypt_string(&input);

    println!("Input:  {input}\nOutput: {output}");

    let rotor_state = enigma.get_rotor_state();
    println!("Final State: {rotor_state}")
    // let test_char = 'A';
    // for _ in 0..126 {
    //     let _cyphered = enigma.encrypt_char(test_char);
    //     println!("Source: {test_char} | Cyphered: {_cyphered}");
    // }
}