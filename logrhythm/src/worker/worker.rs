use std::task;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::comm::{Sender, Receiver, channel};

pub enum WorkerStatus {
	Initializing,
	Running,
	Stopped
}

pub struct CancellationToken {
	signaled: AtomicBool
}

pub trait Worker {
	fn label(&self) -> &str;
	fn task_name(&self) -> &str;
	fn status(&self) -> WorkerStatus;
}

pub trait WorkerSignaler {
	fn is
}

struct WorkerImpl {
	label: String,
	status: WorkerStatus,
	name: String
}

impl Worker for WorkerImpl {
	fn label(&self) -> &str {
		self.label.as_slice()
	}
	fn status(&self) -> WorkerStatus {
		self.status
	}
	fn task_name(&self) -> &str {
		self.name.as_slice()
	}
}

pub fn spawn_worker(label: String, code: proc()) -> Box<Worker> {
	// Create the worker control object
	let mut worker = Arc::new(box WorkerImpl {
		label: label,
		status: Initializing
	});

	spawn(proc() {
		// Set up the task
		worker.status = Running;
		worker.name = task::name().unwrap_or_else(|| -> "".to_string());

		// Run the code!
		code();
	});
}