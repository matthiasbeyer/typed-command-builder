<!-- markdownlint-disable MD033 MD041 -->
<h3 align="center">typed-command-builder</h3>

---

`typed-command-builder` is a crate for declaring interfaces to
`std::process::Command` or `tokio::process::Command` that is typed.

You can use this library for desgning a type that can be called like this:

```rust
// Call `cat` (see `man cat`):
let command = CatCommand::new()
    .show_all(true) // --show-all
    .number(true) // --number
    .file("/some/thing") // [FILE]...
    .file("/some/thing"); // [FILE]...

let out = command.run().unwrap();
```

## Attribution

This crates implementation is heavily based on
[typed-builder](https://github.com/idanarye/rust-typed-builder), authored by
[Idan Arye](https://github.com/idanarye).

A huge portion of the credit belongs to them!

## License

As this crate is based on [typed-builder](https://github.com/idanarye/rust-typed-builder), it is licensed like that crate:

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
