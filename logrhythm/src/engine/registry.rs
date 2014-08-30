use engine::{Input, Output};
use std::collections::HashMap;
use std::ascii::{OwnedAsciiExt, AsciiExt};

pub struct Registry {
	inputs: HashMap<String, fn() -> Box<Input>>,
	outputs: HashMap<String, fn() -> Box<Output>>
}

impl Registry {
	pub fn new() -> Registry {
		Registry {
			inputs: HashMap::new(),
			outputs: HashMap::new()
		}
	}

	pub fn add_input(&mut self, name: String, ctor: fn() -> Box<Input>) {
		self.inputs.insert(name.into_ascii_lower(), ctor);
	}

	pub fn add_output(&mut self, name: String, ctor: fn() -> Box<Output>) {
		self.outputs.insert(name.into_ascii_lower(), ctor);
	}

	pub fn get_input(&self, name: &str) -> Option<fn() -> Box<Input>> {
		match self.inputs.find_equiv(&(name.to_ascii_lower())) {
			Some(&ctor) => Some(ctor),
			None => None
		}
	}

	pub fn get_output(&self, name: &str) -> Option<fn() -> Box<Output>> {
		match self.outputs.find_equiv(&(name.to_ascii_lower())) {
			Some(&ctor) => Some(ctor),
			None => None
		}
	}
}

#[cfg(test)]
mod test {
	use engine::{Input, Output, Registry, Event};

	#[test]
	pub fn test_add_and_get_input() {
		let mut r = Registry::new();
		r.add_input("test".to_string(), make_input);
		assert!(r.get_input("test").is_some());
	}

	#[test]
	pub fn test_add_and_get_output() {
		let mut r = Registry::new();
		r.add_output("test".to_string(), make_output);
		assert!(r.get_output("test").is_some());	
	}

	#[test]
	pub fn test_case_insensitive_lookup() {
		let mut r = Registry::new();
		r.add_input("TestIn".to_string(), make_input);
		r.add_output("TestOut".to_string(), make_output);
		assert!(r.get_input("TESTIN").is_some());	
		assert!(r.get_output("TESTOUT").is_some());	
	}

	// Support code
	struct TestInput;

	impl Input for TestInput {
		fn next_event(&mut self) -> Event {
			fail!("not implemented! for testing only!");
		}
	}

	fn make_input() -> Box<Input> {
		box TestInput as Box<Input>
	}

	struct TestOutput;

	impl Output for TestOutput {
		#[allow(unused_variable)]
		fn receive_event(&mut self, evt: &Event) {
			fail!("not implemented! for testing only!");
		}
	}

	fn make_output() -> Box<Output> {
		box TestOutput as Box<Output>
	}
}