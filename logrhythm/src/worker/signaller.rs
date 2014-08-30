use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Relaxed};

pub trait Signaller {
	fn signal(&mut self);
	fn token(&self) -> SignalToken;
}

pub fn signaller() -> Box<Signaller> {
	box SignallerImpl {
		signal: Arc::new(AtomicBool::new(false))
	} as Box<Signaller>
}

struct SignallerImpl {
	signal: Arc<AtomicBool>
}

impl Signaller for SignallerImpl {
	fn signal(&mut self) {
		self.signal.swap(true, Relaxed);
	}

	fn token(&self) -> SignalToken {
		SignalToken {
			signal: self.signal.clone()
		}
	}
}

#[deriving(Clone)]
pub struct SignalToken {
	signal: Arc<AtomicBool>
}

impl SignalToken {
	fn is_signalled(&self) -> bool {
		self.signal.load(Relaxed)
	}
}

#[cfg(test)]
mod test {
	use std::task::{try_future, deschedule};
	use super::signaller;

	#[test]
	pub fn test_signaller() {
		let mut signlr = signaller();

		// Set up a child task
		let token = signlr.token();
		let child_task = try_future(proc() {
			while !token.is_signalled() {
				deschedule(); // Yield
			}
			42i
		});

		// Signal shutdown and yield scheduler.
		signlr.signal();
		deschedule();

		// Read the final resupt
		assert_eq!(42, child_task.unwrap().ok().unwrap());
	}
}