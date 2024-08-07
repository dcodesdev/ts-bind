![TsBind How it works](./assets/ts-bind.gif)

# TsBind

A Rust crate for generating TypeScript bindings from structs.

## Installation

```bash
cargo add ts-bind

```

Add this to your `Cargo.toml`:

```toml
[dependencies]
ts-bind = "0.1.2"
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

This will generate the corresponding TypeScript interface in the `bindings` directory.

```tsx
// bindings/MyStruct.ts

interface MyStruct {
  field1: string;
  field2: number;
}
```

## Attributes

The `ts_bind` attribute supports the following optional arguments:

- `rename`: Rename the generated interface.

```rust
#[derive(TsBind)]
struct User {
    id: i32,
    #[ts_bind(rename = "postCount")]
    post_count: i32,
}
```

```tsx
export interface User {
  id: number;
  postCount: number;
}
```

## Contributing

Feel free to open issues or submit pull requests on our [GitHub repository](https://github.com/dcodesdev/ts-bind).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
