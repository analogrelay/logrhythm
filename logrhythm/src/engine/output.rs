use engine::Event;

pub trait Output {
	fn receive_event(&mut self, evt: &Event);
}