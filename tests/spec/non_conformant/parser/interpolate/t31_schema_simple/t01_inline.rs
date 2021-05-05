//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/31_schema_simple/01_inline.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: \"[\"\'foo\'\"]\";\
            \n  output: #{\"[\"\'foo\'\"]\"};\
            \n  output: \"[#{\"[\"\'foo\'\"]\"}]\";\
            \n  output: \"#{\"[\"\'foo\'\"]\"}\";\
            \n  output: \'#{\"[\"\'foo\'\"]\"}\';\
            \n  output: \"[\'#{\"[\"\'foo\'\"]\"}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[\" \"foo\" \"]\";\
        \n  output: [ foo ];\
        \n  output: \"[[ foo ]]\";\
        \n  output: \"[ foo ]\";\
        \n  output: \"[ foo ]\";\
        \n  output: \"[\'[ foo ]\']\";\
        \n}\
        \n"
    );
}
