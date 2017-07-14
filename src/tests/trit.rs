use std::convert::TryFrom;

use trit::Trit;

#[test]
fn trit_into_i16() {
    assert_eq!(-1i16, Trit(Trit::BIN_NEG).into());
    assert_eq!(0i16, Trit(Trit::BIN_ZERO).into());
    assert_eq!(1i16, Trit(Trit::BIN_POS).into());
}

#[test]
fn trit_from_i16() {
    assert_eq!(Trit::BIN_NEG, Trit::try_from(-1).unwrap().0);
    assert_eq!(Trit::BIN_ZERO, Trit::try_from(0).unwrap().0);
    assert_eq!(Trit::BIN_POS, Trit::try_from(1).unwrap().0);
}

#[test]
fn trit_negate() {
    let trit_pos = Trit::try_from(1).unwrap();
    let trit_neg = Trit::try_from(-1).unwrap();
    assert_eq!(trit_pos, -trit_neg);
}
