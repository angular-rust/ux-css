//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/156_test_combinator_unification_double_angle.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".a.b > x {a: b}\
            \n.b > y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a.b > x, .b.a > y {\
        \n  a: b;\
        \n}\
        \n"
    );
}