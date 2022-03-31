# ng-clp

Ng-clp (no-grammar definition command-line parser) is one of Rust's command-line parsers. 
A normal command-line parser generates a parser from the definition of command-line options that accepts a command line according to its grammar. In contrast, ng-clp uses a universal parser to discover what it assumes to be options or arguments from the given command-line arguments.

## How it works?

You can run a sample program with `cargo run` as follows:

```sh
$ cargo run -- foo -bar bal --bra boo
Argument: foo
Option: -b ar
Argument: bal
Option: --bra boo
```

## Format of flags and options accepted by ng-clp

The arguments, flags, and options accepted by ng-clp can be described as follows (option names, arguments, etc. are capitalized here to be easier to distinguish).

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

ng-clp allows treating with the command line in the following two interpretations.

* A flag `-a` appears (with no argument; the following `bc` is a normal command-line argument that has nothing to do with the flag `-a`).
* An option `-a` appears with argument `bc`.

## How do I use ng-clp?

(1) Add a dependency to `ng-clp` and `anyhow` in `Cargo.toml`:

```
[dependencies]
ng-clp = "0.3"
anyhow = "1.0"
```

(2) Copy the boilerplate code [boilerplate/main.rs](boilerplate/main.rs) in your `main.rs` and modify it.

## License

MIT/Apache-2.0

## Links

* [ng-clp's page in Crates.io](https://crates.io/crates/ng-clp)
* [ng-clp's repository in GitHub](https://github.com/tos-kamiya/ng-clp)

ng-clp is based on the same idea as the original product [gzclp](https://github.com/tos-kamiya/zgclp), but their APIs are completely different from each other.
