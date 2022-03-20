use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, one_of};
use nom::combinator::{map_res, opt, recognize};
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;
use proc_macro2::{Span, TokenStream};

// Interleaves a number, for example 100000 => 100_000. Mostly used to make clippy happy
pub fn interleave_number(symbol: char, count: usize, n: &str) -> String {
    let number: String = n
        .chars()
        .rev()
        .enumerate()
        .fold(String::new(), |mut acc, (idx, next)| {
            if idx != 0 && idx % count == 0 {
                acc.push(symbol);
            }
            acc.push(next);
            acc
        });
    number.chars().rev().collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Constant {
    Number {
        sign: Option<String>,
        number: String,
    },
    Hex(String),
}

impl quote::ToTokens for Constant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match *self {
            Constant::Number {
                ref sign,
                ref number,
            } => {
                let number = interleave_number('_', 4, number.as_str());
                syn::LitInt::new(
                    &format!(
                        "{}{}",
                        match sign.as_ref() {
                            None => {
                                ""
                            }
                            Some(res) => {
                                res
                            }
                        },
                        number
                    ),
                    Span::call_site(),
                )
                .to_tokens(tokens);
            }
            Constant::Hex(ref n) => {
                let number = interleave_number('_', 4, n);
                syn::LitInt::new(&format!("0x{}", number), Span::call_site()).to_tokens(tokens);
            }
        }
    }
}

pub fn parse_hex(input: &str) -> IResult<&str, Constant> {
    map_res(
        preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(many1(terminated(
                one_of("0123456789abcdefABCDEF"),
                many0(char('_')),
            ))),
        ),
        |out: &str| -> Result<Constant, ()> { Ok(Constant::Hex(out.to_ascii_lowercase())) },
    )(input)
}

pub fn parse_number(input: &str) -> IResult<&str, Constant> {
    map_res(
        tuple((opt(one_of("+-")), many1(one_of("0123456789")))),
        |(sign, number)| -> Result<Constant, ()> {
            Ok(Constant::Number {
                sign: sign.map(|e| format!("{}", e)),
                number: number.into_iter().collect(),
            })
        },
    )(input)
}

pub fn parse_constant(i: &str) -> IResult<&str, Constant> {
    let hex = parse_hex(i);
    let number = parse_number(i);
    return hex.or(number);
}
