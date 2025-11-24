pub trait Push<T> {
	fn push(self, x : T) -> Self;
}

pub trait Pop<T> {
	fn pop(self) -> (Option<T>, Self);
}

pub struct Stack<T> {
	last : Option<T>,
	values : Option<Box<Stack<T>>>,
	length : usize
}

pub struct Queue<T> {
	first : Option<T>,
	values : Option<Box<Stack<T>>>,
	last : Option<T>,
	length : usize
}

pub fn stack<T>() -> Stack<T> { Stack { last : None, values : None, length : 0 } }

pub fn queue<T>() -> Queue<T> { Queue { last : None, values : None, first : None, length : 0 } }

impl<T> Push<T> for Stack<T> {
	fn push(self, x : T) -> Stack<T> {
		if self.length == 0 {
			return Stack { last : Some(x), values : None, length : 1 };
		}

		Stack { last : Some(x), length : self.length + 1, values : Some(Box::new(self)) }
	}
}

impl<T> Pop<T> for Stack<T> {
	fn pop(self) -> (Option<T>, Stack<T>) { (None, stack()) }
}

impl<T> Push<T> for Queue<T> {
	fn push(self, x : T) -> Queue<T> { queue() }
}

impl<T> Pop<T> for Queue<T> {
	fn pop(self) -> (Option<T>, Queue<T>) { (None, self) }
}
