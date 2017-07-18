use error::Result;
use tryte::Tryte;
use ternary::*;
use tests::constants::*;

#[test]
fn ternary_into_i64() {
    assert_eq!(WORD_MIN, Ternary::new(&mut TERNARY_MIN).into());
    assert_eq!(-1i64, Ternary::new(&mut TERNARY_NEG1).into());
    assert_eq!(0i64, Ternary::new(&mut TERNARY_0).into());
    assert_eq!(1i64, Ternary::new(&mut TERNARY_1).into());
    assert_eq!(WORD_MAX, Ternary::new(&mut TERNARY_MAX).into());
}

#[test]
fn ternary_read_int() {
    assert_eq!(Ok(vec_from_slice(&TERNARY_MIN)), trytes_from_i64(WORD_MIN));
    assert_eq!(Ok(vec_from_slice(&TERNARY_NEG1)), trytes_from_i64(-1));
    assert_eq!(Ok(vec_from_slice(&TERNARY_0)), trytes_from_i64(0));
    assert_eq!(Ok(vec_from_slice(&TERNARY_1)), trytes_from_i64(1));
    assert_eq!(Ok(vec_from_slice(&TERNARY_MAX)), trytes_from_i64(WORD_MAX));

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
    Ternary::new(&mut trytes).read_int(n)?;
    Ok(trytes)
}
