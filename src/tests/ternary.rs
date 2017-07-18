use error::Result;
use tryte::Tryte;
use ternary::*;
use tests::constants::*;

#[test]
fn ternary_into_i64() {
    assert_eq!(
        WORD_MIN,
        into_i64(vec![TRYTE_MIN, TRYTE_MIN, TRYTE_MIN, TRYTE_MIN])
    );
    assert_eq!(-1, into_i64(vec![TRYTE_NEG1, TRYTE_0, TRYTE_0, TRYTE_0]));
    assert_eq!(0, into_i64(vec![TRYTE_0, TRYTE_0, TRYTE_0, TRYTE_0]));
    assert_eq!(1, into_i64(vec![TRYTE_1, TRYTE_0, TRYTE_0, TRYTE_0]));
    assert_eq!(
        WORD_MAX,
        into_i64(vec![TRYTE_MAX, TRYTE_MAX, TRYTE_MAX, TRYTE_MAX])
    );
}

fn into_i64(mut trytes: Vec<Tryte>) -> i64 {
    let ternary = Ternary { trytes: &mut trytes };
    ternary.into()
}

#[test]
fn ternary_read_int() {
    assert_eq!(
        Ok(vec![TRYTE_MIN, TRYTE_MIN, TRYTE_MIN, TRYTE_MIN]),
        trytes_from_i64(WORD_MIN)
    );
    assert_eq!(
        Ok(vec![TRYTE_NEG1, TRYTE_0, TRYTE_0, TRYTE_0]),
        trytes_from_i64(-1)
    );
    assert_eq!(
        Ok(vec![TRYTE_0, TRYTE_0, TRYTE_0, TRYTE_0]),
        trytes_from_i64(0)
    );
    assert_eq!(
        Ok(vec![TRYTE_1, TRYTE_0, TRYTE_0, TRYTE_0]),
        trytes_from_i64(1)
    );
    assert_eq!(
        Ok(vec![TRYTE_MAX, TRYTE_MAX, TRYTE_MAX, TRYTE_MAX]),
        trytes_from_i64(WORD_MAX)
    );

    assert!(trytes_from_i64(i64::min_value()).is_err());
    assert!(trytes_from_i64(i64::max_value()).is_err());
}

fn trytes_from_i64(n: i64) -> Result<Vec<Tryte>> {
    let mut trytes = vec![TRYTE_0, TRYTE_0, TRYTE_0, TRYTE_0];

    {
        let mut ternary = Ternary { trytes: &mut trytes };
        ternary.read_int(n)?;
    }

    Ok(trytes)
}
