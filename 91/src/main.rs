#[derive(Debug)]
struct Rectangle {
	w: u32,
	h: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.w * self.h
	}

	fn perimeter(&self) -> u32 {
		2 * self.w + 2 * self.h
	}

	fn scale(&mut self, x: f32) {
		self.w = (self.w as f32 * x) as u32;
		self.h = (self.h as f32 * x) as u32;
	}

	fn scale(&mut self, x: u32) {
		self.w = self.w * x;
		self.h = self.h * x;
	}
}

fn main() {
	let mut r = Rectangle { w: 4, h: 5 };

	dbg!(&r);
	dbg!(&r.area());

	r.scale(1.5);
	dbg!(&r);
	dbg!(&r.area());
}
