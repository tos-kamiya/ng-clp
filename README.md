# ngclp

Ngclp (no-grammar definition command-line parser) is one of Rust's command-line parsers. 
A normal command-line parser generates a parser from the definition of command-line options that accepts a command line according to its grammar. In contrast, ngclp uses a universal parser to discover what it assumes to be options or arguments from the given command-line arguments.

## How it works?

You can run a sample program with `cargo run` as follows:

```sh
$ cargo run -- foo -bar bal --bra boo
Argument: foo
Option: -b ar
Argument: bal
Option: --bra boo
```

## Format of flags and options accepted by ngclp

When you write a single letter of the alphabet as A, B, etc., ngclp accepts the following arguments and options.

|  Format   |                Parsed                 |
| --------- | ------------------------------------- |
| `-A`      | Flag `-A`.                            |
| `-A BC`   | Option `-A` with argument `BC`.       |
| `-ABC`    | Option `-A` with argument `BC`.       |
| `--AB`    | Flag `--AB`.                          |
| `--AB CD` | Option `--AB` with the argument `CD`. |
| `--AB=CD` | Option `--AB` with argument `CD`.     |

"But isn't that ambiguous?" If you are wondering, you are correct.

When the command line is:

`-a bc`

ngclp allows to treat with the command line in the following two interpretations.

* A flag `-a` appears (with no argument; the following `bc` is a normal command-line argument that has nothing to do with the flag `-a`).
* A option `-a` appears with argument `bc`.

## How do I use ngclp?

Copy the boilerplate code [boilerplate/main.rs](boilerplate/main.rs) in your `main.rs` and modify it.

## License

MIT/Apache-2.0

## Links

* [ngclp's page in Crates.io](https://crates.io/crates/ngclp)
* [ngclp's repository in GitHub](https://github.com/tos-kamiya/ngclp)
