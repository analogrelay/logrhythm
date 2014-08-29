pub trait Ctor<T> : Send + Clone {
	fn new(&self) -> T;
}