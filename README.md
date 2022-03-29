# ngclp

Ngclp (no-grammar definition command-line parser) is one of Rust's command-line parsers. 
A normal command-line parser generates a parser from the definition of command-line options that accepts a command line according to its grammar. In contrast, ngclp uses a universal parser to discover what it assumes to be options or arguments from the given command-line arguments.

## How it works?

You can build the sample program with "cargo build" and try it as follows:

```sh
$ cargo build
$ target/debug/ngclp foo -bar bal --bra boo
Argument "foo" .
Option "-b" with argument "ar" .
Argument "bal" .
Option "--bra" with argument "boo" .
```

## Format of options accepted by ngclp

When you write a single letter of the alphabet as A, B, etc., ngclp accepts the following options.

|  Format   |                Parsed                 |
| --------- | ------------------------------------- |
| `-A`      | Option with no argument `-A`.         |
| `-A BC`   | Option `-A` with argument `BC`.       |
| `-ABC`    | Option `-A` with argument `BC`.       |
| `--AB`    | Option with no arguments `--AB`.      |
| `--AB CD` | Option `--AB` with the argument `CD`. |
| `--AB=CD` | Option `--AB` with argument `CD`.     |
| `--`      | Separator.                            |

"But isn't that ambiguous?" If you are wondering, you are correct.

When the command line is

`-a bc`

ngclp will output the following two interpretations.

* The option `-a` appears with no arguments (the next `bc` is a normal command-line argument that has nothing to do with the option `-a`).
* The option `-a` appears with the argument `bc`.

## How do I use ngclp?

Copy the boilerplate code [src/main.rs](src/main.rs) as your `main.rs` and modify it.

## License

MIT/Apache-2.0

## Links

* [ngclp's page in Crates.io](https://crates.io/crates/ngclp)
* [ngclp's repository in GitHub](https://github.com/tos-kamiya/ngclp)
