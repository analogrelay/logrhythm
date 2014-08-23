extern crate time;

pub struct Event {
	pub timestamp: time::Tm,
	pub message: String,
	//fields: TreeMap<String, Box<Any>>
}

impl Event {
	pub fn new(timestamp: time::Tm, message: String) -> Event {
		debug!("creating event from stdin");
		Event {
			timestamp: timestamp,
			message: message,
			//fields: TreeMap::new()
		}
	}
}