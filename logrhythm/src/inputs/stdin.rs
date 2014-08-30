extern crate time;

use std::io;

use engine::{Input, Event, Registry};

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
					line.as_slice().trim_right_chars(&['\r', '\n']).to_string())
			},
			Err(e) => {
				error!("io error: {}", e);
				fail!("io error. aborting. todo: recovery");
			}
		}
	}
}

declare_input!("stdin" {
	box StdIn {
		reader: box io::stdin()
	} as Box<Input>
})

#[cfg(test)]
mod test {
	extern crate time;

	use inputs::stdin::StdIn;
	use engine::Input;

	use std::io::MemReader;

	#[test]
	pub fn test_read_event() {
		// Create a test event in a buffer
		let buf = box MemReader::new("test event\r\n".to_string().into_bytes());

		// Create the reader
		let mut testIn = StdIn {
			reader: buf
		};

		// Read the event!
		let evt = testIn.next_event();

		assert_eq!(evt.message.as_slice(), "test event");
	}
}