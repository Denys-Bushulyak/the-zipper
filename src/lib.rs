//! # The Zipper
//!
//! The Zipper is a Rust library designed to provide efficient and ergonomic utilities
//! for working with data structures using the zipper pattern. This library simplifies
//! navigation and modification of complex data structures while maintaining immutability.
//!
//! HUET G. The Zipper. *Journal of Functional Programming*. 1997;7(5):549–554.
//! doi:[10.1017/S0956796897002864](https://doi.org/10.1017/S0956796897002864)
//!
//! ## Features
//!
//! - Easy-to-use API for zipper-based data manipulation.
//! - Supports various data structures like trees and lists.
//! - Lightweight and performant.
//! - Code coverage is 100%.
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! the_zipper = "0.1.2"
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use the_zipper::{Tree, Location, Path};
//!
//! fn main() {
//!     let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);
//!
//!     let location = Location::new(tree);
//!
//!     let location = location.go_down().unwrap();
//!     assert_eq!(location.cursor, Tree::Item("a"));
//!
//!     let location = location.go_right().unwrap();
//!     assert_eq!(location.cursor, Tree::Item("+"));
//!
//!     let location = location.go_left().unwrap();
//!     assert_eq!(location.cursor, Tree::Item("a"));
//!
//!     let location = location.insert_right(Tree::Item(".")).unwrap();
//!     assert_eq!(
//!         location,
//!         Location {
//!             cursor: Tree::Item("a"),
//!             path: Path::Node {
//!                 left: vec![],
//!                 right: vec![Tree::Item("."), Tree::Item("+"), Tree::Item("b")],
//!                 path: Path::Node {
//!                     left: vec![],
//!                     right: vec![Tree::Section(vec![
//!                         Tree::Item("a"),
//!                         Tree::Item("+"),
//!                         Tree::Item("b")
//!                     ])],
//!                     path: Path::Top.into()
//!                 }
//!                 .into()
//!             }
//!             .into()
//!         }
//!         .into()
//!     );
//! }
//! ```
//!
//! ## Links
//!
//! [F# Implementation](https://github.com/Denys-Bushulyak/the-zipper-fsharp)
//!
//! ## License
//!
//! This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
/// Represents a hierarchical tree structure.
///
/// A tree can either be a single item or a section containing multiple trees.
pub enum Tree<T: Clone> {
    /// A single item value of type T.
    Item(T),
    /// A collection of trees forming a section.
    Section(Vec<Tree<T>>),
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a path within a tree, used for navigation and context tracking.
///
/// The path keeps track of the location in the tree structure,
/// remembering the siblings to the left and right of the current position,
/// as well as the path to the parent.
pub enum Path<T: Clone> {
    /// Represents the top level of the tree hierarchy.
    Top,
    /// Represents a position within the tree structure.
    Node {
        /// Trees to the left of the current position.
        left: Vec<Tree<T>>,
        /// Trees to the right of the current position.
        right: Vec<Tree<T>>,
        /// Path to the parent node.
        path: Rc<Path<T>>,
    },
}

#[derive(Debug, PartialEq, Clone)]
/// Represents a location (cursor) within a tree.
///
/// A location combines a cursor pointing to the current tree node
/// and a path providing context for navigation within the overall tree structure.
pub struct Location<T: Clone> {
    /// The current tree node being focused on.
    pub cursor: Tree<T>,
    /// The path representing the context of this location within the overall tree.
    pub path: Rc<Path<T>>,
}

// Type alias for cache
type Cache<T> = Rc<RefCell<HashMap<usize, Rc<Location<T>>>>>;

// A wrapper that adds memoization capabilities
#[derive(Clone)]
pub struct MemoLocation<T: Clone + Eq + Hash> {
    location: Rc<Location<T>>,
    cache: Cache<T>,
}

impl<T: Clone + Eq + Hash> Location<T> {
    // Memoized navigation function
    pub fn with_memo(self) -> MemoLocation<T> {
        MemoLocation {
            location: Rc::new(self),
            cache: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl<T: Clone> Location<T> {
    /// Creates a new location from a tree.
    ///
    /// # Arguments
    ///
    /// * `tree` - The tree to create a location from.
    ///
    /// # Returns
    ///
    /// A new `Location` instance with the given tree as cursor.
    pub fn new(tree: Tree<T>) -> Self {
        Self {
            cursor: tree.clone(),
            path: Path::Node {
                left: vec![],
                right: vec![tree.clone()],
                path: Rc::new(Path::Top),
            }
            .into(),
        }
    }

    /// Moves the cursor to the left sibling.
    ///
    /// # Returns
    ///
    /// * `Some(Location)` - If there is a left sibling.
    /// * `None` - If there is no left sibling or the location is at the top.
    pub fn go_left(self) -> Option<Self> {
        match self.path.as_ref() {
            Path::Top => None,
            Path::Node { left, right, path } => left.split_first().map(|(first, rest)| Self {
                cursor: first.clone(),
                path: Path::Node {
                    left: rest.to_vec(),
                    path: path.clone(),
                    right: vec![self.cursor].into_iter().chain(right.clone()).collect(),
                }
                .into(),
            }),
        }
    }

    /// Moves the cursor to the right sibling.
    ///
    /// # Returns
    ///
    /// * `Some(Location)` - If there is a right sibling.
    /// * `None` - If there is no right sibling or the location is at the top.
    pub fn go_right(self) -> Option<Self> {
        match self.path.as_ref() {
            Path::Top => None,
            Path::Node { left, right, path } => right.split_first().map(|(first, rest)| Self {
                cursor: first.clone(),
                path: Path::Node {
                    left: vec![self.cursor].into_iter().chain(left.clone()).collect(),
                    right: rest.to_vec(),
                    path: path.clone(),
                }
                .into(),
            }),
        }
    }

    /// Moves the cursor to the parent node.
    ///
    /// # Returns
    ///
    /// * `Some(Location)` - If there is a parent node.
    /// * `None` - If the location is at the top.
    pub fn go_up(self) -> Option<Self> {
        match self.path.as_ref() {
            Path::Top => None,
            Path::Node { left, right, path } => {
                let left = left.iter().rev().cloned().collect::<Vec<Tree<T>>>();
                Self {
                    path: path.clone(),
                    cursor: Tree::Section(
                        [left, vec![self.cursor], right.clone()]
                            .iter()
                            .flatten()
                            .cloned()
                            .collect::<Vec<Tree<T>>>(),
                    ),
                }
                .into()
            }
        }
    }

    /// Moves the cursor to the first child node.
    ///
    /// # Returns
    ///
    /// * `Some(Location)` - If the current node is a section with at least one child.
    /// * `None` - If the current node is an item or an empty section.
    pub fn go_down(self) -> Option<Self> {
        match self.cursor {
            Tree::Item(_) => None,
            Tree::Section(trees) => trees.split_first().map(|(first, rest)| Self {
                cursor: first.clone(),
                path: Path::Node {
                    left: vec![],
                    right: rest.into(),
                    path: self.path,
                }
                .into(),
            }),
        }
    }

    /// Gets the nth child of the current node.
    ///
    /// This is equivalent to n calls to `go_right()`.
    ///
    /// # Arguments
    ///
    /// * `n` - The index of the child to navigate to.
    ///
    /// # Returns
    ///
    /// * `Some(Location)` - If the child exists.
    /// * `None` - If the child doesn't exist or the current node is an item.
    pub fn get_nth(self, n: usize) -> Option<Self> {
        match n {
            0 => self.go_down(),
            n => self.get_nth(n - 1).and_then(Location::go_right),
        }
    }

    /// Replaces the current node with a new tree.
    ///
    /// # Arguments
    ///
    /// * `tree` - The new tree to replace the current node with.
    ///
    /// # Returns
    ///
    /// A new location with the updated cursor.
    pub fn change(self, tree: Tree<T>) -> Self {
        Self {
            cursor: tree,
            path: self.path,
        }
    }

    /// Inserts a new tree to the right of the current node.
    ///
    /// # Arguments
    ///
    /// * `tree` - The tree to insert.
    ///
    /// # Returns
    ///
    /// * `Some(Location)` - If the insertion was successful.
    /// * `None` - If the location is at the top.
    pub fn insert_right(self, tree: Tree<T>) -> Option<Self> {
        match self.path.as_ref() {
            Path::Top => None,
            Path::Node { left, right, path } => Self {
                cursor: self.cursor.clone(),
                path: Path::Node {
                    left: left.clone(),
                    path: path.clone(),
                    right: vec![tree].into_iter().chain(right.clone()).collect(),
                }
                .into(),
            }
            .into(),
        }
    }

    /// Inserts a new tree to the left of the current node.
    ///
    /// # Arguments
    ///
    /// * `tree` - The tree to insert.
    ///
    /// # Returns
    ///
    /// * `Some(Location)` - If the insertion was successful.
    /// * `None` - If the location is at the top.
    pub fn insert_left(self, tree: Tree<T>) -> Option<Self> {
        match self.path.as_ref() {
            Path::Top => None,
            Path::Node { left, right, path } => Self {
                cursor: self.cursor.clone(),
                path: Path::Node {
                    left: vec![tree].into_iter().chain(left.clone()).collect(),
                    right: right.to_vec(),
                    path: path.clone(),
                }
                .into(),
            }
            .into(),
        }
    }

    /// Inserts a new tree as the first child of the current node.
    ///
    /// # Arguments
    ///
    /// * `tree` - The tree to insert.
    ///
    /// # Returns
    ///
    /// * `Some(Location)` - If the current node is a section.
    /// * `None` - If the current node is an item.
    pub fn insert_down(self, tree: Tree<T>) -> Option<Self> {
        match self.cursor {
            Tree::Item(_) => None,
            Tree::Section(children) => Some(Self {
                cursor: tree,
                path: Path::Node {
                    left: vec![],
                    right: children,
                    path: self.path,
                }
                .into(),
            }),
        }
    }

    /// Deletes the current node and moves the cursor to a sibling or parent.
    ///
    /// # Returns
    ///
    /// * `Some(Location)` - If the deletion was successful.
    /// * `None` - If the location is at the top.
    pub fn delete(self) -> Option<Self> {
        match self.path.as_ref() {
            Path::Top => None,
            Path::Node { left, right, path } => {
                let left = left.as_slice();
                let right = right.as_slice();

                let result = match (left, path, right) {
                    // In the middle with existing left and right
                    (left, path, [first_right, rest_right @ ..]) => Self {
                        cursor: first_right.clone(),
                        path: crate::Path::Node {
                            left: left.to_vec(),
                            right: rest_right.to_vec(),
                            path: path.clone(),
                        }
                        .into(),
                    },

                    // With empty right
                    ([first_left, rest_left @ ..], path, &[]) => Self {
                        cursor: first_left.clone(),
                        path: crate::Path::Node {
                            left: rest_left.to_vec(),
                            right: vec![],
                            path: path.clone(),
                        }
                        .into(),
                    },
                    // With empty right and left
                    ([], path, []) => Self {
                        cursor: Tree::Section(vec![]),
                        path: path.clone(),
                    },
                };

                result.into()
            }
        }
    }
}

impl<T: Clone + Eq + Hash> MemoLocation<T> {
    // Memoized version of get_nth
    pub fn get_nth(self, n: usize) -> Option<Self> {
        let cache_rc = self.cache.clone();
        let cached_location = {
            let cache = cache_rc.borrow();
            cache.get(&n).cloned()
        };

        if let Some(cached) = cached_location {
            return Some(MemoLocation {
                location: cached,
                cache: cache_rc,
            });
        }

        // Calculate the result
        let result = match n {
            0 => self.location.as_ref().clone().go_down(),
            _ => {
                let mut loc = self.location.as_ref().clone().go_down()?;
                for _ in 0..n {
                    loc = loc.go_right()?;
                }
                Some(loc)
            }
        };

        // Cache the result if it exists
        if let Some(ref loc) = result {
            let location_rc = Rc::new(loc.clone());
            cache_rc.borrow_mut().insert(n, location_rc.clone());

            Some(MemoLocation {
                location: location_rc,
                cache: cache_rc,
            })
        } else {
            None
        }
    }

    // Unwrap the inner Location
    pub fn into_inner(self) -> Location<T> {
        Rc::try_unwrap(self.location).unwrap_or_else(|rc| (*rc).clone())
    }
}

#[cfg(test)]
mod test {

    use std::rc::Rc;

    use crate::{Location, Path, Tree};

    #[test]
    fn test_new() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location::new(tree.clone());

        assert_eq!(
            location,
            Location {
                cursor: tree.clone(),
                path: Path::Node {
                    left: vec![],
                    right: vec![tree],
                    path: Rc::new(Path::Top),
                }
                .into(),
            }
        );
    }

    #[test]
    fn test_for_readme() {
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

    #[test]
    fn test_go_left_none() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        assert_eq!(location.clone().go_left(), None);
    }

    #[test]
    fn test_go_left() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let result = Location {
            path: Path::Node {
                left: vec![Tree::Item("a")],
                right: vec![Tree::Item("b")],
                path: Path::Node {
                    left: vec![],
                    right: vec![tree.clone()],
                    path: Path::Top.into(),
                }
                .into(),
            }
            .into(),
            cursor: Tree::Item("+"),
        }
        .go_left();

        let expect = Some(Location {
            path: Path::Node {
                left: vec![],
                right: vec![Tree::Item("+"), Tree::Item("b")],
                path: Path::Node {
                    left: vec![],
                    right: vec![tree],
                    path: Path::Top.into(),
                }
                .into(),
            }
            .into(),
            cursor: Tree::Item("a"),
        });

        assert_eq!(result, expect,);
    }

    #[test]
    fn test_go_right() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let result = Location {
            path: Path::Node {
                left: vec![Tree::Item("a")],
                right: vec![Tree::Item("b")],
                path: Path::Node {
                    left: vec![],
                    right: vec![tree.clone()],
                    path: Path::Top.into(),
                }
                .into(),
            }
            .into(),
            cursor: Tree::Item("+"),
        }
        .go_right();

        let expect = Some(Location {
            path: Path::Node {
                right: vec![],
                left: vec![Tree::Item("+"), Tree::Item("a")],
                path: Path::Node {
                    left: vec![],
                    right: vec![tree],
                    path: Path::Top.into(),
                }
                .into(),
            }
            .into(),
            cursor: Tree::Item("b"),
        });

        assert_eq!(result, expect);
    }

    #[test]
    fn test_go_right_none() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        assert_eq!(location.clone().go_right(), None);
    }

    #[test]
    fn test_go_up_none() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        assert_eq!(location.clone().go_up(), None);
    }

    #[test]
    fn test_go_up() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Node {
                left: vec![],
                right: vec![Tree::Item("+"), Tree::Item("b")],
                path: Path::Top.into(),
            }
            .into(),
            cursor: Tree::Item("a"),
        }
        .go_up();

        assert_eq!(
            location,
            Some(Location {
                cursor: tree.clone(),
                path: Path::Top.into(),
            })
        );
    }

    #[test]
    fn test_go_down_none() {
        let tree = Tree::Item("a");

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        assert_eq!(location.go_down(), None);
    }

    #[test]
    fn test_go_down() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        assert_eq!(
            location.go_down(),
            Some(Location {
                cursor: Tree::Item("a"),
                path: Path::Node {
                    left: [].into(),
                    right: [Tree::Item("+"), Tree::Item("b")].into(),
                    path: crate::Path::Top.into(),
                }
                .into()
            })
        );
    }

    #[test]
    fn test_get_nth_0() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        assert_eq!(
            location.get_nth(0),
            Some(Location {
                cursor: Tree::Item("a"),
                path: Path::Node {
                    left: [].into(),
                    right: [Tree::Item("+"), Tree::Item("b")].into(),
                    path: crate::Path::Top.into(),
                }
                .into()
            })
        );
    }

    #[test]
    fn test_get_nth_1() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        assert_eq!(
            location.get_nth(1),
            Some(Location {
                cursor: Tree::Item("+"),
                path: Path::Node {
                    left: [Tree::Item("a")].into(),
                    right: [Tree::Item("b")].into(),
                    path: crate::Path::Top.into(),
                }
                .into()
            })
        );
    }

    #[test]
    fn test_get_nth_2() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        assert_eq!(
            location.get_nth(2),
            Some(Location {
                cursor: Tree::Item("b"),
                path: Path::Node {
                    left: [Tree::Item("+"), Tree::Item("a")].into(),
                    right: [].into(),
                    path: crate::Path::Top.into(),
                }
                .into()
            })
        );
    }

    #[test]
    fn test_get_nth_out_of_bounds() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        assert_eq!(location.get_nth(3), None);
    }

    #[test]
    fn test_change() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        let new_tree = Tree::Item("z");

        assert_eq!(
            location.change(new_tree.clone()),
            Location {
                cursor: new_tree,
                path: Path::Top.into(),
            }
        );
    }

    #[test]
    fn test_change_after_go_left() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        let new_tree = Tree::Item("-");

        let updated_location = location
            .go_down()
            .and_then(Location::go_right)
            .map(|loc| loc.change(new_tree.clone()));

        assert_eq!(
            updated_location,
            Some(Location {
                cursor: Tree::Item("-"),
                path: Path::Node {
                    left: [Tree::Item("a")].into(),
                    right: [Tree::Item("b")].into(),
                    path: crate::Path::Top.into(),
                }
                .into()
            })
        );
    }

    #[test]
    fn test_insert_left() {
        let result = Location {
            path: Path::Node {
                left: vec![],
                right: vec![Tree::Item("+"), Tree::Item("b")],
                path: Path::Top.into(),
            }
            .into(),
            cursor: Tree::Item("a"),
        }
        .insert_left(Tree::Item("."));

        let expect = Location {
            path: Path::Node {
                left: vec![Tree::Item(".")],
                right: vec![Tree::Item("+"), Tree::Item("b")],
                path: Path::Top.into(),
            }
            .into(),
            cursor: Tree::Item("a"),
        }
        .into();

        assert_eq!(result, expect);
    }

    #[test]
    fn test_insert_left_none() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        let new_tree = Tree::Item("-");

        assert!(location.insert_left(new_tree).is_none());
    }

    #[test]
    fn test_insert_right_none() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        let new_tree = Tree::Item("-");

        assert!(location.insert_right(new_tree).is_none());
    }

    #[test]
    fn test_insert_right() {
        let result = Location {
            path: Path::Node {
                left: vec![],
                right: vec![Tree::Item("+"), Tree::Item("b")],
                path: Path::Top.into(),
            }
            .into(),
            cursor: Tree::Item("a"),
        }
        .insert_right(Tree::Item("."));

        let expect = Location {
            path: Path::Node {
                left: vec![],
                right: vec![Tree::Item("."), Tree::Item("+"), Tree::Item("b")],
                path: Path::Top.into(),
            }
            .into(),
            cursor: Tree::Item("a"),
        }
        .into();

        assert_eq!(result, expect);
    }

    #[test]
    fn test_insert_down() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        let new_tree = Tree::Item("-");
        let updated_location = location.insert_down(new_tree);

        assert_eq!(
            updated_location,
            Some(Location {
                cursor: Tree::Item("-"),
                path: Path::Node {
                    left: [].into(),
                    right: vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")],
                    path: crate::Path::Top.into(),
                }
                .into()
            })
        );
    }

    #[test]
    fn test_insert_down_none() {
        let location = Location {
            path: Path::Node {
                left: vec![Tree::Item("a")],
                right: vec![Tree::Item("b")],
                path: Path::Top.into(),
            }
            .into(),
            cursor: Tree::Item("+"),
        };

        let new_tree = Tree::Item("-");
        let updated_location = location.insert_down(new_tree);

        assert_eq!(updated_location, None);
    }

    #[test]
    fn test_delete_top() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        assert_eq!(location.delete(), None);
    }

    #[test]
    fn test_delete_middle_node() {
        let location = Location {
            path: Path::Node {
                left: vec![Tree::Item("+"), Tree::Item("a")],
                right: vec![],
                path: Path::Top.into(),
            }
            .into(),
            cursor: Tree::Item("b"),
        };

        let updated_location = location.delete();

        assert_eq!(
            updated_location,
            Some(Location {
                cursor: Tree::Item("+"),
                path: Path::Node {
                    left: [Tree::Item("a")].into(),
                    right: [].into(),
                    path: crate::Path::Top.into(),
                }
                .into()
            })
        );
    }

    #[test]
    fn test_delete_last_node() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        let updated_location = location.go_down().and_then(Location::delete);

        assert_eq!(
            updated_location,
            Some(Location {
                cursor: Tree::Item("+"),
                path: Path::Node {
                    right: [Tree::Item("b")].into(),
                    left: [].into(),
                    path: crate::Path::Top.into(),
                }
                .into()
            })
        );
    }

    #[test]
    fn test_delete_only_child() {
        let tree = Tree::Section(vec![Tree::Item("a")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };

        let updated_location = location.go_down().and_then(Location::delete);

        assert_eq!(
            updated_location,
            Some(Location {
                cursor: Tree::Section(vec![]),
                path: crate::Path::Top.into(),
            })
        );
    }

    #[test]
    fn test_memo_get_nth() {
        let tree = Tree::Section(vec![
            Tree::Item("a"),
            Tree::Item("+"),
            Tree::Item("b"),
            Tree::Item("*"),
            Tree::Item("c"),
        ]);

        let location = Location::new(tree);
        let memo_location = location.with_memo();

        // Should calculate and cache
        let first_access = memo_location.get_nth(2).unwrap();
        assert_eq!(first_access.into_inner().cursor, Tree::Item("b"));
    }

    #[test]
    fn test_memo_get_nth_cache_reuse() {
        let tree = Tree::Section(vec![
            Tree::Item("a"),
            Tree::Item("+"),
            Tree::Item("b"),
            Tree::Item("*"),
            Tree::Item("c"),
        ]);

        let location = Location::new(tree);
        let memo_location = location.with_memo();

        let memo_location = memo_location.get_nth(2).unwrap();

        // Should use cache
        let second_access = memo_location.get_nth(2).unwrap();
        assert_eq!(second_access.into_inner().cursor, Tree::Item("b"));
    }

    #[test]
    fn test_memo_get_nth_different_index() {
        let tree = Tree::Section(vec![
            Tree::Item("a"),
            Tree::Item("+"),
            Tree::Item("b"),
            Tree::Item("*"),
            Tree::Item("c"),
        ]);

        let location = Location::new(tree);
        let memo_location = location.with_memo();

        let diff_access = memo_location.get_nth(3).unwrap();
        assert_eq!(diff_access.into_inner().cursor, Tree::Item("*"));
    }

    #[test]
    fn test_memo_get_nth_out_of_bounds() {
        let tree = Tree::Section(vec![
            Tree::Item("a"),
            Tree::Item("+"),
            Tree::Item("b"),
        ]);

        let location = Location::new(tree);
        let memo_location = location.with_memo();

        assert!(memo_location.get_nth(5).is_none());
    }

    #[test]
    fn test_memo_get_nth_into_inner() {
        let tree = Tree::Section(vec![
            Tree::Item("a"),
            Tree::Item("+"),
            Tree::Item("b"),
        ]);

        let location = Location::new(tree.clone());
        let regular_location = location.get_nth(1).unwrap();

        let memo_location = Location::new(tree).with_memo();
        let memoized_inner_location = memo_location.get_nth(1).unwrap().into_inner();

        assert_eq!(memoized_inner_location.cursor, regular_location.cursor);
        assert_eq!(memoized_inner_location.cursor, Tree::Item("+"));
    }

    #[test]
    fn test_memo_get_nth_complex_navigation() {
        let tree = Tree::Section(vec![
            Tree::Item("a"),
            Tree::Section(vec![
                Tree::Item("b1"),
                Tree::Item("b2"),
                Tree::Item("b3"),
            ]),
            Tree::Item("c"),
        ]);

        let location = Location::new(tree);

        // Navigate to the Section, then memoize
        let memo_section = location.clone()
            .get_nth(1)
            .unwrap()
            .with_memo();

        let b1 = memo_section.get_nth(0).unwrap();
        assert_eq!(b1.location.cursor, Tree::Item("b1"));
    }

    #[test]
    fn test_memo_get_nth_nested_navigation() {
        let tree = Tree::Section(vec![
            Tree::Item("a"),
            Tree::Section(vec![
                Tree::Item("b1"),
                Tree::Item("b2"),
                Tree::Item("b3"),
            ]),
            Tree::Item("c"),
        ]);

        let location = Location::new(tree);
        let expected_b2 = location.clone()
            .get_nth(1).unwrap()
            .get_nth(1).unwrap();

        let memo_section = location
            .get_nth(1).unwrap()
            .with_memo();

        let b2 = memo_section.get_nth(1).unwrap();

        assert_eq!(b2.location.cursor, expected_b2.cursor);
        assert_eq!(b2.location.cursor, Tree::Item("b2"));
    }

    #[test]
    fn test_memo_get_nth_with_path() {
        let tree = Tree::Section(vec![Tree::Item("a"), Tree::Item("+"), Tree::Item("b")]);

        let location = Location {
            path: Path::Top.into(),
            cursor: tree,
        };
        let memo_location = location.clone().with_memo();

        let expected = location.get_nth(2);
        let memo_result = memo_location.get_nth(2).map(|loc| loc.into_inner());

        // Compare the full structure including path
        assert_eq!(memo_result, expected);

        // Compare path
        assert_eq!(
            memo_result,
            Some(Location {
                cursor: Tree::Item("b"),
                path: Path::Node {
                    left: vec![Tree::Item("+"), Tree::Item("a")],
                    right: vec![],
                    path: crate::Path::Top.into(),
                }
                    .into()
            })
        );
    }
}
