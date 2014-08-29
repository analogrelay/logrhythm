use engine::Event;
use std::sync::mpsc_queue::Queue;

pub trait Input {
	fn next_event(&mut self) -> Event;
}