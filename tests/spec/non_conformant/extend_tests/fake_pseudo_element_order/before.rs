//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/fake-pseudo-element-order/before.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%a:before {x: y}\
            \nb:c {@extend %a}\
            \n"
        )
        .unwrap(),
        "b:c:before {\
        \n  x: y;\
        \n}\
        \n"
    );
}
