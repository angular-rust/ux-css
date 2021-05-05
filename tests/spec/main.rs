//! Tests auto-converted from "sass-spec/spec"
//! version c26991ec6, 2021-03-17 16:50:10 +0100.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["core_functions/selector/extend", "core_functions/selector/is_superselector", "core_functions/selector/unify", "directives/extend", "directives/forward", "directives/use", "libsass-closed-issues/issue_185/mixin.hrx", "libsass-todo-issues/issue_221262.hrx", "libsass-todo-issues/issue_221292.hrx", "libsass/unicode-bom/utf-16-big", "libsass/unicode-bom/utf-16-little", "non_conformant/scss/huge.hrx", "non_conformant/scss/mixin-content.hrx", "non_conformant/scss/multiline_var.hrx"]
use ::css::output::Format;
use ::css::{parse_scss_data, SassError, FsFileContext, ScopeRef};

mod arguments;

mod core_functions;

mod css;

mod directives;

mod expressions;

mod libsass;

mod libsass_closed_issues;

mod libsass_todo_issues;

mod libsass_todo_tests;

mod non_conformant;

mod values;

fn rsass(input: &str) -> Result<String, String> {
    rsass_fmt(Default::default(), input)
}
#[allow(unused)]
fn rsass_fmt(format: Format, input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), format)
        .map_err(|e| {
            eprintln!("{}", e);
            "rsass failed".into()
        })
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
pub fn compile_scss(input: &[u8], format: Format) -> Result<Vec<u8>, SassError> {
    let mut file_context = FsFileContext::new();
    file_context.push_path("tests/spec".as_ref());
    let items = parse_scss_data(input)?;
    format.write_root(&items, ScopeRef::new_global(format), &file_context)
}
