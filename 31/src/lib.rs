pub fn weight_u8(byte: u8) -> u64 {
	1
}

pub fn weight_u64(word: u64) -> u64 {
	1
}

pub fn weight_bytes(bytes: Vec<u8>) -> u64 {
	1
}

pub fn weight_words(words: Vec<u64>) -> u64 {
	1
}

pub fn distance_u8(b: u8, c: u8) -> u64 {
	1
}

pub fn distance_u64(w: u64, x: u64) -> u64 {
	1
}

pub fn distance_bytes(bs: Vec<u8>, cs: Vec<u8>) -> u64 {
	1
}

pub fn distance_words(ws: Vec<u64>, xs: Vec<u64>) -> u64 {
	1
}
