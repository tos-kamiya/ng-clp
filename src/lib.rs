use std::result;

use thiserror::Error;

pub fn is_invalid(a: &str) -> bool {
    a.starts_with("---") || a.starts_with("--=")
}

pub fn is_argument(a: &str) -> bool {
    !is_invalid(a) && (a == "-" || !a.starts_with('-'))
}

pub fn is_separator(a: &str) -> bool {
    a == "--"
}

pub fn is_flag_or_option(a: &str) -> bool {
    !is_invalid(a) && !is_argument(a) && !is_separator(a)
}

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("internal, argument can not have argument: {}", arg)]
    InternalArgumentCanNotHaveArgument { arg: String },

    #[error("internal, separator can not have argument")]
    InternalSeparatorCanNotHaveArgument,

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

    #[error("unexpected separator: {}", .value)]
    UnexpectedSeparator { value: String },

    #[error("unknown flag or option: {}", .name)]
    UnknownFlagOrOption { name: String },

    #[error("invalid string: {}", .s)]
    InvalidString { s: String },
}

pub fn parse<'s, 'a>(
    argv: &'a [&'s str],
    index: usize,
) -> result::Result<(&'s str, Option<&'s str>), ParseError> {
    if index >= argv.len() {
        return Err(ParseError::InternalIndexOutOfRange { index: index });
    }

    let a = argv[index];
    if is_invalid(a) {
        return Err(ParseError::InvalidString { s: a.to_string() });
    } else if is_argument(a) || is_separator(a) {
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
    } else {
        assert!(a.starts_with('-'));
        if a.len() > 2 {
            Ok((&a[..2], Some(&a[2..])))
        } else {
            if index + 1 < argv.len() && is_argument(argv[index + 1]) {
                Ok((a, Some(argv[index + 1])))
            } else {
                Ok((a, None))
            }
        }
    }
}

pub fn next_index(argv: &[&str], index: usize, eat: usize) -> result::Result<usize, ParseError> {
    if index >= argv.len() {
        return Err(ParseError::InternalIndexOutOfRange { index });
    }

    let a = argv[index];
    if eat == 0 {
        if is_invalid(a) {
            return Err(ParseError::InvalidString { s: a.to_string() });
        } else if is_argument(a) {
            return Err(ParseError::InvalidArgument {
                value: a.to_string(),
            });
        } else if is_separator(a) {
            return Err(ParseError::UnexpectedSeparator {
                value: a.to_string(),
            });
        } else if is_flag_or_option(a) {
            if let Some(i) = a.find('=') {
                return Err(ParseError::UnknownFlagOrOption {
                    name: a[..i].to_string(),
                });
            } else {
                return Err(ParseError::UnknownFlagOrOption {
                    name: a.to_string(),
                });
            }
        } else {
        }
    } else if !(eat == 1 || eat == 2) {
        return Err(ParseError::InternalInvalidEatCount { eat });
    }

    let ni = if is_invalid(a) {
        return Err(ParseError::InvalidString { s: a.to_string() });
    } else if is_argument(a) {
        if eat == 2 {
            return Err(ParseError::InternalArgumentCanNotHaveArgument { arg: a.to_string() });
        }
        index + 1
    } else if is_separator(a) {
        if eat == 2 {
            return Err(ParseError::InternalSeparatorCanNotHaveArgument);
        } else {
            assert_eq!(eat, 1);
            index + 1
        }
    } else {
        assert!(is_flag_or_option(a));
        if a.starts_with("--") {
            if let Some(i) = a.find('=') {
                if eat == 1 {
                    return Err(ParseError::FlagWithArgument {
                        name: a[..i].to_string(),
                    });
                } else {
                    assert_eq!(eat, 2);
                    index + 1
                }
            } else {
                if eat == 2 {
                    if index + 1 < argv.len() && is_argument(argv[index + 1]) {
                        index + eat
                    } else {
                        return Err(ParseError::OptionWithoutArgument {
                            name: a.to_string(),
                        });
                    }
                } else {
                    index + eat
                }
            }
        } else if a.len() > 2 {
            assert!(a.starts_with('-') && &a[1..2] != "-");
            if eat == 1 {
                return Err(ParseError::FlagWithArgument {
                    name: a[..2].to_string(),
                });
            } else {
                assert_eq!(eat, 2);
                index + 1
            }
        } else {
            if index + 1 < argv.len() && is_argument(argv[index + 1]) {
                index + eat
            } else if eat == 2 {
                return Err(ParseError::OptionWithoutArgument {
                    name: a.to_string(),
                });
            } else {
                assert_eq!(eat, 1);
                index + 1
            }
        }
    };

    Ok(ni)
}

pub fn unwrap_argument<'s>(
    parse_result: (&'s str, Option<&'s str>),
) -> result::Result<&'s str, ParseError> {
    if let Some(a) = parse_result.1 {
        Ok(a)
    } else {
        Err(ParseError::OptionWithoutArgument {
            name: parse_result.0.to_string(),
        })
    }
}
