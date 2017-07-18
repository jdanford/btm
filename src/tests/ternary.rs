use error::Result;
use trit;
use tryte::Tryte;
use ternary::*;
use tests::constants::*;

macro_rules! ternary {
    ($expr:expr) => { Ternary::new(&mut $expr) };
}

#[test]
fn ternary_into_i64() {
    assert_eq!(WORD_MIN, ternary!(TRYTE4_MIN).into());
    assert_eq!(-1i64, ternary!(TRYTE4_NEG1).into());
    assert_eq!(0i64, ternary!(TRYTE4_0).into());
    assert_eq!(1i64, ternary!(TRYTE4_1).into());
    assert_eq!(WORD_MAX, ternary!(TRYTE4_MAX).into());
}

#[test]
fn ternary_read_int() {
    assert_eq!(Ok(vec_from_slice(&TRYTE4_MIN)), trytes_from_i64(WORD_MIN));
    assert_eq!(Ok(vec_from_slice(&TRYTE4_NEG1)), trytes_from_i64(-1));
    assert_eq!(Ok(vec_from_slice(&TRYTE4_0)), trytes_from_i64(0));
    assert_eq!(Ok(vec_from_slice(&TRYTE4_1)), trytes_from_i64(1));
    assert_eq!(Ok(vec_from_slice(&TRYTE4_MAX)), trytes_from_i64(WORD_MAX));

    assert!(trytes_from_i64(i64::min_value()).is_err());
    assert!(trytes_from_i64(i64::max_value()).is_err());
}

fn vec_from_slice<T: Clone>(slice: &[T]) -> Vec<T> {
    let mut vec = Vec::new();
    vec.extend_from_slice(slice);
    vec
}

fn trytes_from_i64(n: i64) -> Result<Vec<Tryte>> {
    let mut trytes = vec![TRYTE_0, TRYTE_0, TRYTE_0, TRYTE_0];
    ternary!(trytes).read_int(n)?;
    Ok(trytes)
}

#[test]
fn ternary_write_bytes() {
    assert_eq!(vec_from_slice(&BYTES_MIN), get_bytes(&ternary!(TRYTE4_MIN)));
    assert_eq!(
        vec_from_slice(&BYTES_NEG1),
        get_bytes(&ternary!(TRYTE4_NEG1))
    );
    assert_eq!(vec_from_slice(&BYTES_0), get_bytes(&ternary!(TRYTE4_0)));
    assert_eq!(vec_from_slice(&BYTES_1), get_bytes(&ternary!(TRYTE4_1)));
    assert_eq!(vec_from_slice(&BYTES_MAX), get_bytes(&ternary!(TRYTE4_MAX)));
}

fn get_bytes<'a>(ternary: &'a Ternary<'a>) -> Vec<u8> {
    let mut bytes = vec![];
    ternary.write_bytes(&mut bytes).unwrap();
    bytes
}

#[test]
fn ternary_cmp() {
    assert_eq!(trit::ZERO, ternary!(TRYTE4_0).compare(&ternary!(TRYTE4_0)));
    assert_eq!(trit::NEG, ternary!(TRYTE4_0).compare(&ternary!(TRYTE4_MAX)));
    assert_eq!(trit::POS, ternary!(TRYTE4_0).compare(&ternary!(TRYTE4_MIN)));
    assert_eq!(trit::POS, ternary!(TRYTE4_MAX).compare(&ternary!(TRYTE4_0)));
    assert_eq!(
        trit::POS,
        ternary!(TRYTE4_MAX).compare(&ternary!(TRYTE4_MIN))
    );
    assert_eq!(
        trit::ZERO,
        ternary!(TRYTE4_MAX).compare(&ternary!(TRYTE4_MAX))
    );
    assert_eq!(trit::NEG, ternary!(TRYTE4_MIN).compare(&ternary!(TRYTE4_0)));
    assert_eq!(
        trit::NEG,
        ternary!(TRYTE4_MIN).compare(&ternary!(TRYTE4_MAX))
    );
    assert_eq!(
        trit::ZERO,
        ternary!(TRYTE4_MIN).compare(&ternary!(TRYTE4_MIN))
    );
}

#[test]
fn ternary_display_hytes() {
    assert_eq!(
        "0hmmmmmmmm",
        format!("{}", ternary!(TRYTE4_MIN).display_hytes())
    );
    assert_eq!(
        "0h0000000a",
        format!("{}", ternary!(TRYTE4_NEG1).display_hytes())
    );
    assert_eq!(
        "0h00000000",
        format!("{}", ternary!(TRYTE4_0).display_hytes())
    );
    assert_eq!(
        "0h0000000A",
        format!("{}", ternary!(TRYTE4_1).display_hytes())
    );
    assert_eq!(
        "0hMMMMMMMM",
        format!("{}", ternary!(TRYTE4_MAX).display_hytes())
    );
}

#[test]
fn ternary_display_trits() {
    assert_eq!(
        "0tTTTTTTTTTTTTTTTTTTTTTTTT",
        format!("{}", ternary!(TRYTE4_MIN).display_trits())
    );
    assert_eq!(
        "0t00000000000000000000000T",
        format!("{}", ternary!(TRYTE4_NEG1).display_trits())
    );
    assert_eq!(
        "0t000000000000000000000000",
        format!("{}", ternary!(TRYTE4_0).display_trits())
    );
    assert_eq!(
        "0t000000000000000000000001",
        format!("{}", ternary!(TRYTE4_1).display_trits())
    );
    assert_eq!(
        "0t111111111111111111111111",
        format!("{}", ternary!(TRYTE4_MAX).display_trits())
    );
}
