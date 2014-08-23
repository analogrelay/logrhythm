use engine::Event;

pub trait Input {
	fn next_event(&mut self) -> Event;
}