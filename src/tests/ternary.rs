use std::io::Cursor;

use error::Result;
use trit;
use tryte::Tryte;
use ternary::*;

use tests::constants::*;

fn clone_slice<T: Clone>(slice: &[T]) -> Vec<T> {
    let mut vec = Vec::new();
    vec.extend_from_slice(slice);
    vec
}

fn with_cloned_trytes<F>(trytes: &[Tryte], mut f: F) -> Vec<Tryte>
where
    F: FnMut(&mut [Tryte]),
{
    let mut trytes = clone_slice(trytes);
    f(&mut trytes[..]);
    trytes
}

fn with_cloned_trytes2<F>(trytes1: &[Tryte], trytes2: &[Tryte], mut f: F) -> Vec<Tryte>
where
    F: FnMut(&mut [Tryte], &[Tryte]),
{
    let mut trytes1 = clone_slice(trytes1);
    f(&mut trytes1[..], &trytes2);
    trytes1
}

fn with_cloned_trytes3<F>(
    trytes1: &[Tryte],
    trytes2: &[Tryte],
    trytes3: &[Tryte],
    mut f: F,
) -> Vec<Tryte>
where
    F: FnMut(&mut [Tryte], &[Tryte], &[Tryte]),
{
    let mut trytes1 = clone_slice(trytes1);
    f(&mut trytes1[..], &trytes2, &trytes3);
    trytes1
}

fn try_with_cloned_trytes<F>(trytes: &[Tryte], mut f: F) -> Result<Vec<Tryte>>
where
    F: FnMut(&mut [Tryte]) -> Result<()>,
{
    let mut trytes = clone_slice(trytes);
    f(&mut trytes[..])?;
    Ok(trytes)
}

fn try_with_cloned_trytes2<F>(trytes1: &[Tryte], trytes2: &[Tryte], mut f: F) -> Result<Vec<Tryte>>
where
    F: FnMut(&mut [Tryte], &[Tryte]) -> Result<()>,
{
    let mut trytes1 = clone_slice(trytes1);
    f(&mut trytes1[..], &trytes2)?;
    Ok(trytes1)
}

#[test]
fn ternary_into_i64() {
    assert_eq!(WORD_MIN, TRYTE4_MIN.into_i64());
    assert_eq!(-1i64, TRYTE4_NEG1.into_i64());
    assert_eq!(0i64, TRYTE4_0.into_i64());
    assert_eq!(1i64, TRYTE4_1.into_i64());
    assert_eq!(WORD_MAX, TRYTE4_MAX.into_i64());
}

#[test]
fn ternary_read_i64() {
    assert_eq!(&TRYTE4_MIN, &tryte4_from_int(WORD_MIN).unwrap()[..]);
    assert_eq!(&TRYTE4_NEG1, &tryte4_from_int(-1).unwrap()[..]);
    assert_eq!(&TRYTE4_0, &tryte4_from_int(0).unwrap()[..]);
    assert_eq!(&TRYTE4_1, &tryte4_from_int(1).unwrap()[..]);
    assert_eq!(&TRYTE4_MAX, &tryte4_from_int(WORD_MAX).unwrap()[..]);

    assert!(tryte4_from_int(i64::min_value()).is_err());
    assert!(tryte4_from_int(i64::max_value()).is_err());
}

fn tryte4_from_int(n: i64) -> Result<Vec<Tryte>> {
    try_with_cloned_trytes(&TRYTE4_0, |ref mut ternary| ternary.read_i64(n))
}

#[test]
fn ternary_read_bytes() {
    assert_eq!(&TRYTE4_MIN, &tryte4_from_bytes(&BYTES_MIN).unwrap()[..]);
    assert_eq!(&TRYTE4_NEG1, &tryte4_from_bytes(&BYTES_NEG1).unwrap()[..]);
    assert_eq!(&TRYTE4_0, &tryte4_from_bytes(&BYTES_0).unwrap()[..]);
    assert_eq!(&TRYTE4_1, &tryte4_from_bytes(&BYTES_1).unwrap()[..]);
    assert_eq!(&TRYTE4_MAX, &tryte4_from_bytes(&BYTES_MAX).unwrap()[..]);
}

fn tryte4_from_bytes(bytes: &[u8]) -> Result<Vec<Tryte>> {
    try_with_cloned_trytes(&TRYTE4_0, |ref mut ternary| {
        ternary.read_bytes(&mut Cursor::new(bytes))
    })
}

#[test]
fn ternary_write_bytes() {
    assert_eq!(&BYTES_MIN, &get_bytes(&TRYTE4_MIN[..])[..]);
    assert_eq!(&BYTES_NEG1, &get_bytes(&TRYTE4_NEG1[..])[..]);
    assert_eq!(&BYTES_0, &get_bytes(&TRYTE4_0[..])[..]);
    assert_eq!(&BYTES_1, &get_bytes(&TRYTE4_1[..])[..]);
    assert_eq!(&BYTES_MAX, &get_bytes(&TRYTE4_MAX[..])[..]);
}

fn get_bytes(trytes: &[Tryte]) -> Vec<u8> {
    let mut bytes = vec![];
    trytes.write_bytes(&mut bytes).unwrap();
    bytes
}

#[test]
fn ternary_read_hytes() {
    assert_eq!(&TRYTE4_MIN, &tryte4_from_hyte_str("mmmmmmmm").unwrap()[..]);
    assert_eq!(&TRYTE4_NEG1, &tryte4_from_hyte_str("0000000a").unwrap()[..]);
    assert_eq!(&TRYTE4_0, &tryte4_from_hyte_str("00000000").unwrap()[..]);
    assert_eq!(&TRYTE4_1, &tryte4_from_hyte_str("0000000A").unwrap()[..]);
    assert_eq!(&TRYTE4_MAX, &tryte4_from_hyte_str("MMMMMMMM").unwrap()[..]);
}

fn tryte4_from_hyte_str(s: &str) -> Result<Vec<Tryte>> {
    try_with_cloned_trytes(&TRYTE4_0, |ref mut ternary| ternary.read_hytes(s))
}

#[test]
fn ternary_display_hytes() {
    assert_eq!("mmmmmmmm", get_hyte_str(&TRYTE4_MIN[..]));
    assert_eq!("0000000a", get_hyte_str(&TRYTE4_NEG1[..]));
    assert_eq!("00000000", get_hyte_str(&TRYTE4_0[..]));
    assert_eq!("0000000A", get_hyte_str(&TRYTE4_1[..]));
    assert_eq!("MMMMMMMM", get_hyte_str(&TRYTE4_MAX[..]));
}

fn get_hyte_str(trytes: &[Tryte]) -> String {
    let mut s = String::new();
    trytes.write_hytes(&mut s).unwrap();
    s
}

#[test]
fn ternary_read_trits() {
    assert_eq!(
        &TRYTE4_MIN,
        &tryte4_from_trit_str("TTTTTTTTTTTTTTTTTTTTTTTT").unwrap()[..]
    );
    assert_eq!(
        &TRYTE4_NEG1,
        &tryte4_from_trit_str("00000000000000000000000T").unwrap()[..]
    );
    assert_eq!(
        &TRYTE4_0,
        &tryte4_from_trit_str("000000000000000000000000").unwrap()[..]
    );
    assert_eq!(
        &TRYTE4_1,
        &tryte4_from_trit_str("000000000000000000000001").unwrap()[..]
    );
    assert_eq!(
        &TRYTE4_MAX,
        &tryte4_from_trit_str("111111111111111111111111").unwrap()[..]
    );
}

fn tryte4_from_trit_str(s: &str) -> Result<Vec<Tryte>> {
    try_with_cloned_trytes(&TRYTE4_0, |ref mut ternary| ternary.read_trits(s))
}

#[test]
fn ternary_display_trits() {
    assert_eq!("TTTTTTTTTTTTTTTTTTTTTTTT", get_trit_str(&TRYTE4_MIN[..]));
    assert_eq!("00000000000000000000000T", get_trit_str(&TRYTE4_NEG1[..]));
    assert_eq!("000000000000000000000000", get_trit_str(&TRYTE4_0[..]));
    assert_eq!("000000000000000000000001", get_trit_str(&TRYTE4_1[..]));
    assert_eq!("111111111111111111111111", get_trit_str(&TRYTE4_MAX[..]));
}

fn get_trit_str(trytes: &[Tryte]) -> String {
    let mut s = String::new();
    trytes.write_trits(&mut s).unwrap();
    s
}

#[test]
fn ternary_cmp() {
    assert_eq!(trit::ZERO, TRYTE4_0.compare(&TRYTE4_0));
    assert_eq!(trit::NEG, TRYTE4_0.compare(&TRYTE4_MAX));
    assert_eq!(trit::POS, TRYTE4_0.compare(&TRYTE4_MIN));
    assert_eq!(trit::POS, TRYTE4_MAX.compare(&TRYTE4_0));
    assert_eq!(trit::POS, TRYTE4_MAX.compare(&TRYTE4_MIN));
    assert_eq!(trit::ZERO, TRYTE4_MAX.compare(&TRYTE4_MAX));
    assert_eq!(trit::NEG, TRYTE4_MIN.compare(&TRYTE4_0));
    assert_eq!(trit::NEG, TRYTE4_MIN.compare(&TRYTE4_MAX));
    assert_eq!(trit::ZERO, TRYTE4_MIN.compare(&TRYTE4_MIN));
}

#[test]
fn ternary_negate() {
    assert_eq!(&TRYTE4_MIN, &tryte4_negate(&TRYTE4_MAX)[..]);
    assert_eq!(&TRYTE4_NEG1, &tryte4_negate(&TRYTE4_1)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_negate(&TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_1, &tryte4_negate(&TRYTE4_NEG1)[..]);
    assert_eq!(&TRYTE4_MAX, &tryte4_negate(&TRYTE4_MIN)[..]);
}

fn tryte4_negate(trytes: &[Tryte]) -> Vec<Tryte> {
    with_cloned_trytes(trytes, |ref mut ternary| { ternary.negate(); })
}

#[test]
fn ternary_and() {
    assert_eq!(&TRYTE4_0, &tryte4_and(&TRYTE4_0, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_and(&TRYTE4_0, &TRYTE4_MAX)[..]);
    assert_eq!(&TRYTE4_MIN, &tryte4_and(&TRYTE4_0, &TRYTE4_MIN)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_and(&TRYTE4_MAX, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_MAX, &tryte4_and(&TRYTE4_MAX, &TRYTE4_MAX)[..]);
    assert_eq!(&TRYTE4_MIN, &tryte4_and(&TRYTE4_MAX, &TRYTE4_MIN)[..]);
    assert_eq!(&TRYTE4_MIN, &tryte4_and(&TRYTE4_MIN, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_MIN, &tryte4_and(&TRYTE4_MIN, &TRYTE4_MAX)[..]);
    assert_eq!(&TRYTE4_MIN, &tryte4_and(&TRYTE4_MIN, &TRYTE4_MIN)[..]);
}

fn tryte4_and(trytes1: &[Tryte], trytes2: &[Tryte]) -> Vec<Tryte> {
    with_cloned_trytes2(trytes1, trytes2, |ref mut lhs, ref rhs| { lhs.and(rhs); })
}

#[test]
fn ternary_or() {
    assert_eq!(&TRYTE4_0, &tryte4_or(&TRYTE4_0, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_MAX, &tryte4_or(&TRYTE4_0, &TRYTE4_MAX)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_or(&TRYTE4_0, &TRYTE4_MIN)[..]);
    assert_eq!(&TRYTE4_MAX, &tryte4_or(&TRYTE4_MAX, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_MAX, &tryte4_or(&TRYTE4_MAX, &TRYTE4_MAX)[..]);
    assert_eq!(&TRYTE4_MAX, &tryte4_or(&TRYTE4_MAX, &TRYTE4_MIN)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_or(&TRYTE4_MIN, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_MAX, &tryte4_or(&TRYTE4_MIN, &TRYTE4_MAX)[..]);
    assert_eq!(&TRYTE4_MIN, &tryte4_or(&TRYTE4_MIN, &TRYTE4_MIN)[..]);
}

fn tryte4_or(trytes1: &[Tryte], trytes2: &[Tryte]) -> Vec<Tryte> {
    with_cloned_trytes2(trytes1, trytes2, |ref mut lhs, ref rhs| { lhs.or(rhs); })
}

#[test]
fn ternary_tcmp() {
    assert_eq!(&TRYTE4_MIN, &tryte4_tcmp(&TRYTE4_MIN, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_MAX, &tryte4_tcmp(&TRYTE4_MAX, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_NEG1, &tryte4_tcmp(&TRYTE4_NEG1, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_tcmp(&TRYTE4_0, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_1, &tryte4_tcmp(&TRYTE4_1, &TRYTE4_0)[..]);

    assert_eq!(&TRYTE4_MAX, &tryte4_tcmp(&TRYTE4_0, &TRYTE4_MIN)[..]);
    assert_eq!(&TRYTE4_MIN, &tryte4_tcmp(&TRYTE4_0, &TRYTE4_MAX)[..]);
    assert_eq!(&TRYTE4_1, &tryte4_tcmp(&TRYTE4_0, &TRYTE4_NEG1)[..]);
    assert_eq!(&TRYTE4_NEG1, &tryte4_tcmp(&TRYTE4_0, &TRYTE4_1)[..]);

    assert_eq!(&TRYTE4_0, &tryte4_tcmp(&TRYTE4_MIN, &TRYTE4_MIN)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_tcmp(&TRYTE4_MAX, &TRYTE4_MAX)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_tcmp(&TRYTE4_NEG1, &TRYTE4_NEG1)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_tcmp(&TRYTE4_1, &TRYTE4_1)[..]);
}

fn tryte4_tcmp(trytes1: &[Tryte], trytes2: &[Tryte]) -> Vec<Tryte> {
    with_cloned_trytes2(trytes1, trytes2, |ref mut lhs, ref rhs| { lhs.tcmp(rhs); })
}

#[test]
fn ternary_tmul() {
    assert_eq!(&TRYTE4_0, &tryte4_tmul(&TRYTE4_MIN, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_tmul(&TRYTE4_MAX, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_tmul(&TRYTE4_NEG1, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_tmul(&TRYTE4_0, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_tmul(&TRYTE4_1, &TRYTE4_0)[..]);

    assert_eq!(&TRYTE4_MIN, &tryte4_tmul(&TRYTE4_MIN, &TRYTE4_MAX)[..]);
    assert_eq!(&TRYTE4_MAX, &tryte4_tmul(&TRYTE4_MAX, &TRYTE4_MAX)[..]);
    assert_eq!(&TRYTE4_NEG1, &tryte4_tmul(&TRYTE4_NEG1, &TRYTE4_MAX)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_tmul(&TRYTE4_0, &TRYTE4_MAX)[..]);
    assert_eq!(&TRYTE4_1, &tryte4_tmul(&TRYTE4_1, &TRYTE4_MAX)[..]);

    assert_eq!(&TRYTE4_MAX, &tryte4_tmul(&TRYTE4_MIN, &TRYTE4_MIN)[..]);
    assert_eq!(&TRYTE4_MIN, &tryte4_tmul(&TRYTE4_MAX, &TRYTE4_MIN)[..]);
    assert_eq!(&TRYTE4_1, &tryte4_tmul(&TRYTE4_NEG1, &TRYTE4_MIN)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_tmul(&TRYTE4_0, &TRYTE4_MIN)[..]);
    assert_eq!(&TRYTE4_NEG1, &tryte4_tmul(&TRYTE4_1, &TRYTE4_MIN)[..]);
}

fn tryte4_tmul(trytes1: &[Tryte], trytes2: &[Tryte]) -> Vec<Tryte> {
    with_cloned_trytes2(trytes1, trytes2, |ref mut lhs, ref rhs| { lhs.tmul(rhs); })
}

#[test]
fn ternary_add() {
    assert_eq!(&TRYTE4_0, &tryte4_add(&TRYTE4_1, &TRYTE4_NEG1)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_add(&TRYTE4_MAX, &TRYTE4_MIN)[..]);

    assert_eq!(&TRYTE4_MIN, &tryte4_add(&TRYTE4_MIN, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_NEG1, &tryte4_add(&TRYTE4_NEG1, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_add(&TRYTE4_0, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_1, &tryte4_add(&TRYTE4_1, &TRYTE4_0)[..]);
    assert_eq!(&TRYTE4_MAX, &tryte4_add(&TRYTE4_MAX, &TRYTE4_0)[..]);

    assert_eq!(&TRYTE4_MIN, &tryte4_add(&TRYTE4_0, &TRYTE4_MIN)[..]);
    assert_eq!(&TRYTE4_NEG1, &tryte4_add(&TRYTE4_0, &TRYTE4_NEG1)[..]);
    assert_eq!(&TRYTE4_1, &tryte4_add(&TRYTE4_0, &TRYTE4_1)[..]);
    assert_eq!(&TRYTE4_MAX, &tryte4_add(&TRYTE4_0, &TRYTE4_MAX)[..]);
}

fn tryte4_add(trytes1: &[Tryte], trytes2: &[Tryte]) -> Vec<Tryte> {
    with_cloned_trytes2(trytes1, trytes2, |ref mut lhs, ref rhs| {
        let _ = lhs.add(rhs, trit::ZERO);
    })
}

#[test]
fn ternary_multiply() {
    assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_NEG4096, &TRYTE2_0)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_NEG1, &TRYTE2_0)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_0, &TRYTE2_0)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_1, &TRYTE2_0)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_4096, &TRYTE2_0)[..]);

    assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_0, &TRYTE2_NEG4096)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_0, &TRYTE2_NEG1)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_0, &TRYTE2_1)[..]);
    assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_0, &TRYTE2_4096)[..]);

    assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_NEG4096, &TRYTE2_1)[..]);
    assert_eq!(&TRYTE4_NEG1, &tryte4_mul(&TRYTE2_NEG1, &TRYTE2_1)[..]);
    assert_eq!(&TRYTE4_1, &tryte4_mul(&TRYTE2_1, &TRYTE2_1)[..]);
    assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_4096, &TRYTE2_1)[..]);

    assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_1, &TRYTE2_NEG4096)[..]);
    assert_eq!(&TRYTE4_NEG1, &tryte4_mul(&TRYTE2_1, &TRYTE2_NEG1)[..]);
    assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_1, &TRYTE2_4096)[..]);

    assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_NEG4096, &TRYTE2_NEG1)[..]);
    assert_eq!(&TRYTE4_1, &tryte4_mul(&TRYTE2_NEG1, &TRYTE2_NEG1)[..]);
    assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_4096, &TRYTE2_NEG1)[..]);

    assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_NEG1, &TRYTE2_NEG4096)[..]);
    assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_NEG1, &TRYTE2_4096)[..]);

    assert_eq!(&TRYTE4_64, &tryte4_mul(&TRYTE2_8, &TRYTE2_8)[..]);
    assert_eq!(&TRYTE4_64, &tryte4_mul(&TRYTE2_NEG8, &TRYTE2_NEG8)[..]);
    assert_eq!(&TRYTE4_NEG64, &tryte4_mul(&TRYTE2_8, &TRYTE2_NEG8)[..]);
    assert_eq!(&TRYTE4_NEG64, &tryte4_mul(&TRYTE2_NEG8, &TRYTE2_8)[..]);

    assert_eq!(&TRYTE4_81, &tryte4_mul(&TRYTE2_9, &TRYTE2_9)[..]);
    assert_eq!(&TRYTE4_81, &tryte4_mul(&TRYTE2_NEG9, &TRYTE2_NEG9)[..]);
    assert_eq!(&TRYTE4_NEG81, &tryte4_mul(&TRYTE2_9, &TRYTE2_NEG9)[..]);
    assert_eq!(&TRYTE4_NEG81, &tryte4_mul(&TRYTE2_NEG9, &TRYTE2_9)[..]);

    assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_8, &TRYTE2_512)[..]);
    assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_NEG8, &TRYTE2_NEG512)[..]);
    assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_8, &TRYTE2_NEG512)[..]);
    assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_NEG8, &TRYTE2_512)[..]);

    assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_512, &TRYTE2_8)[..]);
    assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_NEG512, &TRYTE2_NEG8)[..]);
    assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_512, &TRYTE2_NEG8)[..]);
    assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_NEG512, &TRYTE2_8)[..]);

    assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_16, &TRYTE2_256)[..]);
    assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_NEG16, &TRYTE2_NEG256)[..]);
    assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_16, &TRYTE2_NEG256)[..]);
    assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_NEG16, &TRYTE2_256)[..]);

    assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_256, &TRYTE2_16)[..]);
    assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_NEG256, &TRYTE2_NEG16)[..]);
    assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_256, &TRYTE2_NEG16)[..]);
    assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_NEG256, &TRYTE2_16)[..]);

    assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_64, &TRYTE2_64)[..]);
    assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_NEG64, &TRYTE2_NEG64)[..]);
    assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_64, &TRYTE2_NEG64)[..]);
    assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_NEG64, &TRYTE2_64)[..]);
}

fn tryte4_mul(trytes1: &[Tryte], trytes2: &[Tryte]) -> Vec<Tryte> {
    with_cloned_trytes3(
        &TRYTE4_0,
        trytes1,
        trytes2,
        |ref mut dest, ref lhs, ref rhs| { dest.multiply(lhs, rhs); },
    )
}
