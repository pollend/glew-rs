use std::io::ErrorKind;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alphanumeric1, digit1};
use nom::combinator::{map, map_opt, map_res, opt, value};
use nom::error::{context, ContextError, ParseError, VerboseError};
use nom::{Finish, IResult};
use nom::multi::many1;
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

#[derive(Clone)]
enum PointerType{
    None,
    Normal,
    ConstPointer
}

#[derive(Clone)]
enum Arg {
    Fundamental(FundamentalType),
    Alias(String)
}

struct ArgumentDef {
    argument: Arg,
    pointer: [PointerType; 5],
    name: String
}

#[test]
fn parser_test_fundamental_type() {
    let test_parser = |string: &str, type_expect: FundamentalType| {
        let value = parse_fundamental_type::<VerboseError<&str>>(string);
        let result = value.finish();
        assert!(result.is_ok());
        assert!(result.unwrap().1 == type_expect);
    };

    test_parser("long long int", FundamentalType::SignedLongLongInt);
    test_parser("long int long", FundamentalType::SignedLongLongInt);
    test_parser("signed long long int", FundamentalType::SignedLongLongInt);

    test_parser("unsigned long int long", FundamentalType::UnsignedLongLongInt);
    test_parser("unsigned long long int", FundamentalType::UnsignedLongLongInt);

    test_parser("int", FundamentalType::SignedInt);

}


/// parser combinators are constructed from the bottom up:
/// first we write parsers for the smallest elements (here a space character),
/// then we'll combine them in larger parsers
fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";

    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    take_while(move |c| chars.contains(c))(i)
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
            value(SymbolType::UnsignedType, preceded(sp, tag("unsigned"))),
            value(SymbolType::SignedType, preceded(sp, tag("signed"))),
            value(SymbolType::ShortType, preceded(sp, tag("short"))),
            value(SymbolType::IntType, preceded(sp, tag("int"))),
            value(SymbolType::LongType, preceded(sp, tag("long"))),
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

//
// fn parse_pointer_definition<'a, E: ParseError<&'a str>>(
//     i: &'a str,
// ) -> IResult<&'a str, &'a Vec<PointerType>, E> {
//     let pointer = preceded(sp, many1("*"));
//     let const_type = preceded(sp, tag("const"));
//     (map_res(many1(tuple((&pointer, opt(&const_type)))),
//              |items| {
//                  Ok(items.iter()
//                      .map(|it| {
//                          if it.1.is_some() {
//                              return PointerType::ConstPointer;
//                          }
//                          return PointerType::Normal
//                      }).collect())
//              }))(i)
// }
//
// fn parse_argument<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
//     let ident = preceded(sp, alphanumeric1);
//     let const_arg = preceded(sp, tag("const"));
//     alt((
//         tuple((parse_fundamental_type, parse_pointer_definition, &ident)),
//         tuple((&ident, parse_pointer_definition, &ident)),
//         tuple((&const_arg, &ident, parse_pointer_definition, &ident)),
//         tuple((&const_arg, &ident, parse_pointer_definition, &ident)),
//     ))
// }
