// A sample main function which parse command-line arguments with ng-clp.

use std::env;

use ng_clp::{is_argument, next_index, parse, unwrap_argument};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

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
                println!("Usage: {} [-h] [-o OUTPUT] [-v] <file>...", NAME);
                return Ok(())
            }
            "-V" | "--version" => {
                println!("{} {}", NAME, VERSION);
                return Ok(())
            }

            "-o" | "--output" => { // option (takes one argument)
                let output = unwrap_argument(pr)?; // if no argument is given, fails by this unwrapping
                println!("output = {}", output);
                2 // 1 for the option + 1 for argument
            }

            "-v" | "--verbose" => { // flag (does not take argument)
                println!("verbose");
                1 // 1 for the flag // if the command-line says -v option has an argument (like, --verbose=1), fails later in calling `next_index`
            }

            "--" => { // separator (the remaining items are arguments, including items having prefix `-`)
                argv_index += 1; // skip the `--`
                break;
            }

            a if is_argument(a) => { // argument (does not take argument)
                println!("file = {}", a);
                1 // 1 for the argument
            }

            _ => 0 // unknown flag/option // fails later in calling `next_index`
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
