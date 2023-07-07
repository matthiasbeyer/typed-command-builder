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

---

Released under terms of [MPL-2.0 license](./LICENSE).
