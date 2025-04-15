# The Zipper

![Crates.io](https://img.shields.io/crates/v/the_zipper)

The Zipper is a Rust library designed to provide efficient and ergonomic utilities for working with data structures using the zipper pattern. This library simplifies navigation and modification of complex data structures while maintaining immutability.

HUET G. The Zipper. Journal of Functional Programming. 1997;7(5):549-554. doi:10.1017/S0956796897002864

## Features

- Easy-to-use API for zipper-based data manipulation.
- Supports various data structures like trees and lists.
- Lightweight and performant.
- Code coverage is 100%.

## Usage

```rust
use the_zipper::Location;

fn main() {
    let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

    let location = Location::new(tree);

    let location = location.go_down().unwrap();
    assert_eq!(location.cursor, Tree::Item("a"));

    let location = location.go_right().unwrap();
    assert_eq!(location.cursor, Tree::Item("+"));

    let location = location.go_left().unwrap();
    assert_eq!(location.cursor, Tree::Item("a"));

    let location = location.insert_right(Tree::Item(".")).unwrap();
    assert_eq!(
        location,
        Location {
            cursor: Tree::Item("a"),
            path: Path::Node {
                left: vec![],
                right: vec![Tree::Item("."), Tree::Item("+"), Tree::Item("b")],
                path: Path::Node {
                    left: vec![],
                    right: vec![Tree::Section(vec![
                        Tree::Item("a"),
                        Tree::Item("+"),
                        Tree::Item("b")
                    ])],
                    path: Path::Top.into()
                }
                .into()
            }
            .into()
        }
        .into()
    );
}
```

## Links

[F# Implementation](https://github.com/Denys-Bushulyak/the-zipper-fsharp)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

<!--
---
![Docs.rs](https://docs.rs/zipper_rust/badge.svg)
![Coverage](https://img.shields.io/badge/coverage-100%25-brightgreen) -->
