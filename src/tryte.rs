pub const TRYTE_MASK: u16 = 0b0000000000000011;
pub const NEGATE_MASK: u16 = 0b0000101010101010;

pub fn get_trit(tryte: u16, i: usize) -> u16 {
	let shf = (i as u16) * 2;
	(tryte >> shf) & TRYTE_MASK
}

pub fn negate(tryte: u16) -> u16 {
    tryte ^ NEGATE_MASK
}

pub fn to_int(tryte: u16) -> i16 {
	let mut n = 0;

	// ???

	n
}
