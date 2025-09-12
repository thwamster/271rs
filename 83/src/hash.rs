#[allow(non_snake_case)]

pub fn hash(input: &str) -> String {
	let mut H: [u64; 8] = [
		0x6a09e667f3bcc908,
		0xbb67ae8584caa73b,
		0x3c6ef372fe94f82b,
		0xa54ff53a5f1d36f1,
		0x510e527fade682d1,
		0x9b05688c2b3e6c1f,
		0x1f83d9abfb41bd6b,
		0x5be0cd19137e2179,
	];

	let K: [u64; 80] = [
		0x428a2f98d728ae22,
		0x7137449123ef65cd,
		0xb5c0fbcfec4d3b2f,
		0xe9b5dba58189dbbc,
		0x3956c25bf348b538,
		0x59f111f1b605d019,
		0x923f82a4af194f9b,
		0xab1c5ed5da6d8118,
		0xd807aa98a3030242,
		0x12835b0145706fbe,
		0x243185be4ee4b28c,
		0x550c7dc3d5ffb4e2,
		0x72be5d74f27b896f,
		0x80deb1fe3b1696b1,
		0x9bdc06a725c71235,
		0xc19bf174cf692694,
		0xe49b69c19ef14ad2,
		0xefbe4786384f25e3,
		0x0fc19dc68b8cd5b5,
		0x240ca1cc77ac9c65,
		0x2de92c6f592b0275,
		0x4a7484aa6ea6e483,
		0x5cb0a9dcbd41fbd4,
		0x76f988da831153b5,
		0x983e5152ee66dfab,
		0xa831c66d2db43210,
		0xb00327c898fb213f,
		0xbf597fc7beef0ee4,
		0xc6e00bf33da88fc2,
		0xd5a79147930aa725,
		0x06ca6351e003826f,
		0x142929670a0e6e70,
		0x27b70a8546d22ffc,
		0x2e1b21385c26c926,
		0x4d2c6dfc5ac42aed,
		0x53380d139d95b3df,
		0x650a73548baf63de,
		0x766a0abb3c77b2a8,
		0x81c2c92e47edaee6,
		0x92722c851482353b,
		0xa2bfe8a14cf10364,
		0xa81a664bbc423001,
		0xc24b8b70d0f89791,
		0xc76c51a30654be30,
		0xd192e819d6ef5218,
		0xd69906245565a910,
		0xf40e35855771202a,
		0x106aa07032bbd1b8,
		0x19a4c116b8d2d0c8,
		0x1e376c085141ab53,
		0x2748774cdf8eeb99,
		0x34b0bcb5e19b48a8,
		0x391c0cb3c5c95a63,
		0x4ed8aa4ae3418acb,
		0x5b9cca4f7763e373,
		0x682e6ff3d6b2b8a3,
		0x748f82ee5defb2fc,
		0x78a5636f43172f60,
		0x84c87814a1f0ab72,
		0x8cc702081a6439ec,
		0x90befffa23631e28,
		0xa4506cebde82bde9,
		0xbef9a3f7b2c67915,
		0xc67178f2e372532b,
		0xca273eceea26619c,
		0xd186b8c721c0c207,
		0xeada7dd6cde0eb1e,
		0xf57d4f7fee6ed178,
		0x06f067aa72176fba,
		0x0a637dc5a2c898a6,
		0x113f9804bef90dae,
		0x1b710b35131c471b,
		0x28db77f523047d84,
		0x32caab7b40c72493,
		0x3c9ebe0a15c9bebc,
		0x431d67c49c100d4c,
		0x4cc5d4becb3e42b6,
		0x597f299cfc657e2a,
		0x5fcb6fab3ad6faec,
		0x6c44198c4a475817,
	];

	let mut hash: String = String::new();
	let mut bytes: Vec<u8> = input.as_bytes().to_vec();
	let length: u64 = input.len() as u64;

	let padding: u64 = 128 - (length + 9) % 128;

	bytes.push(128);
	for _ in 0..padding {
		bytes.push(0);
	}
	bytes.extend_from_slice(&(length * 8).to_be_bytes());

	for c in 0..bytes.len() >> 7 {
		let mut w: [u64; 80] = [0; 80];
		for i in 0..16 {
			w[i] = u64::from_be_bytes(
				bytes[c * 128 + i * 8..c * 128 + i * 8 + 8]
					.try_into()
					.unwrap(),
			);
		}

		for i in 16..80 {
			let s0: u64 =
				rotate_right(w[i - 15], 1) ^ rotate_right(w[i - 15], 8) ^ (w[i - 15] >> 7);
			let s1: u64 = rotate_right(w[i - 2], 19) ^ rotate_right(w[i - 2], 61) ^ (w[i - 2] >> 6);
			w[i] = w[i - 16]
				.wrapping_add(s0)
				.wrapping_add(w[i - 7])
				.wrapping_add(s1);
		}

		let mut a = H[0];
		let mut b = H[1];
		let mut c = H[2];
		let mut d = H[3];
		let mut e = H[4];
		let mut f = H[5];
		let mut g = H[6];
		let mut h = H[7];

		for i in 0..80 {
			let S1: u64 = rotate_right(e, 14) ^ rotate_right(e, 18) ^ rotate_right(e, 41);
			let ch: u64 = (e & f) ^ ((!e) & g);
			let temp1: u64 = h
				.wrapping_add(S1)
				.wrapping_add(ch)
				.wrapping_add(K[i])
				.wrapping_add(w[i]);
			let S0: u64 = rotate_right(a, 28) ^ rotate_right(a, 34) ^ rotate_right(a, 39);
			let maj: u64 = (a & b) ^ (a & c) ^ (b & c);
			let temp2: u64 = S0.wrapping_add(maj);

			h = g;
			g = f;
			f = e;
			e = d.wrapping_add(temp1);
			d = c;
			c = b;
			b = a;
			a = temp1.wrapping_add(temp2);
		}

		H[0] = H[0].wrapping_add(a);
		H[1] = H[1].wrapping_add(b);
		H[2] = H[2].wrapping_add(c);
		H[3] = H[3].wrapping_add(d);
		H[4] = H[4].wrapping_add(e);
		H[5] = H[5].wrapping_add(f);
		H[6] = H[6].wrapping_add(g);
		H[7] = H[7].wrapping_add(h);
	}

	for n in H {
		hash.push_str(format!("{:016x}", n).as_str());
	}

	hash
}

fn rotate_right(u: u64, v: u64) -> u64 {
	(u >> v % 64) | (u << (64 - v) % 64)
}
