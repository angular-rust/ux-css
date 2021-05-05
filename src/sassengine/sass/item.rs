use super::{CallArgs, FormalArgs, Name, SassString, Value};
use crate::sassengine::parser::SourcePos;
use crate::sassengine::selectors::Selectors;

/// Every sass file is a sequence of sass items.
/// Scoping items contains further sequences of items.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum Item {
    /// An `@import` directive.
    Import(Vec<SassString>, Value, SourcePos),
    /// A variable declaration.
    VariableDeclaration {
        /// The variable name
        name: String,
        /// The bound value
        val: Value,
        /// true if this is a `!default` variable.
        default: bool,
        /// true if this is a `!global` variable.
        global: bool,
    },

    /// An `@at-root` directive.
    AtRoot(Selectors, Vec<Item>),
    /// A generic `@` directive.
    AtRule {
        /// The name of this directive
        name: String,
        /// Any arguments
        args: Value,
        /// The directive may have a body.
        body: Option<Vec<Item>>,
    },
    /// An `@debug` directive.
    Debug(Value),
    /// An `@warn` directive.
    Warn(Value),
    /// An `@error` directive.
    Error(Value),

    /// A `@mixin` directive, declaring a mixin.
    MixinDeclaration(String, FormalArgs, Vec<Item>),
    /// An `@include` directive, calling a mixin.
    MixinCall(String, CallArgs, Vec<Item>),
    /// An `@content` directive (in a mixin declaration).
    Content,

    /// An `@function` declaration.
    FunctionDeclaration(String, FormalArgs, SourcePos, Vec<Item>),
    /// An `@return` statement in a function declaration.
    Return(Value),

    /// An `@if` conditional directive.
    IfStatement(Value, Vec<Item>, Vec<Item>),
    /// An `@each` loop directive.
    ///
    /// The value may be or evaluate to a list.
    Each(Vec<Name>, Value, Vec<Item>),
    /// An `@for` loop directive.
    For {
        /// The name of the iteration variable.
        name: Name,
        /// The start value for the iteration.
        from: Box<Value>,
        /// The end value for the iteration.
        to: Box<Value>,
        /// True if the end should be included in the range.
        inclusive: bool,
        /// The body of the loop.
        body: Vec<Item>,
    },
    /// An `@while` loop directive.
    While(Value, Vec<Item>),

    /// An `@use` directive.
    Use(SassString, UseAs),
    /// A sass rule; selectors followed by a block of items.
    Rule(Selectors, Vec<Item>),
    /// A sass namespace rule; a name followed by a block of properties.
    NamespaceRule(SassString, Value, Vec<Item>),
    /// A sass property; a name and a value.
    Property(SassString, Value),
    /// A comment (that might be preserved for the output).
    Comment(SassString),
    /// Nothing
    None,
}

/// The `as` part of an `@use` directive.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum UseAs {
    /// A plain `@use foo;`.
    KeepName,
    /// Include the module contents directly in the scope, `@use foo as *;`.
    Star,
    /// An explicit name, `@use foo as bar`.
    Name(SassString),
}
