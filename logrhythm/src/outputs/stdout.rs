extern crate time;

use std::io;
use pipeline::{Output, Event, Factory, Registry};

pub struct StdOut {
	writer: Box<io::Writer>
}

impl Output for StdOut {
	fn receive_event(&mut self, evt: &Event) {
		// Format the time
		debug!("writing event: {}", evt.id);
		let result = writeln!(self.writer, "{}: {}", &evt.timestamp.rfc3339(), &evt.message);
		if result.is_err() {
			fail!("error writing to stdout!");
		}
	}
}

struct StdOutFactory;

impl Factory<Box<Output>> for StdOutFactory {
	fn build(&self) -> Box<Output> {
		box StdOut {
			writer: box io::stdout()
		} as Box<Output>
	}
}

pub fn register(r: &mut Registry) {
	debug!(" registered stdout component");
	r.add_output("stdout".to_string(), box StdOutFactory);
}