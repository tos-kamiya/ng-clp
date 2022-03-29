#![doc = include_str!("../README.md")]

use std::result;

use thiserror::Error;

pub fn is_argument(a: &str) -> bool {
    a == "-" || a == "--" || !a.starts_with('-')
}

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("internal, argument can not have argument: {}", arg)]
    InternalArgumentCanNotHaveArgument { arg: String },

    #[error("internal, invalid eat count: {}", .eat)]
    InternalInvalidEatCount { eat: usize },

    #[error("internal, index out of range: {}", .index)]
    InternalIndexOutOfRange { index: usize },

    #[error("flag can not take argument: {}", .name)]
    FlagWithArgument { name: String },

    #[error("option missing argument: {}", .name)]
    OptionWithoutArgument { name: String },

    #[error("invalid argument: {}", .value)]
    InvalidArgument { value: String },
    
    #[error("unknown flag or option: {}", .name)]
    UnknownFlagOrOption { name: String },
}

pub fn parse<'s, 'a>(argv: &'a [&'s str], index: usize) -> result::Result<(&'s str, Option<&'s str>), ParseError> {
    if index >= argv.len() {
        return Err(ParseError::InternalIndexOutOfRange { index: index });
    }

    let a = argv[index];
    if is_argument(a) {
        Ok((a, None))
    } else if a.starts_with("--") {
        if let Some(i) = a.find('=') {
            Ok((&a[..i], Some(&a[i + 1..])))
        } else {
            if index + 1 < argv.len() && is_argument(argv[index + 1]) {
                Ok((a, Some(argv[index + 1])))
            } else {
                Ok((a, None))
            }
        }
    } else if a.starts_with('-') {
        if a.len() > 2 {
            Ok((&a[..2], Some(&a[2..])))
        } else {
            if index + 1 < argv.len() && is_argument(argv[index + 1]) {
                Ok((&a, Some(argv[index + 1])))
            } else {
                Ok((a, None))
            }
        }
    } else {
        unreachable!();
    }
}

pub fn next_index(argv: &[&str], index: usize, eat: usize) -> result::Result<usize, ParseError> {
    if index >= argv.len() {
        return Err(ParseError::InternalIndexOutOfRange { index });
    }

    let a = argv[index];
    if eat == 0 {
        if is_argument(a) {
            return Err(ParseError::InvalidArgument { value: a.to_string() });
        } else if a.starts_with("--") {
            if let Some(i) = a.find('=') {
                return Err(ParseError::UnknownFlagOrOption { name: a[..i].to_string() });
            } else {
                return Err(ParseError::UnknownFlagOrOption { name: a.to_string() });
            }
        } else if a.starts_with('-') {
            if a.len() > 2 {
                return Err(ParseError::UnknownFlagOrOption { name: a[..2].to_string() });
            } else {
                return Err(ParseError::UnknownFlagOrOption { name: a.to_string() });
            }
        } else {
            unreachable!();
        }
    } else if !(eat == 1 || eat == 2) {
        return Err(ParseError::InternalInvalidEatCount { eat });
    }

    let ni = if is_argument(a) {
        if eat == 2 {
            return Err(ParseError::InternalArgumentCanNotHaveArgument { arg: a.to_string() });
        }
        index + 1
    } else if a.starts_with("--") {
        if let Some(i) = a.find('=') {
            if eat == 1 {
                return Err(ParseError::FlagWithArgument { name: a[..i].to_string() });
            } else {
                assert_eq!(eat, 2);
                index + 1
            }
        } else {
            if index + 1 < argv.len() && is_argument(argv[index + 1]) {
                index + eat
            } else if eat == 2 {
                return Err(ParseError::OptionWithoutArgument { name: a.to_string() });
            } else {
                assert_eq!(eat, 1);
                index + 1
            }
        }
    } else if a.starts_with('-') {
        if a.len() > 2 {
            if eat == 1 {
                return Err(ParseError::FlagWithArgument { name: a[..2].to_string() });
            } else {
                assert_eq!(eat, 2);
                index + 1
            }
        } else {
            if index + 1 < argv.len() && is_argument(argv[index + 1]) {
                index + eat
            } else if eat == 2 {
                return Err(ParseError::OptionWithoutArgument { name: a.to_string() });
            } else {
                assert_eq!(eat, 1);
                index + 1
            }
        }
    } else {
        unreachable!();
    };

    Ok(ni)
}

pub fn unwrap_argument<'s>(parse_result: (&'s str, Option<&'s str>)) -> result::Result<&'s str, ParseError> {
    if let Some(a) = parse_result.1 {
        Ok(a)
    } else {
        Err(ParseError::OptionWithoutArgument { name: parse_result.0.to_string() })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn short_option_simple() {
        let argv = vec!["-a", "1", "-f", "-g3"];

        let pr = parse(&argv, 0);
        assert_eq!(pr, Ok(("-a", Some("1"))));
        let pr = parse(&argv, 1);
        assert_eq!(pr, Ok(("1", None)));
        let pr = parse(&argv, 2);
        assert_eq!(pr, Ok(("-f", None)));
        let pr = parse(&argv, 3);
        assert_eq!(pr, Ok(("-g", Some("3"))));
        let pr = parse(&argv, 4);
        assert_eq!(pr, Err(ParseError::InternalIndexOutOfRange { index: 4 }));

        let ni = next_index(&argv, 0, 1);
        assert_eq!(ni, Ok(1));
        let ni = next_index(&argv, 0, 2);
        assert_eq!(ni, Ok(2));
        let ni = next_index(&argv, 0, 3);
        assert_eq!(ni, Err(ParseError::InternalInvalidEatCount { eat: 3 }));
        let ni = next_index(&argv, 1, 1);
        assert_eq!(ni, Ok(2));
        let ni = next_index(&argv, 1, 2);
        assert_eq!(ni, Err(ParseError::InternalArgumentCanNotHaveArgument { arg: "1".to_string() }));
        let ni = next_index(&argv, 2, 1);
        assert_eq!(ni, Ok(3));
        let ni = next_index(&argv, 2, 2);
        assert_eq!(ni, Err(ParseError::OptionWithoutArgument { name: "-f".to_string() }));
        let ni = next_index(&argv, 3, 1);
        assert_eq!(ni, Err(ParseError::FlagWithArgument { name: "-g".to_string() }));
        let ni = next_index(&argv, 3, 2);
        assert_eq!(ni, Ok(4));
        let ni = next_index(&argv, 4, 1);
        assert_eq!(ni, Err(ParseError::InternalIndexOutOfRange { index: 4 }));

        let mut i = 0;
        loop {
            let pr = parse(&argv, i);
            let eat = match pr.unwrap() {
                ("-a", a2) => {
                    assert_eq!(a2, Some("1"));
                    1
                },
                ("-f", a2) => {
                    assert_eq!(a2, None);
                    1
                },
                ("-g", a2) => {
                    assert_eq!(a2, Some("3"));
                    2
                },
                (v, a2) => {
                    assert_eq!(v, "1");
                    assert_eq!(a2, None);
                    1
                }
            };
            let j = next_index(&argv, i, eat).unwrap();
            assert!(j > i);
            i = j;
            if i >= argv.len() {
                break;
            }
        }
    }

    #[test]
    fn short_option_complicated() {
        let argv = vec!["-a=1", "-f", "-", "-g", "--", "-h"];

        let pr = parse(&argv, 0);
        assert_eq!(pr, Ok(("-a", Some("=1"))));
        let pr = parse(&argv, 1);
        assert_eq!(pr, Ok(("-f", Some("-"))));
        let pr = parse(&argv, 2);
        assert_eq!(pr, Ok(("-", None)));
        let pr = parse(&argv, 3);
        assert_eq!(pr, Ok(("-g", Some("--"))));
        let pr = parse(&argv, 4);
        assert_eq!(pr, Ok(("--", None)));
        let pr = parse(&argv, 5);
        assert_eq!(pr, Ok(("-h", None)));
        let pr = parse(&argv, 6);
        assert_eq!(pr, Err(ParseError::InternalIndexOutOfRange { index: 6 }));

        let ni = next_index(&argv, 0, 1);
        assert_eq!(ni, Err(ParseError::FlagWithArgument { name: "-a".to_string() }));
        let ni = next_index(&argv, 0, 2);
        assert_eq!(ni, Ok(1));
        let ni = next_index(&argv, 0, 3);
        assert_eq!(ni, Err(ParseError::InternalInvalidEatCount { eat: 3 }));
        let ni = next_index(&argv, 1, 1);
        assert_eq!(ni, Ok(2));
        let ni = next_index(&argv, 1, 2);
        assert_eq!(ni, Ok(3));
        let ni = next_index(&argv, 2, 1);
        assert_eq!(ni, Ok(3));
        let ni = next_index(&argv, 2, 2);
        assert_eq!(ni, Err(ParseError::InternalArgumentCanNotHaveArgument { arg: "-".to_string() }));
        let ni = next_index(&argv, 3, 1);
        assert_eq!(ni, Ok(4));
        let ni = next_index(&argv, 3, 2);
        assert_eq!(ni, Ok(5));
        let ni = next_index(&argv, 4, 1);
        assert_eq!(ni, Ok(5));
        let ni = next_index(&argv, 4, 2);
        assert_eq!(ni, Err(ParseError::InternalArgumentCanNotHaveArgument { arg: "--".to_string() }));
        let ni = next_index(&argv, 5, 1);
        assert_eq!(ni, Ok(6));
        let ni = next_index(&argv, 5, 2);
        assert_eq!(ni, Err(ParseError::OptionWithoutArgument { name: "-h".to_string() }));
        let ni = next_index(&argv, 6, 1);
        assert_eq!(ni, Err(ParseError::InternalIndexOutOfRange { index: 6 }));
    }

    #[test]
    fn long_option_simple() {
        let argv = vec!["--aa", "1", "--ff", "--gg=3"];
        let pr = parse(&argv, 0);
        assert_eq!(pr, Ok(("--aa", Some("1"))));
        let pr = parse(&argv, 1);
        assert_eq!(pr, Ok(("1", None)));
        let pr = parse(&argv, 2);
        assert_eq!(pr, Ok(("--ff", None)));
        let pr = parse(&argv, 3);
        assert_eq!(pr, Ok(("--gg", Some("3"))));
        let pr = parse(&argv, 4);
        assert_eq!(pr, Err(ParseError::InternalIndexOutOfRange { index: 4 }));

        let ni = next_index(&argv, 0, 1);
        assert_eq!(ni, Ok(1));
        let ni = next_index(&argv, 0, 2);
        assert_eq!(ni, Ok(2));
        let ni = next_index(&argv, 0, 3);
        assert_eq!(ni, Err(ParseError::InternalInvalidEatCount { eat: 3 }));
        let ni = next_index(&argv, 1, 1);
        assert_eq!(ni, Ok(2));
        let ni = next_index(&argv, 1, 2);
        assert_eq!(ni, Err(ParseError::InternalArgumentCanNotHaveArgument { arg: "1".to_string() }));
        let ni = next_index(&argv, 2, 1);
        assert_eq!(ni, Ok(3));
        let ni = next_index(&argv, 2, 2);
        assert_eq!(ni, Err(ParseError::OptionWithoutArgument { name: "--ff".to_string() }));
        let ni = next_index(&argv, 3, 1);
        assert_eq!(ni, Err(ParseError::FlagWithArgument { name: "--gg".to_string() }));
        let ni = next_index(&argv, 3, 2);
        assert_eq!(ni, Ok(4));
        let ni = next_index(&argv, 4, 1);
        assert_eq!(ni, Err(ParseError::InternalIndexOutOfRange { index: 4 }));
    }
}
