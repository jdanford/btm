use std::convert::TryFrom;

use ternary::trit::*;

#[test]
fn trit_into_i16() {
    assert_eq!(-1i16, Trit(BIN_NEG).into());
    assert_eq!(0i16, Trit(BIN_ZERO).into());
    assert_eq!(1i16, Trit(BIN_POS).into());
}

#[test]
fn trit_from_i16() {
    assert_eq!(Ok(Trit(BIN_NEG)), Trit::try_from(-1));
    assert_eq!(Ok(Trit(BIN_ZERO)), Trit::try_from(0));
    assert_eq!(Ok(Trit(BIN_POS)), Trit::try_from(1));

    assert!(Trit::try_from(-2).is_err());
    assert!(Trit::try_from(2).is_err());
}

#[test]
fn trit_into_char() {
    assert_eq!('T', Trit(BIN_NEG).into());
    assert_eq!('0', Trit(BIN_ZERO).into());
    assert_eq!('1', Trit(BIN_POS).into());
}

#[test]
fn trit_from_char() {
    assert_eq!(Ok(Trit(BIN_NEG)), Trit::try_from('T'));
    assert_eq!(Ok(Trit(BIN_ZERO)), Trit::try_from('0'));
    assert_eq!(Ok(Trit(BIN_POS)), Trit::try_from('1'));

    assert!(Trit::try_from('t').is_err());
    assert!(Trit::try_from('S').is_err());
    assert!(Trit::try_from('2').is_err());
}

#[test]
fn trit_negate() {
    assert_eq!(POS, -NEG);
    assert_eq!(ZERO, -ZERO);
    assert_eq!(NEG, -POS);
}

#[test]
fn trit_and() {
    assert_eq!(ZERO, ZERO & ZERO);
    assert_eq!(ZERO, ZERO & POS);
    assert_eq!(NEG, ZERO & NEG);
    assert_eq!(ZERO, POS & ZERO);
    assert_eq!(POS, POS & POS);
    assert_eq!(NEG, POS & NEG);
    assert_eq!(NEG, NEG & ZERO);
    assert_eq!(NEG, NEG & POS);
    assert_eq!(NEG, NEG & NEG);
}

#[test]
fn trit_or() {
    assert_eq!(ZERO, ZERO | ZERO);
    assert_eq!(POS, ZERO | POS);
    assert_eq!(ZERO, ZERO | NEG);
    assert_eq!(POS, POS | ZERO);
    assert_eq!(POS, POS | POS);
    assert_eq!(POS, POS | NEG);
    assert_eq!(ZERO, NEG | ZERO);
    assert_eq!(POS, NEG | POS);
    assert_eq!(NEG, NEG | NEG);
}

#[test]
fn trit_add() {
    assert_eq!((ZERO, ZERO), ZERO.add_with_carry(ZERO, ZERO));
    assert_eq!((POS, ZERO), ZERO.add_with_carry(ZERO, POS));
    assert_eq!((NEG, ZERO), ZERO.add_with_carry(ZERO, NEG));
    assert_eq!((POS, ZERO), ZERO.add_with_carry(POS, ZERO));
    assert_eq!((NEG, POS), ZERO.add_with_carry(POS, POS));
    assert_eq!((ZERO, ZERO), ZERO.add_with_carry(POS, NEG));
    assert_eq!((NEG, ZERO), ZERO.add_with_carry(NEG, ZERO));
    assert_eq!((ZERO, ZERO), ZERO.add_with_carry(NEG, POS));
    assert_eq!((POS, NEG), ZERO.add_with_carry(NEG, NEG));
    assert_eq!((POS, ZERO), POS.add_with_carry(ZERO, ZERO));
    assert_eq!((NEG, POS), POS.add_with_carry(ZERO, POS));
    assert_eq!((ZERO, ZERO), POS.add_with_carry(ZERO, NEG));
    assert_eq!((NEG, POS), POS.add_with_carry(POS, ZERO));
    assert_eq!((ZERO, POS), POS.add_with_carry(POS, POS));
    assert_eq!((POS, ZERO), POS.add_with_carry(POS, NEG));
    assert_eq!((ZERO, ZERO), POS.add_with_carry(NEG, ZERO));
    assert_eq!((POS, ZERO), POS.add_with_carry(NEG, POS));
    assert_eq!((NEG, ZERO), POS.add_with_carry(NEG, NEG));
    assert_eq!((NEG, ZERO), NEG.add_with_carry(ZERO, ZERO));
    assert_eq!((ZERO, ZERO), NEG.add_with_carry(ZERO, POS));
    assert_eq!((POS, NEG), NEG.add_with_carry(ZERO, NEG));
    assert_eq!((ZERO, ZERO), NEG.add_with_carry(POS, ZERO));
    assert_eq!((POS, ZERO), NEG.add_with_carry(POS, POS));
    assert_eq!((NEG, ZERO), NEG.add_with_carry(POS, NEG));
    assert_eq!((POS, NEG), NEG.add_with_carry(NEG, ZERO));
    assert_eq!((NEG, ZERO), NEG.add_with_carry(NEG, POS));
    assert_eq!((ZERO, NEG), NEG.add_with_carry(NEG, NEG));
}

#[test]
fn trit_mul() {
    assert_eq!(ZERO, ZERO * ZERO);
    assert_eq!(ZERO, ZERO * POS);
    assert_eq!(ZERO, ZERO * NEG);
    assert_eq!(ZERO, POS * ZERO);
    assert_eq!(POS, POS * POS);
    assert_eq!(NEG, POS * NEG);
    assert_eq!(ZERO, NEG * ZERO);
    assert_eq!(NEG, NEG * POS);
    assert_eq!(POS, NEG * NEG);
}

#[test]
fn trit_cmp() {
    assert!(ZERO == ZERO);
    assert!(ZERO < POS);
    assert!(ZERO > NEG);
    assert!(POS > ZERO);
    assert!(POS > NEG);
    assert!(POS == POS);
    assert!(NEG < ZERO);
    assert!(NEG < POS);
    assert!(NEG == NEG);
}
