#![allow(dead_code)]

#![feature(phase)]
#[phase(plugin, link)] extern crate log;

use engine::{Registry, PipelineBuilder, run_pipeline};

mod engine;
mod inputs;
mod outputs;

fn main() {
	debug!("Starting logrhythm driver");
	let r = get_registry();
	
	let mut builder = box PipelineBuilder::new();
	builder.add_input(r.create_input("stdin").unwrap());
	builder.add_output(r.create_output("stdout").unwrap());

	// Create the engine and run it!
	run_pipeline(builder);
}

fn get_registry() -> Registry {
	debug!("Creating Component Registry");
	
	let mut r = Registry::new();
	inputs::register(&mut r);
	outputs::register(&mut r);
	r
}