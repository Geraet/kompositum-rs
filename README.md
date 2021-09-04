# Kompositum for Rust

This is the first attempt of porting Kompositum to Rust. Keep in mind that this is subject to change and might not be ready yet for production.

## Requirements

Install Rust from here [https://www.rust-lang.org/](https://www.rust-lang.org/).

## How to build and run the tests

Building the project:
```
git clone https://github.com/Geraet/kompositum-rs.git
cd kompositum-rs
cargo build
```

Running the tests:
```
cargo test --lib -- --show-output
```

## Example

Create a new project:
```sh
cargo new myapp
```

```Cargo.toml```
```toml
[package]
name = "myapp"
version = "0.1.0"
edition = "2018"

[dependencies]
kompositum = { git = "https://github.com/Geraet/kompositum-rs", branch = "master" }
```

```main.rs```
```rust
use kompositum;

fn main() {
    const TREE_MAP_DEF: &[(kompositum::IDType, kompositum::IDType)] =
        &[(1, 2), (1, 3), (1, 4), (4, 5), (4, 6), (1, 7)];
    let tree_map: kompositum::MultiMap<kompositum::IDType, kompositum::IDType> =
        TREE_MAP_DEF.iter().cloned().collect();

    let root = kompositum::builder::build_composite(1, &tree_map);
    root.accept(&mut kompositum::printer::Printer::new());
}
```

## License

Released under the [MIT License](LICENSE)
