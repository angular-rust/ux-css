use crate::sassengine::css::Value;
use crate::sassengine::output::Format;
use crate::sassengine::parser::{ParseError, SourcePos};
use crate::sassengine::sass::Name;
use crate::sassengine::value::RangeError;
use std::convert::From;
use std::{fmt, io};

/// Most functions in rsass that returns a Result uses this Error type.
#[derive(Debug)]
pub enum SassError {
    /// An IO error encoundered on a specific path
    Input(String, io::Error),
    /// An IO error without specifying a path.
    ///
    /// This is (probably) an error writing output.
    IoError(io::Error),
    /// A bad call to a builtin function, with call- and optionally
    /// declaration position.
    BadCall(String, SourcePos, Option<SourcePos>),
    InvalidFunctionName(SourcePos),
    BadValue(String),
    BadArgument(Name, String),
    /// The pos here is the function declaration.
    /// This error will be wrapped in a BadCall, giving the pos of the call.
    BadArguments(String, SourcePos),
    /// A range error
    BadRange(RangeError),
    /// Error parsing sass data.
    ParseError(ParseError),
    UndefinedVariable(String),
    /// Fallback error type.
    ///
    /// This just contains a string with some message.
    S(String),
}

impl std::error::Error for SassError {}

impl SassError {
    pub fn bad_value(expected: &str, actual: &Value) -> Self {
        SassError::BadValue(format!(
            "Error: {} is not {}.",
            actual.format(Default::default()),
            expected,
        ))
    }

    /// Wrong kind of argument to a sass function.
    /// `expected` is a string describing what the parameter should
    /// have been, `actual` is the argument.
    pub fn bad_arg(
        name: Name,
        actual: &Value,
        problem: &'static str,
    ) -> SassError {
        SassError::BadArgument(
            name,
            format!("{} {}", actual.format(Format::introspect()), problem),
        )
    }

    pub fn undefined_variable(name: &str) -> Self {
        SassError::UndefinedVariable(name.to_string())
    }

    pub fn error<T: AsRef<str>>(msg: T) -> Self {
        SassError::S(format!("Error: {}.", msg.as_ref()))
    }
}

impl fmt::Display for SassError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SassError::S(ref s) => write!(out, "{}", s),
            SassError::Input(ref p, ref e) => {
                write!(out, "Failed to read {:?}: {}", p, e)
            }
            SassError::UndefinedVariable(ref name) => {
                write!(out, "Undefined variable: \"${}\"", name)
            }
            SassError::BadArgument(ref name, ref problem) => {
                write!(out, "Error: ${}: {}.", name, problem)
            }
            SassError::ParseError(ref err) => err.fmt(out),
            SassError::BadCall(ref msg, ref callpos, ref declpos) => {
                msg.fmt(out)?;
                writeln!(out)?;
                if let Some(declpos) = declpos {
                    callpos.show_detail(out, '^', " invocation")?;
                    writeln!(out)?;
                    declpos.show_detail(out, '=', " declaration")?;
                    callpos.show_files(out)
                } else {
                    callpos.show(out)
                }
            }
            SassError::InvalidFunctionName(ref pos) => {
                writeln!(out, "Error: Invalid function name.")?;
                pos.show(out)
            }
            SassError::BadRange(ref err) => err.fmt(out),
            SassError::BadValue(ref err) => err.fmt(out),
            // fallback
            ref x => write!(out, "{:?}", x),
        }
    }
}

impl From<io::Error> for SassError {
    fn from(e: io::Error) -> Self {
        SassError::IoError(e)
    }
}

impl From<ParseError> for SassError {
    fn from(e: ParseError) -> Self {
        SassError::ParseError(e)
    }
}
impl From<RangeError> for SassError {
    fn from(e: RangeError) -> Self {
        SassError::BadRange(e)
    }
}
