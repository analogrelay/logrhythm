extern crate time;

use std::collections::HashMap;
//use std::collections::TreeMap;
//use std::any::Any;

pub struct Event {
	pub timestamp: time::Tm,
	pub message: String,
	//fields: TreeMap<String, Box<Any>>
}

impl Event {
	pub fn new(timestamp: time::Tm, message: String) -> Event {
		Event {
			timestamp: timestamp,
			message: message,
			//fields: TreeMap::new()
		}
	}
}

pub trait Input {
	fn next_event(&mut self) -> Event;
}

pub trait Output {
	fn receive_event(&mut self, evt: &Event);
}

pub trait Factory<T> {
	fn build(&self) -> T;
}

pub struct Registry {
	inputs: HashMap<String, Box<Factory<Box<Input>>>>,
	outputs: HashMap<String, Box<Factory<Box<Output>>>>
}

impl Registry {
	pub fn new() -> Registry {
		Registry {
			inputs: HashMap::new(),
			outputs: HashMap::new()
		}
	}

	pub fn add_input(&mut self, name: String, factory: Box<Factory<Box<Input>>>) {
		self.inputs.insert(name, factory);
	}

	pub fn add_output(&mut self, name: String, factory: Box<Factory<Box<Output>>>) {
		self.outputs.insert(name, factory);
	}

	pub fn create_input(&self, name: &str) -> Option<Box<Input>> {
		match self.inputs.find_equiv(&name) {
			Some(factory) => Some(factory.build()),
			None => None
		}
	}

	pub fn create_output(&self, name: &str) -> Option<Box<Output>> {
		match self.outputs.find_equiv(&name) {
			Some(factory) => Some(factory.build()),
			None => None
		}
	}
}