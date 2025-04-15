use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T: Clone> {
    Item(T),
    Section(Vec<Tree<T>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Path<T: Clone> {
    Top,
    Node {
        left: Vec<Tree<T>>,
        right: Vec<Tree<T>>,
        path: Rc<Path<T>>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Location<T: Clone> {
    pub cursor: Tree<T>,
    pub path: Rc<Path<T>>,
}

impl<T: Clone> Location<T> {
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

    pub fn get_nth(self, n: usize) -> Option<Self> {
        match n {
            0 => self.go_down(),
            n => self.get_nth(n - 1).and_then(Location::go_right),
        }
    }

    pub fn change(self, tree: Tree<T>) -> Self {
        Self {
            cursor: tree,
            path: self.path,
        }
    }

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
}
