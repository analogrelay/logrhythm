use engine::Registry;

mod stdout;

pub fn register(r: &mut Registry) {
	debug!("Registering output components");
	stdout::register(r);
}