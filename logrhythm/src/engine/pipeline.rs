use engine::{Input, Output, Event};
use std::sync::deque::BufferPool;
use std::sync::mpsc_queue::Queue;
use std::sync::Arc;
use std::comm::channel;

//pub static DEFAULT_WORKER_COUNT: int = 4;

pub struct PipelineBuilder {
	inputs: Vec<Box<Input>>,
	outputs: Vec<Box<Output>>,
}

impl PipelineBuilder {
	pub fn new() -> PipelineBuilder {
		PipelineBuilder {
			inputs: Vec::new(),
			outputs: Vec::new()
		}
	}

	pub fn add_input(&mut self, input: Box<Input>) {
		self.inputs.push(input);
	}

	pub fn add_output(&mut self, output: Box<Output>) {
		self.outputs.push(output);
	}
}

pub fn run_pipeline(builder: Box<PipelineBuilder>) {
	info!("Starting pipeline");

	// Create queues
	let input_queue = Arc::new(Queue::new());

	// Spawn tasks to read from the inputs
	for inp in builder.inputs.move_iter() {
		let my_queue = input_queue.clone();
		spawn(proc() {
			run_input_loop(inp, my_queue);
		})
	}

	// Spawn tasks to write to the outputs
	let mut output_channels = Vec::new();
	for outp in builder.outputs.move_iter() {
		let (tx, rx) = channel();
		output_channels.push(tx);
		spawn(proc() {
			run_output_loop(outp, rx);
		})
	}

	// Pump messages from the input queue to the output channels!
	loop {
		match input_queue.casual_pop() {
			Some(evt) => {
				for out_chan in output_channels.iter() {
					out_chan.send(evt.clone());
				}
			},
			None => {}
		}
	}
}

fn run_input_loop(inp: Box<Input>, queue: Arc<Queue<Event>>) {
	loop {
		let evt = inp.next_event();
		queue.push(evt);
	}
}

fn run_output_loop(outp: Box<Output>, input: Receiver<Event>) {
	loop {
		let evt = input.recv();
		outp.receive_event(&evt);
	}
}