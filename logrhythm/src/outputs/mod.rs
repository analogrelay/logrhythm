use pipeline::Registry;

mod stdout;

pub fn register(r: &mut Registry) {
	stdout::register(r);
}