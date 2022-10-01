#[cfg(test)]

mod test {
    use  ng_clp::*;

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

        let ni = next_index(&argv, 0, 0);
        assert_eq!(ni, Err(ParseError::UnknownFlagOrOption { name: "-a".to_string() }));
        let ni = next_index(&argv, 0, 1);
        assert_eq!(ni, Ok(1));
        let ni = next_index(&argv, 0, 2);
        assert_eq!(ni, Ok(2));
        let ni = next_index(&argv, 0, 3);
        assert_eq!(ni, Err(ParseError::InternalInvalidEatCount { eat: 3 }));
        let ni = next_index(&argv, 1, 1);
        assert_eq!(ni, Ok(2));
        let ni = next_index(&argv, 1, 2);
        assert_eq!(
            ni,
            Err(ParseError::InternalArgumentCanNotHaveArgument {
                arg: "1".to_string()
            })
        );
        let ni = next_index(&argv, 2, 1);
        assert_eq!(ni, Ok(3));
        let ni = next_index(&argv, 2, 2);
        assert_eq!(
            ni,
            Err(ParseError::OptionWithoutArgument {
                name: "-f".to_string()
            })
        );
        let ni = next_index(&argv, 3, 1);
        assert_eq!(
            ni,
            Err(ParseError::FlagWithArgument {
                name: "-g".to_string()
            })
        );
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
                }
                ("-f", a2) => {
                    assert_eq!(a2, None);
                    1
                }
                ("-g", a2) => {
                    assert_eq!(a2, Some("3"));
                    2
                }
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
        assert_eq!(pr, Ok(("-g", None)));
        let pr = parse(&argv, 4);
        assert_eq!(pr, Ok(("--", None)));
        let pr = parse(&argv, 5);
        assert_eq!(pr, Ok(("-h", None)));
        let pr = parse(&argv, 6);
        assert_eq!(pr, Err(ParseError::InternalIndexOutOfRange { index: 6 }));

        let ni = next_index(&argv, 0, 1);
        assert_eq!(
            ni,
            Err(ParseError::FlagWithArgument {
                name: "-a".to_string()
            })
        );
        let ni = next_index(&argv, 0, 2);
        assert_eq!(ni, Ok(1));
        let ni = next_index(&argv, 1, 0);
        assert_eq!(ni, Err(ParseError::UnknownFlagOrOption { name: "-f".to_string() }));
        let ni = next_index(&argv, 1, 1);
        assert_eq!(ni, Ok(2));
        let ni = next_index(&argv, 1, 2);
        assert_eq!(ni, Ok(3));
        let ni = next_index(&argv, 1, 3);
        assert_eq!(ni, Err(ParseError::InternalInvalidEatCount { eat: 3 }));
        let ni = next_index(&argv, 2, 1);
        assert_eq!(ni, Ok(3));
        let ni = next_index(&argv, 2, 2);
        assert_eq!(
            ni,
            Err(ParseError::InternalArgumentCanNotHaveArgument {
                arg: "-".to_string()
            })
        );
        let ni = next_index(&argv, 3, 1);
        assert_eq!(ni, Ok(4));
        let ni = next_index(&argv, 3, 2);
        assert_eq!(
            ni,
            Err(ParseError::OptionWithoutArgument { 
                name: "-g".to_string()
            })
        );
        let ni = next_index(&argv, 4, 1);
        assert_eq!(ni, Ok(5));
        let ni = next_index(&argv, 4, 2);
        assert_eq!(ni, Err(ParseError::InternalSeparatorCanNotHaveArgument));
        let ni = next_index(&argv, 5, 1);
        assert_eq!(ni, Ok(6));
        let ni = next_index(&argv, 5, 2);
        assert_eq!(
            ni,
            Err(ParseError::OptionWithoutArgument {
                name: "-h".to_string()
            })
        );
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
        assert_eq!(
            ni,
            Err(ParseError::InternalArgumentCanNotHaveArgument {
                arg: "1".to_string()
            })
        );
        let ni = next_index(&argv, 2, 1);
        assert_eq!(ni, Ok(3));
        let ni = next_index(&argv, 2, 2);
        assert_eq!(
            ni,
            Err(ParseError::OptionWithoutArgument {
                name: "--ff".to_string()
            })
        );
        let ni = next_index(&argv, 3, 1);
        assert_eq!(
            ni,
            Err(ParseError::FlagWithArgument {
                name: "--gg".to_string()
            })
        );
        let ni = next_index(&argv, 3, 2);
        assert_eq!(ni, Ok(4));
        let ni = next_index(&argv, 4, 1);
        assert_eq!(ni, Err(ParseError::InternalIndexOutOfRange { index: 4 }));
    }

    #[test]
    fn confusing_string() {
        let argv = vec!["---", "1"];
        let pr = parse(&argv, 0);
        assert_eq!(pr, Err(ParseError::InvalidString { s: "---".to_string() }));

        let ni = next_index(&argv, 0, 0);
        assert_eq!(ni, Err(ParseError::InvalidString { s: "---".to_string() }));
        let ni = next_index(&argv, 0, 1);
        assert_eq!(ni, Err(ParseError::InvalidString { s: "---".to_string() }));
        let ni = next_index(&argv, 0, 2);
        assert_eq!(ni, Err(ParseError::InvalidString { s: "---".to_string() }));

        let argv = vec!["--.", "1"];
        let pr = parse(&argv, 0);
        assert_eq!(pr, Ok(("--.", Some("1")))); // "---" is accepted as option

        let ni = next_index(&argv, 0, 1);
        assert_eq!(ni, Ok(1));
        let ni = next_index(&argv, 0, 2);
        assert_eq!(ni, Ok(2));

        let argv = vec!["--=", "1"];
        let pr = parse(&argv, 0);
        assert_eq!(pr, Err(ParseError::InvalidString { s: "--=".to_string() }));
        let ni = next_index(&argv, 0, 0);
        assert_eq!(ni, Err(ParseError::InvalidString { s: "--=".to_string() }));
        let ni = next_index(&argv, 0, 1);
        assert_eq!(ni, Err(ParseError::InvalidString { s: "--=".to_string() }));
        let ni = next_index(&argv, 0, 2);
        assert_eq!(ni, Err(ParseError::InvalidString { s: "--=".to_string() }));
    }

    #[test]
    fn long_option_with_empty_value() {
        let argv = vec!["--aa=", "2"];
        let pr = parse(&argv, 0);
        assert_eq!(pr, Ok(("--aa", Some(""))));
        let pr = parse(&argv, 1);
        assert_eq!(pr, Ok(("2", None)));

        let ni = next_index(&argv, 0, 2);
        assert_eq!(ni, Ok(1));
        let ni = next_index(&argv, 0, 1);
        assert_eq!(
            ni,
            Err(ParseError::FlagWithArgument {
                name: "--aa".to_string()
            })
        );
        let ni = next_index(&argv, 1, 1);
        assert_eq!(ni, Ok(2));
    }
}
