extern crate time;

use std::Vec;
use std::iter::{Iterator, range};
use std::task::TaskBuilder;
use std::comm::channel;
use std::sync::mpsc_queue;
use std::sync::atomics::{AtomicInt, INIT_ATOMIC_INT, SeqCst};
use std::sync::deque::BufferPool;
use std::collections::HashMap;

pub use input::Input;
pub use output::Output;
pub use registry::{Factory, Registry};

mod input;
mod output;
mod registry;

static mut NEXT_EVENT: AtomicInt = INIT_ATOMIC_INT;

pub static DEFAULT_WORKER_COUNT: int = 4;

pub struct Event {
	pub id: int,
	pub timestamp: time::Tm,
	pub message: String,
	//fields: TreeMap<String, Box<Any>>
}

enum PipelineControlMessage {
	Shutdown
}

enum OutputMessage {
	Event(Event),
	Control(PipelineControlMessage)
}

impl Event {
	pub fn new(timestamp: time::Tm, message: String) -> Event {
		unsafe {
			let id = NEXT_EVENT.fetch_add(1, SeqCst);
			debug!("creating event #{}", id);
			Event {
				id: id
				timestamp: timestamp,
				message: message,
				//fields: TreeMap::new()
			}
		}
	}
}

pub trait Pipeline {

}

struct PipelineImpl;

pub struct PipelineBuilder {
	inputs: Vec<Box<Input>>,
	outputs: Vec<Box<Output>>,
	message_workers: int
}

impl PipelineBuilder {
	fn new() -> PipelineBuilder {
		PipelineBuilder::with_worker_count(DEFAULT_WORKER_COUNT);
	}

	fn with_worker_count(message_workers: int) -> PipelineBuilder -> {
		PipelineBuilder {
			inputs: Vec::new(),
			outputs: Vec::new(),
			message_workers: message_workers
		}
	}

	fn add_input(&mut self, input: Box<Input>) {
		self.inputs.insert(input);
	}

	fn add_output(&mut self, output: Box<Output>) {
		self.outputs.insert(output);
	}
}

fn start_pipeline(builder: Box<PipelineBuilder>) -> Box<Pipeline> {
	// Build the output channels and tasks
	let output_channels : Vec = builder.outputs.move_iter()
		.map(|output| -> {
			let (output_tx, output_rx) = channel();
			spawn(proc() { Output::run_loop(output, output_rx); });
			output_tx
		})
		.collect();

	// Build the message workers
	let pool = BufferPool::new();
	let (mut worker, mut stealer) = pool.deque();
	for i in range(0, builder.message_workers) {
		let my_stealer = stealer.clone();
		spawn(proc() {
			loop {
				// Steal a message from the deque
				
			}
		})
	}
}