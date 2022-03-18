use std::io::ErrorKind;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alpha1, alphanumeric1, char, digit1, space0, space1};
use nom::combinator::{all_consuming, map, map_opt, map_res, opt, recognize, value, verify};
use nom::error::{context, ContextError, ParseError, VerboseError};
use nom::{Finish, IResult};
use nom::character::is_alphabetic;
use nom::multi::{many0, many0_count, many1};
use nom::sequence::{preceded, tuple};
use crate::argument_parser::FundamentalType::SignedLongLongInt;

#[derive(Clone, Eq, PartialEq)]
enum FundamentalType {
    SignedShortInt,
    SignedInt,
    SignedLongInt,
    SignedLongLongInt,
    UnsignedShortInt,
    UnsignedInt,
    UnsignedLongInt,
    UnsignedLongLongInt,
}

#[derive(Clone, Eq, PartialEq)]
enum Arg {
    Fundamental(FundamentalType),
    Alias(String)
}

#[derive(Clone, Eq, PartialEq)]
struct ArgumentDef {
    argument: Arg,
    pointer: [PointerType; 5],
    name: String
}


#[derive(Clone, Eq, PartialEq)]
enum PointerType{
    None,
    Normal,
    ConstPointer
}



#[test]
fn parser_test_fundamental_type() {
    let test_valid = |string: &str, type_expect: FundamentalType| {
        let value = parse_fundamental_type::<VerboseError<&str>>(string);
        let result = value.finish();
        assert!(result.is_ok());
        assert!(result.unwrap().1 == type_expect);
    };

    let test_invalid = |string: &str| {
        let value = parse_fundamental_type::<VerboseError<&str>>(string);
        let result = value.finish();
        assert!(result.is_err());

    };

    test_valid(" long long int", FundamentalType::SignedLongLongInt);
    test_valid("long int long", FundamentalType::SignedLongLongInt);
    test_valid("signed long long int", FundamentalType::SignedLongLongInt);

    test_valid("unsigned long int long", FundamentalType::UnsignedLongLongInt);
    test_valid("unsigned long long int", FundamentalType::UnsignedLongLongInt);

    test_valid("int", FundamentalType::SignedInt);


    test_valid("int abe", FundamentalType::SignedInt);

    test_invalid("longlong");
    test_invalid("inglong");
}


fn parse_fundamental_type<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, FundamentalType, E> {

    #[derive(Clone)]
    enum SymbolType {
        SignedType,
        UnsignedType,
        ShortType,
        LongType,
        IntType,
    }

    (context("fundamental_type",map_opt(
    many1(alt((
            value(SymbolType::UnsignedType, tuple((space0,  verify(recognize(alphanumeric1), |a: &str| a.eq("unsigned"))))),
            value(SymbolType::SignedType, tuple((space0,  verify(recognize(alphanumeric1), |a: &str| a.eq("signed"))))),
            value(SymbolType::ShortType, tuple((space0, verify(recognize(alphanumeric1), |a: &str| a.eq("short"))))),
            value(SymbolType::IntType, tuple((space0, verify(recognize(alphanumeric1), |a: &str| a.eq("int"))))),
            value(SymbolType::LongType, tuple((space0, verify(recognize(alphanumeric1), |a: &str| a.eq("long"))))),
        ))),
        |items| {
            let signed_count = items.iter().filter(|x| match x {
                SymbolType::SignedType => true,
                _ => false
            }).count();
            let unsigned_count = items
                .iter()
                .filter(|x| match x {
                    SymbolType::UnsignedType => true,
                    _ => false
                })
                .count();
            let short_count = items.iter().filter(|x| match x {
                SymbolType::ShortType => true,
                _ => false
            }).count();
            let int_count = items.iter().filter(|x| match x {
                SymbolType::IntType => true,
                _ => false
            }).count();
            let long_count = items.iter().filter(|x| match x {
                SymbolType::LongType => true,
                _ => false
            }).count();

            let is_unsigned = if unsigned_count == 1 {
                true
            } else if (signed_count == 0 || signed_count == 1) && unsigned_count == 0 {
                false
            } else {
                return None
            };

            if long_count == 2 && int_count <= 1  && short_count == 0 {
                return Some(if is_unsigned {
                    FundamentalType::UnsignedLongLongInt
                } else {
                    FundamentalType::SignedLongLongInt
                })
            }

            if long_count == 1 && int_count <= 1  && short_count == 0 {
                return Some(if is_unsigned {
                    FundamentalType::UnsignedLongInt
                } else {
                    FundamentalType::SignedLongInt
                })
            }

            if short_count == 1 && int_count <= 1 && long_count == 0{
                return Some(if is_unsigned {
                    FundamentalType::UnsignedShortInt
                } else {
                    FundamentalType::SignedShortInt
                })
            }

            if int_count <= 1 {
                return Some((if is_unsigned {
                    FundamentalType::UnsignedInt
                } else {
                    FundamentalType::SignedInt
                }))
            }
            return None
        },
    )))(i)
}



#[test]
fn parser_test_pointer_type() {
    let test_valid = |string: &str, type_expect: &[PointerType]| {
        let value = parse_pointer_definition::<VerboseError<&str>>(string);
        let result = value.finish();
        assert!(result.is_ok());
        let c = result.unwrap().1;
        assert_eq!(c.len(), type_expect.len());
        assert!(c.eq(type_expect));
    };

    test_valid("**const", &[PointerType::Normal, PointerType::ConstPointer]);
    test_valid("*const", &[ PointerType::ConstPointer]);
    test_valid("*", &[ PointerType::Normal]);
}

fn parse_pointer_definition<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<PointerType>, E> {
    (many0(
        map(tuple((space0, char('*'), opt(tag("const")))),
            |(_, _, const_type)| {
                if (const_type.is_some()) {
                    return PointerType::ConstPointer;
                }
                return PointerType::Normal;
            })))(i)
}

fn parse_ident<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    (map(tuple((space0, recognize(alphanumeric1))), |(_, identity)| {
        identity
    }))(i)
}


fn parse_const<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    (map(tuple((space0,  verify(recognize(alphanumeric1), |a: &str| a.eq("const")))), |(_, c)| {
        c
    }))(i)
}

// fn parse_argument<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
//
//     alt((
//         tuple((parse_const, parse_ident, opt(parse_pointer_definition), parse_ident)),
//         tuple((parse_ident, opt(parse_pointer_definition), parse_ident)),
//         tuple((parse_const, parse_fundamental_type, opt(parse_pointer_definition), parse_ident)),
//         tuple((parse_fundamental_type, opt(parse_pointer_definition), parse_ident)),
//     ))
// }
