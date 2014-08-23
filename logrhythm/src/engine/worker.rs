use std::comms::Sender;

enum ControlMessage {
	Shutdown
}

pub struct Worker {
	control_tx: Sender<ControlMessage>
}

impl Worker {
	fn shutdown(&self) {
		control_tx.send(Shutdown);
	}
}