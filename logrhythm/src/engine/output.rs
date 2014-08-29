use engine::Event;

pub trait Output : Send {
	fn receive_event(&mut self, evt: &Event);
}