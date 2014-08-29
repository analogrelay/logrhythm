use engine::Event;

pub trait Input : Send {
	fn next_event(&mut self) -> Event;
}