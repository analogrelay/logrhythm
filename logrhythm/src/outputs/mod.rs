use engine::Registry;

macro_rules! declare_output(
	($name:expr $ctor:block) => (
		pub fn register(r: &mut Registry) {
			fn make_it<'a>() -> Box<Output+'a> $ctor
			
			debug!(" registering {} component", $name);
			r.add_output($name.to_string(), make_it);
		}
	)
)

mod stdout;

pub fn register(r: &mut Registry) {
	debug!("Registering output components");
	stdout::register(r);
}