use util::Ctor;
use engine::{Input, Output, Event};
use std::sync::deque::BufferPool;
use std::sync::mpsc_queue::Queue;
use std::sync::Arc;
use std::comm::channel;

//pub static DEFAULT_WORKER_COUNT: int = 4;

pub struct PipelineBuilder {
	inputs: Vec<Box<Ctor<Box<Input>>+Send>>,
	outputs: Vec<Box<Ctor<Box<Output>>+Send>>,
}

impl PipelineBuilder {
	pub fn new() -> PipelineBuilder {
		PipelineBuilder {
			inputs: Vec::new(),
			outputs: Vec::new()
		}
	}

	pub fn add_input(&mut self, input: Box<Ctor<Box<Input>>+Send>) {
		self.inputs.push(input);
	}

	pub fn add_output(&mut self, output: Box<Ctor<Box<Output>>+Send>) {
		self.outputs.push(output);
	}
}

pub fn run_pipeline(builder: Box<PipelineBuilder>) {
	info!("Starting pipeline");

	// Move the pipeline into a local
	let pipeline = *builder;

	// Create queues
	let input_queue = Arc::new(Queue::new());

	// Spawn tasks to read from the inputs
	for inp in pipeline.inputs.move_iter() {
		let my_queue = input_queue.clone();
		spawn(proc() {
			let input = inp.new();
			run_input_loop(input, my_queue);
		})
	}

	// Spawn tasks to write to the outputs
	let mut output_channels = Vec::new();
	for outp in pipeline.outputs.move_iter() {
		let (tx, rx) = channel();
		output_channels.push(tx);
		spawn(proc() {
			let output = outp.new();
			run_output_loop(output, rx);
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

fn run_input_loop(mut inp: Box<Input>, queue: Arc<Queue<Event>>) {
	loop {
		let evt = inp.next_event();
		queue.push(evt);
	}
}

fn run_output_loop(mut outp: Box<Output>, input: Receiver<Event>) {
	loop {
		let evt = input.recv();
		outp.receive_event(&evt);
	}
}