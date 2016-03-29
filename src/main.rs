//! RuBefunge-93
//!
//! A simple, largely incomplete Befunge interpreter written in Rust.

extern crate rand;
pub mod befunge;

fn main() {
	// Create a random number generator
    let values = vec![0u32];
    //let instructions = vec!["1248::+1> #+?\\# _.@".chars().collect()];

    // Create the Sieve of Eratosthenes
    let mut instructions = Vec::new();
    instructions.push("2>:3g\" \"-!v\\  g30          <".chars().collect());
    instructions.push(" |!`\"O\":+1_:.:03p>03g+:\"O\"`|".chars().collect());
    instructions.push(" @               ^  p3\\\" \":<".chars().collect());
    instructions.push("2 234567890123456789012345678901234567890123456789012345678901234567890123456789".chars().collect());

	let program = befunge::Program::new(values, instructions);
	let mut interpreter = befunge::Interpreter::from_program(program);

	interpreter.execute();
}
