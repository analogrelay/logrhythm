extern crate time;

use std::io;

use pipeline::{Input, Event, Factory, Registry};

pub struct StdIn { 
	reader: Box<io::Buffer>
}

impl Input for StdIn {
	fn next_event(&mut self) -> Event {
		match self.reader.read_line() {
			Ok(line) => {
				debug!("received: {}", line);
				Event::new(
					time::now_utc(),
					line.as_slice().trim_right_chars(&['\r', '\n']).to_string()))
			},
			Err(e) => {
				error!("io error: {}", e);
				fail!("io error. aborting. todo: recovery");
			}
		}
	}
}

struct StdInFactory;

impl Factory<Box<Input>> for StdInFactory {
	fn build(&self) -> Box<Input> {
		box StdIn {
			reader: box io::stdin()
		} as Box<Input>
	}
}

pub fn register(r: &mut Registry) {
	debug!(" registered stdin component");
	r.add_input("stdin".to_string(), box StdInFactory);
}