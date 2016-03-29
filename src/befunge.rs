//! The Befunge interpreter module
//!
//! Contains the structs used to construct a Befunge interpreter.
//!
//! # Examples
//!
//! ```
//! use rubefunge_93::befunge;
//!
//! // Create a random number generator
//! let values = vec![0u32];
//! let instructions = vec!["1248::+1> #+?\\# _.@".chars().collect()];
//!
//! let program = befunge::Program::new(values, instructions);
//! let mut interpreter = befunge::Interpreter::from_program(program);
//!
//! interpreter.execute();
//! ```

use rand::{Rng, thread_rng};
use std::char;

/// The Befunge program stack
///
/// Contains a `Vec` with modified `push` and `pop` functions.
pub struct Stack {
	stack: Vec<u32>
}

/// Creates an empty stack.
impl Default for Stack {
	fn default() -> Stack {
		Stack {
			stack: Vec::new(),
		}
	}
}

impl Stack {

	/// Pops the Befunge stack.
	///
	/// Returns 0 if the stack is empty and the top item otherwise.
	pub fn pop(&mut self) -> u32 {
		match self.stack.pop() {
			Some(x) => x,
			None => 0,
		}
	}

	/// Pushes a new item to the stack.
	pub fn push(&mut self, item: u32) {
		self.stack.push(item);
	}

	/// Duplicates the top item on the stack.
	///
	/// If the stack is empty pushes two zeros to the stack.
	pub fn duplicate_top(&mut self) {
		match self.stack.pop() {
			Some(x) => {
				self.stack.push(x.clone());
				self.stack.push(x.clone());
			},
			None => {
				self.stack.push(0);
				self.stack.push(0);
			},
		}
	}

	/// Switches the two items at the top of the stack.
	///
	/// If the stack does not contain two items then zeros
	/// are pushed to the stack.
	pub fn switch_top(&mut self) {
		let a = self.stack.pop();

		if let Some(a) = a {
			let b = self.stack.pop();

			if let Some(b) = b {
				// Push in reverse order.
				self.stack.push(a);
				self.stack.push(b);
			} else {
				// Push a back and a zero.
				self.stack.push(a);
				self.stack.push(0);
			}
			
		} else {
			// Fill stack with two zeros.
			self.stack.push(0);
			self.stack.push(0);
		}
	}
}

/// The Befunge program
///
/// Contains the user specified values and the
/// instructions.
pub struct Program {
	values: Vec<u32>,
	instructions: Vec<Vec<char>>,
}

impl Program {
	/// Create a new Befunge program with specified user values
	/// and instructions.
	pub fn new(values: Vec<u32>, instructions: Vec<Vec<char>>) -> Program {
		Program {
			values: values,
			instructions: instructions,
		}
	}

	/// Number of lines in the Befunge program.
	pub fn lines(&self) -> usize {
		self.instructions.len()
	}

	/// Number of characters in the specified line.
	pub fn chars_in_line(&self, line: usize) -> usize {
		self.instructions[line].len()
	}

	/// Gets the next user values from the front of the `Vec`.
	pub fn next_value(&mut self) -> u32 {
		let value = self.values[0].clone();
		self.values = self.values.split_off(1);

		value
	}

	/// Gets the instruction character at the given position.
	pub fn get_instruction_char(&self, pos: [usize;2]) -> char {
		self.instructions[pos[0]][pos[1]]
	}

	/// Sets the instruction character at the given position.
	pub fn set_instruction_char(&mut self, pos: [usize;2], c: char) {
		self.instructions[pos[0]][pos[1]] = c;
	}
}

/// Direction for the instruction pointer.
#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
	/// Moving up.
	Up,
	/// Moving down.
	Down,
	/// Moving left.
	Left,
	/// Moving right.
	Right,
}

/// Current state of the interpreter.
#[derive(PartialEq, Clone, Copy)]
pub enum State {
	/// Normal mode.
	Normal,
	/// String capture mode.
	String,
}

/// Actions to be taken by the interpreter.
#[derive(PartialEq)]
pub enum Action {
	/// Change direction of pointer movement.
	ChangeDir(Direction),
	/// Change the state of the interpreter.
	ChangeState(State),
	/// Skip the next instruction character.
	Trampoline,
	/// Take no action.
	None,
	/// End the program.
	End,
}

/// The Befunge interpreter.
pub struct Interpreter {
	stack: Stack,
	direction: Direction,
	state: State,
	pos: [usize; 2],
	program: Program,
}

impl Interpreter {
	/// Create a new interpreter from the given program.
	pub fn from_program(program: Program) -> Interpreter {
		Interpreter {
			stack: Stack::default(),
			direction: Direction::Right,
			state: State::Normal,
			pos: [0,0],
			program: program,
		}
	}

	/// Execute the program with the interpreter.
	pub fn execute(&mut self) {
		let instruct_char = self.program.get_instruction_char(self.pos);
		let mut action = self.process_instruction(instruct_char);

		while action != Action::End {
			match action {
				Action::ChangeDir(ref direction) => self.direction = *direction,
				Action::ChangeState(ref state) => self.state = *state,
				Action::Trampoline => self.update_pos(),
				_ => {},
			} 

			self.update_pos();

			let instruct_char = self.program.get_instruction_char(self.pos);
			action = self.process_instruction(instruct_char);
		}

		self.end_program();
	}

	/// Update the position of the instruction pointer.
	fn update_pos(&mut self) {
		match self.direction {
			Direction::Right => {
				if self.pos[1] == self.program.chars_in_line(self.pos[0]) - 1 {
					self.pos[1] = 0;
				} else {
					self.pos[1] += 1;
				}
			},
			Direction::Left => {
				if self.pos[1] == 0 {
					self.pos[1] = self.program.chars_in_line(self.pos[0]) - 1;
				} else {
					self.pos[1] -= 1;

				}
			},
			Direction::Up => {
				if self.pos[0] == 0 {
					self.pos[0] = self.program.lines() - 1;
				} else {
					self.pos[0] -= 1;
				}
			},
			Direction::Down => {
				if self.pos[0] == self.program.lines() - 1 {
					self.pos[0] = 0;
				} else {
					self.pos[0] += 1;
				}
			},
		}
	}

	/// Process the instruction character.
	fn process_instruction(&mut self, instruction: char) -> Action {
		match self.state {

			State::String => {
				match instruction {
					'"' => Action::ChangeState(State::Normal),
					_ => {
						self.stack.push(instruction as u32);
						Action::None
						}
					}
				},

			State::Normal => {
				match instruction {
				'0'...'9' => {
					self.stack.push(instruction.to_digit(10).unwrap());
					Action::None
				},
				'+' => {
					let a = self.stack.pop();
					let b = self.stack.pop();
					self.stack.push(b + a);
					Action::None
				},
				'-' => {
					let a = self.stack.pop();
					let b = self.stack.pop();
					self.stack.push(b - a);
					Action::None
				},
				'*' => {
					let a = self.stack.pop();
					let b = self.stack.pop();
					self.stack.push(b * a);
					Action::None
				},
				'/' => {
					let a = self.stack.pop();
					let b = self.stack.pop();
					self.stack.push(b / a);
					Action::None
				},
				'%' => {
					let a = self.stack.pop();
					let b = self.stack.pop();
					self.stack.push(b % a);
					Action::None
				},
				'!' => {
					let a = self.stack.pop();
					self.stack.push(if a == 0 { 1 } else { 0 });
					Action::None
				},
				'`' => {
					let a = self.stack.pop();
					let b = self.stack.pop();
					self.stack.push(if b > a { 1 } else { 0 });
					Action::None
				},
				'>' => Action::ChangeDir(Direction::Right),
				'<' => Action::ChangeDir(Direction::Left),
				'^' => Action::ChangeDir(Direction::Up),
				'v' => Action::ChangeDir(Direction::Down),
				'?' => {
					let dir_int = thread_rng().gen_range(0,4);
					match dir_int {
						0 => Action::ChangeDir(Direction::Right),
						1 => Action::ChangeDir(Direction::Left),
						2 => Action::ChangeDir(Direction::Up),
						3 => Action::ChangeDir(Direction::Down),
						_ => panic!("Generated number outside of range. Blame rand!")
					}
				},
				'_' => {
					let a = self.stack.pop();
					if a == 0 {
						Action::ChangeDir(Direction::Right)
					} else {
						Action::ChangeDir(Direction::Left)
					}
				},
				'|' => {
					let a = self.stack.pop();
					if a == 0 {
						Action::ChangeDir(Direction::Down)
					} else {
						Action::ChangeDir(Direction::Up)
					}
				},
				'"' => Action::ChangeState(State::String),
				':' => {
					self.stack.duplicate_top();
					Action::None
				},
				'\\' => {
					self.stack.switch_top();
					Action::None
				},
				'$' => {
					self.stack.pop();
					Action::None
				},
				'.' => {
					print!("{} ", self.stack.pop());
					Action::None
				},
				',' => {
					print!("{} ", char::from_u32(self.stack.pop()).unwrap());
					Action::None
				},
				'#' => Action::Trampoline,
				'p' => {
					let x = self.stack.pop();
					let y = self.stack.pop();
					let v = self.stack.pop();

					self.program.set_instruction_char([x as usize,y as usize],
														char::from_u32(v).unwrap());
					Action::None
				},
				'g' => {
					let x = self.stack.pop();
					let y = self.stack.pop();
					self.stack.push(self.program.get_instruction_char([x as usize, y as usize]) as u32);
					Action::None
				},
				'&' => {
					let val = self.program.next_value();
					self.stack.push(val);
					Action::None
				},
				'~' => {
					let val = self.program.next_value();
					self.stack.push(val);
					Action::None
				},
				'@' => Action::End,
				_ => Action::None,
				}
			}
		}
	}

	/// End the Bufenge program.
	fn end_program(&self) {
		println!("\n----- Program Finished -----");
	}
}