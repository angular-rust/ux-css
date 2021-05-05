//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/076_test_pseudo_unification.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a .baz:after {a: b}\
            \n:foo {@extend .baz} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .baz:after, -a :foo:after {\
        \n  a: b;\
        \n}\
        \n"
    );
}