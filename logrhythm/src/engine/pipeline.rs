use engine::{Input, Output};

pub static DEFAULT_WORKER_COUNT: int = 4;

pub struct PipelineBuilder {
	inputs: Vec<Box<Input>>,
	outputs: Vec<Box<Output>>,
	message_workers: int
}

impl PipelineBuilder {
	pub fn new() -> PipelineBuilder {
		PipelineBuilder::with_worker_count(DEFAULT_WORKER_COUNT)
	}

	pub fn with_worker_count(message_workers: int) -> PipelineBuilder {
		PipelineBuilder {
			inputs: Vec::new(),
			outputs: Vec::new(),
			message_workers: message_workers
		}
	}

	pub fn add_input(&mut self, input: Box<Input>) {
		self.inputs.push(input);
	}

	pub fn add_output(&mut self, output: Box<Output>) {
		self.outputs.push(output);
	}
}

pub trait Pipeline {
}