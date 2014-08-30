use engine::Registry;

macro_rules! declare_input(
	($name:expr $ctor:block) => (
		pub fn register(r: &mut Registry) {
			fn make_it() -> Box<Input> $ctor

			debug!(" registering {} input", $name);
			r.add_input($name.to_string(), make_it);
		}
	)
)

mod stdin;

pub fn register(r: &mut Registry) {
	debug!("Registering input components");
	stdin::register(r);
}