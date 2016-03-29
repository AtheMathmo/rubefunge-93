//! RuBefunge-93
//!
//! A simple, largely incomplete Befunge compiler written in Rust.

extern crate rand;
pub mod befunge;

fn main() {
	// Create a random number generator
    let values = vec![0u32];
    let instructions = vec!["1248::+1> #+?\\# _.@".chars().collect()];

	let program = befunge::Program::new(values, instructions);
	let mut interpreter = befunge::Interpreter::from_program(program);

	interpreter.execute();
}
