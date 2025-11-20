type Heap<T> = Vec<T>;

/*
fn heapify<T>(mut h: Heap<T>, i: usize, gt: fn(&T, &T) -> bool) -> Heap<T> {
	todo!();
}

fn reheapify<T>(mut h: Heap<T>, i: usize, gt: fn(&T, &T) -> bool) -> Heap<T> {
	todo!();
}
*/

fn vec_to_heap<T>(xs: Vec<T>, gt: fn(&T, &T) -> bool) -> Heap<T> {
	let mut h: Heap<T> = Vec::<T>::from(xs);

	for n in 0..h.len() {
		let mut i: i64;
		let mut j: i64 = n as i64;

		while j > 0 {
			i = j;
			j = ((j + 1) / 2) - 1;

			if gt(&h[i as usize], &h[j as usize]) {
				h.swap(i as usize, j as usize);
			}
		}
	}

	h
}

fn heap_to_vec<T>(mut h: Heap<T>, gt: fn(&T, &T) -> bool) -> Vec<T> {
	for i in 1..h.len() {
		let mark: usize = 2usize.pow(usize::ilog2(i + 1) - 1) - 1;

		for j in mark..=(mark + usize::ilog2(i) as usize) {
			if gt(&h[i], &h[j]) {
				h.swap(i, j);
			}
		}
	}

	for n in 0..=usize::ilog2(h.len()) {
		for i in (2usize.pow(n) - 1)..=std::cmp::min(h.len() - 2, 2usize.pow(n + 1) - 2) {
			for j in (i + 1)..=std::cmp::min(h.len() - 1, 2usize.pow(n + 1) - 1) {
				if gt(&h[j], &h[i]) {
					h.swap(i, j);
				}
			}
		}
	}

	h
}

fn hsort<T>(xs: Vec<T>, gt: fn(&T, &T) -> bool) -> Vec<T> {
	heap_to_vec(vec_to_heap(xs, gt), gt)
}

fn main() {
	let xs: Vec<u64> = vec![
		657, 539, 631, 68, 446, 111, 470, 7, 443, 756, 738, 143, 972, 952, 744, 114, 877, 57, 537,
		724, 296, 82, 262, 800, 826, 201, 107, 55, 942, 440, 494, 608, 757, 522, 93, 959, 369, 971,
		81, 719, 544, 673, 402, 399, 306, 568, 337, 448, 380, 442, 224, 762, 928, 63, 728, 982,
		897, 847, 325, 478, 36, 692, 76, 889,
	]; // [2, 4, 6, 8, 5, 3, 7]

	fn f(x: &u64, y: &u64) -> bool {
		x > y
	}
	dbg!(&xs);
	let sorted: Vec<u64> = hsort(xs, f);
	dbg!(&sorted);
	return;
}
