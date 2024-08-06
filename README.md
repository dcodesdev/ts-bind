# ts-bind

A Rust crate for generating TypeScript bindings from structs.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ts-bind = "0.1.0"
```

## Usage

Add the following to your Rust code:

```rust
use ts_bind::TsBind;

#[derive(TsBind)]
struct MyStruct {
    field1: String,
    field2: i32,
}
```

This will generate a TypeScript interface in the `bindings` directory.

## Contributing

Feel free to open issues or submit pull requests on our [GitHub repository](https://github.com/dcodesdev/ts-bind).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
