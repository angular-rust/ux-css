use super::strings::name;
use super::util::{ignore_comments, opt_spacelike};
use super::value::space_list;
use super::Span;
use crate::sassengine::sass::{CallArgs, FormalArgs};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;

pub fn formal_args(input: Span) -> IResult<Span, FormalArgs> {
    let (input, _) = terminated(tag("("), opt_spacelike)(input)?;
    let (input, v) = separated_list0(
        preceded(tag(","), opt_spacelike),
        map(
            pair(
                delimited(tag("$"), name, opt_spacelike),
                opt(delimited(
                    terminated(tag(":"), opt_spacelike),
                    space_list,
                    opt_spacelike,
                )),
            ),
            |(name, d)| (name.into(), d),
        ),
    )(input)?;
    let (input, _) = terminated(opt(tag(",")), opt_spacelike)(input)?;
    let (input, va) = terminated(opt(tag("...")), opt_spacelike)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((
        input,
        if va.is_none() {
            FormalArgs::new(v)
        } else {
            FormalArgs::new_va(v)
        },
    ))
}

pub fn call_args(input: Span) -> IResult<Span, CallArgs> {
    let (input, _) = tag("(")(input)?;
    let (input, v) = separated_list0(
        delimited(opt_spacelike, tag(","), opt_spacelike),
        pair(
            opt(delimited(
                tag("$"),
                map(name, |n: String| n.replace("-", "_")),
                preceded(ignore_comments, tag(":")),
            )),
            alt((
                space_list,
                delimited(ignore_comments, space_list, ignore_comments),
            )),
        ),
    )(input)?;
    let (input, _) = preceded(
        opt(delimited(opt_spacelike, opt(tag(",")), opt_spacelike)),
        tag(")"),
    )(input)?;
    Ok((input, CallArgs::new(v)))
}
