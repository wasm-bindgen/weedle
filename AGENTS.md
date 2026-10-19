# weedle

weedle is a WebIDL parser built on nom. The `ast_types!` macro in
`src/macros.rs` declares each AST node together with its parser, so the
grammar lives in the type definitions across `src/` (`interface.rs`,
`dictionary.rs`, `types.rs` and so on), and `weedle::parse` in `src/lib.rs`
produces `Definitions`.

## Build & Test

```sh
cargo build
cargo test
```

## Conventions

- Grammar changes should follow the WebIDL specification and be covered by
  `test!`/`test_variants!` unit tests in the module that defines the node.
- `tests/webidl.rs` parses real-world IDL in `tests/defs` and asserts
  definition counts; a grammar change that alters those counts needs
  explaining.
- weedle is a published library consumed by wasm-bindgen's WebIDL frontend,
  so changes to public AST types are semver-relevant and worth calling out.
