#![allow(dead_code)]

#![feature(phase)]
#[phase(plugin, link)] extern crate log;

use engine::{Registry, PipelineBuilder, run_pipeline};

mod engine;
mod inputs;
mod outputs;
mod util;

fn main() {
	debug!("Starting logrhythm driver");
	let r = get_registry();
	
}

fn get_registry() -> Registry {
	debug!("Creating Component Registry");
	
	let mut r = Registry::new();
	inputs::register(&mut r);
	outputs::register(&mut r);
	r
}