#![allow(dead_code)]

#![feature(phase)]
#[phase(plugin, link)] extern crate log;

use engine::{Input, Output, Registry};

mod engine;
mod inputs;
mod outputs;

fn main() {
	debug!("Starting logrhythm driver");
	let r = get_registry();
	
	let mut inp = r.create_input("stdin").unwrap();
	let mut outp = r.create_output("stdout").unwrap();

	loop {
		let evt = inp.next_event();
		outp.receive_event(&evt);
	}
}

fn get_registry() -> Registry {
	debug!("Creating Component Registry");
	
	let mut r = Registry::new();
	inputs::register(&mut r);
	outputs::register(&mut r);
	r
}