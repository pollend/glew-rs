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
use crate::argument_parser::Arg::{Alias, Fundamental, Struct};
use crate::argument_parser::FundamentalType::{SignedLongLongInt, UnsignedLongInt};

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
    Struct(String),
    Alias(String)
}

#[derive(Clone, Eq, PartialEq)]
struct ArgumentDef {
    is_const: bool,
    argument: Arg,
    pointer: Option<Vec<PointerType>>,
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

fn parse_struct_type<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    (map(tuple((space0, verify(recognize(alphanumeric1), |a: &str| a.eq("struct")), space0, recognize(alphanumeric1))),
         |(_, _, _, value)| {
             value
         }))(i)
}


fn parse_fundamental_type<'a, E: ParseError<&'a str>>(
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

    (map_opt(
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
    ))(i)
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
    (many1(
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


#[test]
fn parser_test_argument() {
    let arg = |string: &str, def: ArgumentDef| {
        let value = parse_argument::<VerboseError<&str>>(string);
        let result = value.finish();
        assert!(result.is_ok());
        let value = result.unwrap().1;
        assert!(def.argument == value.argument);
        assert_eq!(def.is_const, value.is_const);
        assert!(def.name.eq(value.name.as_str()));
        match (def.pointer, value.pointer) {
            (Some(a), Some(b)) => {
                assert!(a.as_slice().eq(b.as_slice()))
            },
            (None, None) => {},
            (_, _) => { assert!(false) }
        }
    };

    arg("GLenum type", ArgumentDef {
        is_const: false,
        argument: Alias("GLenum".to_string()),
        pointer: None,
        name: "type".to_string()
    });

    arg("unsigned long int test1", ArgumentDef {
        is_const: false,
        argument: Fundamental(UnsignedLongInt),
        pointer: None,
        name: "test1".to_string()
    });

    arg("const GLdouble* c", ArgumentDef {
        is_const: true,
        argument: Alias("GLdouble".to_string()),
        pointer: Some(vec![PointerType::Normal]),
        name: "c".to_string()
    });

    arg("struct test* c", ArgumentDef {
        is_const: false,
        argument: Struct("test".to_string()),
        pointer: Some(vec![PointerType::Normal]),
        name: "c".to_string()
    });

    arg("GLenum** rdsdf12", ArgumentDef {
        is_const: false,
        argument: Alias("GLenum".to_string()),
        pointer: Some(vec![PointerType::Normal, PointerType::Normal]),
        name: "rdsdf12".to_string()
    });

    arg("const GLenum** rdsdf12", ArgumentDef {
        is_const: true,
        argument: Alias("GLenum".to_string()),
        pointer: Some(vec![PointerType::Normal, PointerType::Normal]),
        name: "rdsdf12".to_string()
    });
}



fn parse_argument<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, ArgumentDef, E> {
    (alt((
        map(tuple((opt(parse_const), parse_struct_type, opt(parse_pointer_definition), parse_ident)), |(const_type, struct_name, pointer_def, variable_name)| {
            ArgumentDef {
                is_const: const_type.is_some(),
                argument: Struct(struct_name.to_string()),
                pointer: pointer_def,
                name: variable_name.to_string()
            }
        }),
        map(tuple((opt(parse_const), parse_fundamental_type, opt(parse_pointer_definition), parse_ident)), |(const_type, type_def, pointer_def, variable_name)| {
            ArgumentDef {
                is_const: const_type.is_some(),
                argument: Fundamental(type_def),
                pointer: pointer_def,
                name: variable_name.to_string()
            }
        }),
        map(tuple((opt(parse_const), parse_ident, opt(parse_pointer_definition), parse_ident)), |(const_type, type_name, pointer_def, variable_name)| {
            ArgumentDef {
                is_const: const_type.is_some(),
                argument: Alias(type_name.to_string()),
                pointer: pointer_def,
                name: variable_name.to_string()
            }
        })
    )))(i)
}
