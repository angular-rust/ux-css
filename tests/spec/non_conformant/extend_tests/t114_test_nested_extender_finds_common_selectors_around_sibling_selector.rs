//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/114_test_nested_extender_finds_common_selectors_around_sibling_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "a ~ b c .c1 {a: b}\
            \nb c .c2 {@extend .c1}\
            \n"
        )
        .unwrap(),
        "a ~ b c .c1, a ~ b c .c2 {\
        \n  a: b;\
        \n}\
        \n"
    );
}
