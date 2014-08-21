use pipeline::Registry;

mod stdin;

pub fn register(r: &mut Registry) {
	stdin::register(r);
}