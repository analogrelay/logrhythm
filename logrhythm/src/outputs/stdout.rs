extern crate time;

use std::io;
use engine::{Output, Event, Registry};

pub struct StdOut {
	writer: Box<io::Writer>
}

impl Output for StdOut {
	fn receive_event(&mut self, evt: &Event) {
		// Format the time
		debug!("writing event to stdout");
		let result = writeln!(self.writer, "{}: {}", &evt.timestamp.rfc3339(), &evt.message);
		if result.is_err() {
			fail!("error writing to stdout!");
		}
	}
}

declare_output!("stdout" {
	box StdOut {
		writer: box io::stdout()
	} as Box<Output>
})

#[cfg(test)]
mod test {
	extern crate time;

	use outputs::stdout::StdOut;
	use engine::{Event, Output};

	use std::io::MemWriter;

	#[test]
	pub fn test_write_event() {
		// Create a test event and a buffer to hold the output
		let mut buf = box MemWriter::new();
		let timestamp = time::strptime("2014-08-22T03:40:16Z", "%Y-%m-%dT%TZ").unwrap();
		let evt = Event::new(timestamp, "test event".to_string());

		// Create the writer
		let mut testOut = StdOut {
			writer: box buf.by_ref()
		};

		// Write the event
		testOut.receive_event(&evt);

		// Unwrap the buffer
		let message = String::from_utf8(buf.unwrap()).unwrap();
		assert_eq!(message.as_slice(), "2014-08-22T03:40:16Z: test event\n");
	}
}