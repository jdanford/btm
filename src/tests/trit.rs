use std::convert::TryFrom;

use trit::Trit;

#[test]
fn trit_into_i16() {
    let trit = Trit(Trit::BIN_POS);
    assert_eq!(1i16, trit.into());
}

#[test]
fn trit_from_i16() {
    let trit = Trit::try_from(1).unwrap();
    assert_eq!(Trit::BIN_POS, trit.0);
}

#[test]
fn trit_negate() {
    let trit_pos = Trit::try_from(1).unwrap();
    let trit_neg = Trit::try_from(-1).unwrap();
    assert_eq!(trit_pos, -trit_neg);
}
