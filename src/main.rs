// A sample main function which parse command-line arguments with ngclp.

use std::env;

use anyhow;

use ngclp::{is_argument, next_index, parse, unwrap_argument};

fn main() -> anyhow::Result<()> {
    let argv_store: Vec<String> = env::args().collect();
    let argv: Vec<&str> = argv_store.iter().map(|item| item.as_ref()).collect();

    if argv.len() == 1 {
        println!("No arguments.");
        return Ok(());
    }

    let mut argv_index = 1;
    loop {
        let pr = parse(&argv, argv_index)?;
        let eat = match pr.0 {
            "-h" | "--help" => { // help
                println!("Usage: ngclp [-h] [-o OUTPUT] [-v] <file>...");
                return Ok(())
            }

            "-o" | "--output" => { // option (takes one argument)
                let output = unwrap_argument(pr)?;
                println!("output = {}", output);
                2 // 1 for the option + 1 for argument
            }

            "-v" | "--verbose" => { // flag (does not take argument)
                println!("verbose");
                1 // 1 for the flag
            }

            "--" => { // separator (the remaining items are arguments, including items having prefix `-`)
                break;
            }

            a if is_argument(a) => { // argument (does not take argument)
                println!("file = {}", a);
                1 // 1 for the argument
            }

            _ => 0 // unknown flag/option
        };

        argv_index = next_index(&argv, argv_index, eat)?;
        if argv_index >= argv.len() {
            break;
        }
    }

    if argv_index < argv.len() {
        println!("remaining argv items = {}", argv[argv_index..].join(","));
    }

    Ok(())
}
