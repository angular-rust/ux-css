//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/082_test_not_remains_at_end_of_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo:not(.bar) {a: b}\
            \n.baz {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo:not(.bar), .baz:not(.bar) {\
        \n  a: b;\
        \n}\
        \n"
    );
}
