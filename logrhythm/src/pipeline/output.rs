pub trait Output {
	fn receive_event(&mut self, evt: &Event);

	fn run_loop(output: Box<Output>, output_rx: Receiver<) {
		loop {
			match output_rx.recv() {
				Event(evt) => output.receive_event(&evt),
				Control(Shutdown) => return;
			}
		}
	}
}