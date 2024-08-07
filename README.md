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

| Argument | Description                     |
| -------- | ------------------------------- |
| `rename` | Rename the generated interface. |

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

## Todo

The library is far from complete. Here are some of the features that are planned:

- [ ] Support for enums.
- [ ] `#[ts_bind(export = "path/to/export")]` custom export path.
- [ ] `#[ts_bind(rename_all = "camelCase")]` attribute to rename all fields.
- [ ] `#[ts_bind(skip)]` attribute to skip fields.
- [ ] `#[ts_bind(skip_if = "condition")]` attribute to skip fields based on a condition.

## Contributing

Feel free to open issues or submit pull requests on our [GitHub repository](https://github.com/dcodesdev/ts-bind).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
