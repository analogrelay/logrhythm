use util::Ctor;
use engine::{Input, Output};
use std::collections::HashMap;
use std::ascii::{OwnedAsciiExt, AsciiExt};

pub struct Registry {
	inputs: HashMap<String, Box<Ctor<Box<Input>>+Send>>,
	outputs: HashMap<String, Box<Ctor<Box<Output>>+Send>>
}

impl Registry {
	pub fn new() -> Registry {
		Registry {
			inputs: HashMap::new(),
			outputs: HashMap::new()
		}
	}

	pub fn add_input(&mut self, name: String, factory: Box<Ctor<Box<Input>>+Send>) {
		self.inputs.insert(name.into_ascii_lower(), factory);
	}

	pub fn add_output(&mut self, name: String, factory: Box<Ctor<Box<Output>>+Send>) {
		self.outputs.insert(name.into_ascii_lower(), factory);
	}

	pub fn get_input(&self, name: &str) -> Option<Box<Ctor<Box<Output>>+Send>> {
		match self.inputs.find_equiv(&(name.to_ascii_lower())) {
			Some(factory) => Some(factory),
			None => None
		}
	}

	pub fn get_output(&self, name: &str) -> Option<Box<Ctor<Box<Output>>+Send>> {
		match self.outputs.find_equiv(&(name.to_ascii_lower())) {
			Some(factory) => Some(factory),
			None => None
		}
	}
}

#[cfg(test)]
mod test {
	use engine::{Input, Output, Factory, Registry, Event};

	#[test]
	pub fn test_add_and_create_input() {
		let mut r = Registry::new();
		r.add_input("test".to_string(), box TestInputFactory);
		assert!(r.create_input("test").is_some());
	}

	#[test]
	pub fn test_add_and_create_output() {
		let mut r = Registry::new();
		r.add_output("test".to_string(), box TestOutputFactory);
		assert!(r.create_output("test").is_some());	
	}

	#[test]
	pub fn test_case_insensitive_lookup() {
		let mut r = Registry::new();
		r.add_input("TestIn".to_string(), box TestInputFactory);
		r.add_output("TestOut".to_string(), box TestOutputFactory);
		assert!(r.create_input("TESTIN").is_some());	
		assert!(r.create_output("TESTOUT").is_some());	
	}

	// Support code
	struct TestInput;

	impl Input for TestInput {
		fn next_event(&mut self) -> Event {
			fail!("not implemented! for testing only!");
		}
	}

	struct TestOutput;

	impl Output for TestOutput {
		#[allow(unused_variable)]
		fn receive_event(&mut self, evt: &Event) {
			fail!("not implemented! for testing only!");
		}
	}

	struct TestInputFactory;

	impl Factory<Box<Input>> for TestInputFactory {
		fn build(&self) -> Box<Input> {
			box TestInput as Box<Input>
		}
	}

	struct TestOutputFactory;

	impl Factory<Box<Output>> for TestOutputFactory {
		fn build(&self) -> Box<Output> {
			box TestOutput as Box<Output>
		}
	}
}