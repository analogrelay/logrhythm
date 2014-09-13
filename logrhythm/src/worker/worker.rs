use std::sync::{Arc, Future};
use std::cell::Cell;
use std::task::{TaskBuilder, failing};
use worker::{Signaller, signaller};

pub enum WorkerStatus {
	Starting,
	Running,
	Stopped,
	Failed
}

// Represents the external view of the worker
pub trait Worker {
	fn name<'a>(&'a self) -> &'a str;
	fn request_shutdown(&self);
	fn status(&self) -> WorkerStatus;
	fn failure(&self) -> &Any;

	fn done(&self) -> bool {
		match self.status() {
			Stopped | Failed => true,
			_ => false
		}
	}
}

// Represent the internal view of the worker
pub trait WorkerContext {
	fn name<'a>(&'a self) -> &'a str;
	fn shutdown_requested(&self) -> bool;
}

pub fn spawn<'a>(name: String, f: proc(&WorkerContext)) -> Box<Worker+'a> {
	// Create WorkerImpl.
	let mut worker = WorkerImpl	{
		name: name,
		signaller: signaller(),
		status: Starting
	}

	// Move the WorkerImpl into the Arc.
	let shared_worker = Arc::new(worker);

	// Build a Context for the task to use to communicate with the Worker
	let mut context = make_context(shared_worker.clone());

	// Spawn the task
	let mut result_future = Some(TaskBuilder::new()
		.named(name)
		.try_future(proc() {
			context.worker.state.set(Running);
			f(&context);
		}));
}

struct WorkerImpl {
	name: String,
	signaller: Signaller,
	state: Cell<WorkerStatus>,
	failure: Option<Future<Result<(), Box<Any+Send>>>>
}

impl WorkerImpl {
	fn new(name: String) -> WorkerImpl {
		WorkerImpl {
			name: name,
			signaller: signaller()
		}
	}

	fn signal_shutdown(&self) {
		self.signaller.signal();
	}
}

struct WorkerContextImpl {
	name: String,
	signal_token: SignalToken,
	worker: Arc<WorkerImpl>
}

impl Drop for WorkerContextImpl {
	fn drop(&mut self) {
		if(failing()) {
			self.worker.state.set(Failed);
		}
		else {
			self.worker.state.set(Stopped);
		}
	}
}

fn make_context(worker: RefCell<WorkerImpl>) -> WorkerContextImpl {
	WorkerContextImpl {
		name: self.name.clone(),
		signal_token: self.signaller.token(),
		worker: worker
	}
}