fn main() {
	let mut s = include_str!("input.txt").lines().next().unwrap().to_string();
	s.push_str("\n");

	println!("{}", hash(s));
}

#[allow(non_snake_case)]
fn hash(input : String) -> String {
	let mut H : [u64; 8] = [
		0x6A09E667F3BCC908,
		0xBB67AE8584CAA73B,
		0x3C6EF372FE94F82B,
		0xA54FF53A5F1D36F1,
		0x510E527FADE682D1,
		0x9B05688C2B3E6C1F,
		0x1F83D9ABFB41BD6B,
		0x5BE0CD19137E2179
	];

	let K : [u64; 80] = [
		0x428A2F98D728AE22,
		0x7137449123EF65CD,
		0xB5C0FBCFEC4D3B2F,
		0xE9B5DBA58189DBBC,
		0x3956C25BF348B538,
		0x59F111F1B605D019,
		0x923F82A4AF194F9B,
		0xAB1C5ED5DA6D8118,
		0xD807AA98A3030242,
		0x12835B0145706FBE,
		0x243185BE4EE4B28C,
		0x550C7DC3D5FFB4E2,
		0x72BE5D74F27B896F,
		0x80DEB1FE3B1696B1,
		0x9BDC06A725C71235,
		0xC19BF174CF692694,
		0xE49B69C19EF14AD2,
		0xEFBE4786384F25E3,
		0x0FC19DC68B8CD5B5,
		0x240CA1CC77AC9C65,
		0x2DE92C6F592B0275,
		0x4A7484AA6EA6E483,
		0x5CB0A9DCBD41FBD4,
		0x76F988DA831153B5,
		0x983E5152EE66DFAB,
		0xA831C66D2DB43210,
		0xB00327C898FB213F,
		0xBF597FC7BEEF0EE4,
		0xC6E00BF33DA88FC2,
		0xD5A79147930AA725,
		0x06CA6351E003826F,
		0x142929670A0E6E70,
		0x27B70A8546D22FFC,
		0x2E1B21385C26C926,
		0x4D2C6DFC5AC42AED,
		0x53380D139D95B3DF,
		0x650A73548BAF63DE,
		0x766A0ABB3C77B2A8,
		0x81C2C92E47EDAEE6,
		0x92722C851482353B,
		0xA2BFE8A14CF10364,
		0xA81A664BBC423001,
		0xC24B8B70D0F89791,
		0xC76C51A30654BE30,
		0xD192E819D6EF5218,
		0xD69906245565A910,
		0xF40E35855771202A,
		0x106AA07032BBD1B8,
		0x19A4C116B8D2D0C8,
		0x1E376C085141AB53,
		0x2748774CDF8EEB99,
		0x34B0BCB5E19B48A8,
		0x391C0CB3C5C95A63,
		0x4ED8AA4AE3418ACB,
		0x5B9CCA4F7763E373,
		0x682E6FF3D6B2B8A3,
		0x748F82EE5DEFB2FC,
		0x78A5636F43172F60,
		0x84C87814A1F0AB72,
		0x8CC702081A6439EC,
		0x90BEFFFA23631E28,
		0xA4506CEBDE82BDE9,
		0xBEF9A3F7B2C67915,
		0xC67178F2E372532B,
		0xCA273ECEEA26619C,
		0xD186B8C721C0C207,
		0xEADA7DD6CDE0EB1E,
		0xF57D4F7FEE6ED178,
		0x06F067AA72176FBA,
		0x0A637DC5A2C898A6,
		0x113F9804BEF90DAE,
		0x1B710B35131C471B,
		0x28DB77F523047D84,
		0x32CAAB7B40C72493,
		0x3C9EBE0A15C9BEBC,
		0x431D67C49C100D4C,
		0x4CC5D4BECB3E42B6,
		0x597F299CFC657E2A,
		0x5FCB6FAB3AD6FAEC,
		0x6C44198C4A475817
	];

	let mut hash : String = String::new();
	let mut bytes : Vec<u8> = input.as_bytes().to_vec();
	let length : u64 = input.len() as u64;

	let padding : u64 = 128 - (length + 9) % 128;

	bytes.push(128);
	for _ in 0..padding {
		bytes.push(0);
	}
	bytes.extend_from_slice(&(length * 8).to_be_bytes());

	for c in 0..bytes.len() >> 7 {
		let mut w : [u64; 80] = [0; 80];
		for i in 0..16 {
			w[i] =
				u64::from_be_bytes(bytes[c * 128 + i * 8..c * 128 + i * 8 + 8].try_into().unwrap());
		}

		for i in 16..80 {
			let s0 : u64 =
				rotate_right(w[i - 15], 1) ^ rotate_right(w[i - 15], 8) ^ (w[i - 15] >> 7);
			let s1 : u64 =
				rotate_right(w[i - 2], 19) ^ rotate_right(w[i - 2], 61) ^ (w[i - 2] >> 6);
			w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
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
			let S1 : u64 = rotate_right(e, 14) ^ rotate_right(e, 18) ^ rotate_right(e, 41);
			let ch : u64 = (e & f) ^ ((!e) & g);
			let temp1 : u64 =
				h.wrapping_add(S1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
			let S0 : u64 = rotate_right(a, 28) ^ rotate_right(a, 34) ^ rotate_right(a, 39);
			let maj : u64 = (a & b) ^ (a & c) ^ (b & c);
			let temp2 : u64 = S0.wrapping_add(maj);

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

fn rotate_right(u : u64, v : u64) -> u64 { (u >> v % 64) | (u << (64 - v) % 64) }
