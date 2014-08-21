use pipeline::{Input, Output, Registry};

mod pipeline;
mod inputs;
mod outputs;

fn main() {
	let r = get_registry();

	let mut inp = r.create_input("stdin").unwrap();
	let mut outp = r.create_output("stdout").unwrap();

	loop {
		let evt = inp.next_event();
		outp.receive_event(&evt);
	}
}

fn get_registry() -> Registry {
	let mut r = Registry::new();
	inputs::register(&mut r);
	outputs::register(&mut r);
	r
}