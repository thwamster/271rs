fn main() {
	println!("{}", hash(include_str!("input.txt").trim()));
}

fn hash(input: &str) -> String {
	let mut H: [u32; 8] = [
		0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
		0x5be0cd19,
	];

	let K: [u32; 64] = [
		0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
		0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
		0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
		0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
		0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
		0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
		0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
		0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
		0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
		0xc67178f2,
	];

	let mut hash: String = String::new();
	let mut bytes: Vec<u8> = input.as_bytes().to_vec();
	let length: u64 = input.len() as u64;

	let padding: u64 = 64 - (length + 9) % 64;

	bytes.push(128);
	for _ in 0..padding {
		bytes.push(0);
	}
	bytes.extend_from_slice(&(length * 8).to_be_bytes());

	for c in 0..bytes.len() % 64 {
		let mut w: [u32; 64] = [0; 64];
		for i in 0..16 {
			w[i] = u32::from_be_bytes(bytes[c + i * 4..c + i * 4 + 4].try_into().unwrap());
		}

		for i in 16..64 {
			let s0 = rotate_right(w[i - 15], 7)
				^ rotate_right(w[i - 15], 18)
				^ rotate_right(w[i - 15], 3);
			let s1 = rotate_right(w[i - 2], 17)
				^ rotate_right(w[i - 2], 19)
				^ rotate_right(w[i - 2], 10);
			w[i] = w[i - 16] + s0 + w[i - 7] + s1;
		}

		let mut a = H[0];
		let mut b = H[1];
		let mut c = H[2];
		let mut d = H[3];
		let mut e = H[4];
		let mut f = H[5];
		let mut g = H[6];
		let mut h = H[7];

		for i in 0..64 {
			let S1 = rotate_right(e, 6) ^ rotate_right(e, 11) ^ rotate_right(e, 25);
			let ch = (e & f) ^ (!e & g);
			let temp1 = h + S1 + ch + K[i] + w[i];
			let S0 = rotate_right(a, 2) ^ rotate_right(a, 13) ^ rotate_right(a, 22);
			let maj = (a & b) ^ (a & c) ^ (b & c);
			let temp2 = S0 + maj;

			h = g;
			g = f;
			f = e;
			e = d + temp1;
			d = c;
			c = b;
			b = a;
			a = temp1 + temp2;
		}

		H[0] = H[0] + a;
		H[1] = H[1] + b;
		H[2] = H[2] + c;
		H[3] = H[3] + d;
		H[4] = H[4] + e;
		H[5] = H[5] + f;
		H[6] = H[6] + g;
		H[7] = H[7] + h;
	}

	for n in H {
		hash.push_str(&n.to_string());
		println!("{:b}", n);
	}

	hash
}

fn choice(u: u32, v: u32, w: u32) -> u32 {
	(u & v) ^ ((!u) & w)
}

fn median(u: u32, v: u32, w: u32) -> u32 {
	(u & v) ^ (u & w) ^ (v & w)
}

fn rotate_right(u: u32, v: u32) -> u32 {
	(u >> v % 32) | (u << (32 - v) % 32)
}
