use engine::Event;
use std::comm::Receiver;

pub trait Output {
	fn receive_event(&mut self, evt: &Event);
}