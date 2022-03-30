use std::env;

use ng_clp::{is_argument, next_index, parse, unwrap_argument};

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
            "--" => { // separator (the remaining items are arguments, including items having prefix `-`)
                break;
            }

            a if is_argument(a) => { // argument (does not take argument)
                println!("Argument: {}", a);
                1 // 1 for the argument
            }

            fo => { // flag or option
                if let Ok(a) = unwrap_argument(pr) {
                    println!("Option: {} {}", fo, a);
                    2 // 1 for the option + 1 for argument
                } else {
                    println!("Flag: {}", fo);
                    1 // 1 for the flag
                }
            }
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
