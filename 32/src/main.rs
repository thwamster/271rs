fn main() {
	dbg!(max!(2, 1) == 2);
	dbg!(max!(2, 3, 1) == 3);

	dbg!(min!(1, 2) == 1);
	dbg!(min!(3, 2, 1) == 1);

	dbg!(choice!(0, 0, 0) == 0);
	dbg!(choice!(0, 0, 1) == 1);
	dbg!(choice!(0, 1, 0) == 0);
	dbg!(choice!(0, 1, 1) == 1);
	dbg!(choice!(1, 0, 0) == 0);
	dbg!(choice!(1, 0, 1) == 0);
	dbg!(choice!(1, 1, 0) == 1);
	dbg!(choice!(1, 1, 1) == 1);

	dbg!(median!(0, 0, 0) == 0);
	dbg!(median!(0, 0, 1) == 0);
	dbg!(median!(0, 1, 0) == 0);
	dbg!(median!(0, 1, 1) == 1);
	dbg!(median!(1, 0, 0) == 0);
	dbg!(median!(1, 0, 1) == 1);
	dbg!(median!(1, 1, 0) == 1);
	dbg!(median!(1, 1, 1) == 1);

	dbg!(rotate_left!(0b00101101, 0) == 0b00101101);
	dbg!(rotate_left!(0b00101101, 1) == 0b01011010);
	dbg!(rotate_left!(0b00101101, 2) == 0b10110100);
	dbg!(rotate_left!(0b00101101, 3) == 0b01101001);
	dbg!(rotate_left!(0b00101101, 4) == 0b11010010);
	dbg!(rotate_left!(0b00101101, 5) == 0b10100101);
	dbg!(rotate_left!(0b00101101, 6) == 0b01001011);
	dbg!(rotate_left!(0b00101101, 7) == 0b10010110);
	dbg!(rotate_left!(0b00101101, 8) == 0b00101101);

	dbg!(rotate_right!(0b00101101, 0) == 0b00101101);
	dbg!(rotate_right!(0b00101101, 1) == 0b10010110);
	dbg!(rotate_right!(0b00101101, 2) == 0b01001011);
	dbg!(rotate_right!(0b00101101, 3) == 0b10100101);
	dbg!(rotate_right!(0b00101101, 4) == 0b11010010);
	dbg!(rotate_right!(0b00101101, 5) == 0b01101001);
	dbg!(rotate_right!(0b00101101, 6) == 0b10110100);
	dbg!(rotate_right!(0b00101101, 7) == 0b01011010);
	dbg!(rotate_right!(0b00101101, 8) == 0b00101101);
}

#[macro_export]
macro_rules! max {
	($a:expr, $($x:expr),*) => {
		{
			let mut max = $a;
		$(if $x > max { max = $x; })*
		max
		}
	};
}

#[macro_export]
macro_rules! min {
	($a:expr, $($x:expr),*) => {
		{
			let mut max = $a;
		$(if $x < max { max = $x; })*
		max
		}
	};
}

#[macro_export]
macro_rules! choice {
	($a:expr, $b:expr, $c:expr) => {
		($a & $b) ^ ((!$a) & $c)
	};
}

#[macro_export]
macro_rules! median {
	($a:expr, $b:expr, $c:expr) => {
		($a & $b) ^ ($a & $c) ^ ($b & $c)
	};
}

#[macro_export]
macro_rules! rotate_right {
	($a:expr, $b:expr) => {
		(($a >> $b % 8) | ($a << (8 - $b) % 8)) as u8
	};
}

#[macro_export]
macro_rules! rotate_left {
	($a:expr, $b:expr) => {
		(($a << $b % 8) | ($a >> (8 - $b) % 8)) as u8
	};
}
