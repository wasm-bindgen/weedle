
# Weedle

**A WebIDL Parser**

[API Docs](https://docs.rs/weedle) | [Chat](https://rustwasm.github.io/weedle/chat/)

Built with 🦀🕸 by The Rust and WebAssembly Working Group

---

## About

Weedle is a parser for WebIDL (Web Interface Definition Language) specifications. It takes valid WebIDL definitions as input and produces a structured Rust data representation starting from the `Definitions` type.

WebIDL is commonly used to define interfaces and APIs for web browsers and other environments, making Weedle useful for projects involving WebAssembly, browser APIs, or tooling around web standards.

---

## Features

- Parses valid WebIDL definitions
- Produces strongly typed Rust data structures
- Lightweight and easy to integrate into Rust projects
- Maintained by the Rust and WebAssembly Working Group

---

## Installation

Add Weedle as a dependency in your `Cargo.toml`:

```toml
[dependencies]
weedle = "0.9.0"
```

---

## Usage

Here is a simple example demonstrating how to parse a WebIDL interface definition:

```rust
fn main() {
    let idl = "interface Window { readonly attribute Storage sessionStorage; };";
    let parsed = weedle::parse(idl).unwrap();
    println!("{:?}", parsed);
}
```

This will parse the WebIDL string and print the resulting data structure.

---

## Documentation

For detailed API documentation, visit the [docs.rs page](https://docs.rs/weedle).

---

## Contributing

Contributions are welcome! Please open issues or pull requests on the [GitHub repository](https://github.com/rustwasm/weedle).

---

## License

This project is licensed under the MIT License.

---

## Acknowledgments

Built and maintained by the Rust and WebAssembly Working Group.

---

If you want, I can help you generate badges, examples, or additional sections like FAQ or Troubleshooting!

Citations:
[1] https://github.com/rustwasm/weedle

