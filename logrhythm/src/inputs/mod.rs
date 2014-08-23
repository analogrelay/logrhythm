use pipeline::Registry;

mod stdin;

pub fn register(r: &mut Registry) {
	debug!("Registering input components");
	stdin::register(r);
}