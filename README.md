# The Zipper

The Zipper is a Rust library designed to provide efficient and ergonomic utilities for working with data structures using the zipper pattern. This library simplifies navigation and modification of complex data structures while maintaining immutability.

## Features

- Easy-to-use API for zipper-based data manipulation.
- Supports various data structures like trees and lists.
- Lightweight and performant.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
the_zipper = "0.1.0"
```

## Usage

```rust
use the_zipper::Location;

fn main() {
    let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

    let location = Location::new(tree.clone());
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

![Crates.io](https://img.shields.io/crates/v/zipper_rust)
![Docs.rs](https://docs.rs/zipper_rust/badge.svg)
![Coverage](https://img.shields.io/badge/coverage-100%25-brightgreen)  