type Heap<T> = Vec<T>;

fn vec_to_heap<T>(xs : Vec<T>, gt : fn(&T, &T) -> bool) -> Heap<T> {
	let mut h : Heap<T> = Vec::<T>::from(xs);

	for n in (0..h.len()).rev() {
		let mut i : usize = n;
		let mut next : (usize, usize);
		let mut max : usize;

		while {
			next = (i * 2 + 1, i * 2 + 2);
			max = if next.1 < h.len() && gt(&h[next.1], &h[next.0]) { next.1 } else { next.0 };

			next.0 < h.len() && gt(&h[max], &h[i])
		} {
			h.swap(i, max);
			i = max;
		}
	}

	h
}

fn heap_to_vec<T>(mut h : Heap<T>, gt : fn(&T, &T) -> bool) -> Vec<T> {
	let mut n : usize = h.len();
	let mut i : usize;
	let mut prev : usize;
	let mut next : (usize, usize);
	let mut max : usize;

	while n > 0 {
		i = 0;

		while {
			next = (i * 2 + 1, i * 2 + 2);
			max = if next.1 < n && gt(&h[next.1], &h[next.0]) { next.1 } else { next.0 };

			next.0 < n && gt(&h[i], &h[max])
		} {
			h.swap(i, max);
			i = max;
		}

		h.swap(i, n - 1);
		n -= 1;

		while {
			prev = (((i + 1) / 2) as i32 - 1) as usize;
			i > 0 && i < n && gt(&h[i], &h[prev])
		} {
			h.swap(i, prev);
			i = prev;
		}
	}

	h.reverse();
	h
}

fn hsort<T>(xs : Vec<T>, gt : fn(&T, &T) -> bool) -> Vec<T> { heap_to_vec(vec_to_heap(xs, gt), gt) }

fn main() {
	let xs : Vec<u64> = vec![2, 4, 6, 8, 5, 3, 7];

	fn f(x : &u64, y : &u64) -> bool { x > y }
	dbg!(&xs);
	let sorted : Vec<u64> = hsort(xs, f);
	dbg!(&sorted);
	return;
}
