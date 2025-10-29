use std::fmt;

// #[derive(Debug)]
pub struct Stack<T> {
	value: Option<T>,
	next: Option<Box<Stack<T>>>,
	length: usize,
}

impl<T> Stack<T> {
	pub fn new() -> Stack<T> {
		Stack {
			value: None,
			next: None,
			length: 0,
		}
	}

	pub fn push(self, x: T) -> Stack<T> {
		if self.length == 0 {
			return Stack {
				value: Some(x),
				next: None,
				length: self.length + 1,
			};
		}

		Stack {
			value: Some(x),
			length: self.length + 1,
			next: Some(Box::new(self)),
		}
	}

	pub fn pop(self) -> (Stack<T>, Option<T>) {
		if self.length == 0 {
			return (self, None);
		}

		if self.length == 1 {
			return (
				Stack {
					value: None,
					length: 0,
					next: None,
				},
				self.value,
			);
		}

		(*(self.next.unwrap()), self.value)
	}
}

impl<T> fmt::Debug for Stack<T>
where
	T: fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[")?;

		if self.value.is_some() {
			write!(f, "{:?}", self.value.as_ref().unwrap())?;

			let mut current: &Option<Box<Stack<T>>> = &self.next;
			while current.is_some() {
				write!(
					f,
					", {:?}",
					current.as_ref().unwrap().value.as_ref().unwrap()
				)?;
				current = &(current.as_ref().unwrap().next);
			}
		}
		write!(f, "]")
	}
}
