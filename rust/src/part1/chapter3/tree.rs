use std::cmp::max;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
enum Tree<A> {
    Leaf {
        value: A,
    },
    Branch {
        left: Rc<Tree<A>>,
        right: Rc<Tree<A>>,
    },
}

impl<A> Tree<A>
where
    A: Clone,
{
    fn size(&self) -> usize {
        match self {
            Tree::Leaf { .. } => 1,
            Tree::Branch { left, right } => 1 + left.size() + right.size(),
        }
    }

    fn depth(&self) -> usize {
        match self {
            Tree::Leaf { .. } => 0,
            Tree::Branch { left, right } => 1 + max(left.depth(), right.depth()),
        }
    }

    fn map<B, F>(&self, f: F) -> Tree<B>
    where
        B: Clone,
        F: Fn(A) -> B + Copy,
    {
        match self {
            Tree::Leaf { value } => Tree::Leaf {
                value: f(value.clone()),
            },
            Tree::Branch { left, right } => Tree::Branch {
                left: Rc::new(left.map(f)),
                right: Rc::new(right.map(f)),
            },
        }
    }

    fn fold<B, F, G>(&self, f: F, g: G) -> B
    where
        F: Fn(A) -> B + Copy,
        G: Fn(B, B) -> B + Copy,
    {
        match self {
            Tree::Leaf { value } => f(value.clone()),
            Tree::Branch { left, right } => g(left.fold(f, g), right.fold(f, g)),
        }
    }

    fn size_via_fold(&self) -> usize {
        self.fold(|_| 1, |x, y| 1 + x + y)
    }

    fn depth_via_fold(&self) -> usize {
        self.fold(|_| 0, |left, right| 1 + max(left, right))
    }

    fn map_via_fold<B, F>(&self, f: F) -> Tree<B>
    where
        F: Fn(A) -> B + Copy,
    {
        self.fold(
            |value| Tree::Leaf { value: f(value) },
            |left, right| Tree::Branch {
                left: Rc::new(left),
                right: Rc::new(right),
            },
        )
    }
}

impl Tree<i32> {
    fn maximum(&self) -> i32 {
        match self {
            Tree::Leaf { value } => value.clone(),
            Tree::Branch { left, right } => max(left.maximum(), right.maximum()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise325() {
        let tree = Tree::Branch {
            left: Rc::new(Tree::Branch {
                left: Rc::new(Tree::Leaf { value: 1 }),
                right: Rc::new(Tree::Leaf { value: 2 }),
            }),
            right: Rc::new(Tree::Branch {
                left: Rc::new(Tree::Leaf { value: 3 }),
                right: Rc::new(Tree::Leaf { value: 4 }),
            }),
        };
        assert_eq!(tree.maximum(), 4);
    }

    #[test]
    fn test_exercise326() {
        let tree = Tree::Branch {
            left: Rc::new(Tree::Leaf { value: 1 }),
            right: Rc::new(Tree::Branch {
                left: Rc::new(Tree::Leaf { value: 2 }),
                right: Rc::new(Tree::Branch {
                    left: Rc::new(Tree::Leaf { value: 3 }),
                    right: Rc::new(Tree::Leaf { value: 3 }),
                }),
            }),
        };
        assert_eq!(tree.depth(), 3);
    }

    #[test]
    fn test_exercise327() {
        let tree = Tree::Branch {
            left: Rc::new(Tree::Leaf { value: 1 }),
            right: Rc::new(Tree::Branch {
                left: Rc::new(Tree::Leaf { value: 2 }),
                right: Rc::new(Tree::Branch {
                    left: Rc::new(Tree::Leaf { value: 3 }),
                    right: Rc::new(Tree::Leaf { value: 3 }),
                }),
            }),
        };
        let expect = Tree::Branch {
            left: Rc::new(Tree::Leaf {
                value: "1".to_string(),
            }),
            right: Rc::new(Tree::Branch {
                left: Rc::new(Tree::Leaf {
                    value: "2".to_string(),
                }),
                right: Rc::new(Tree::Branch {
                    left: Rc::new(Tree::Leaf {
                        value: "3".to_string(),
                    }),
                    right: Rc::new(Tree::Leaf {
                        value: "3".to_string(),
                    }),
                }),
            }),
        };
        assert_eq!(tree.map(|x| x.to_string()), expect);
    }

    #[test]
    fn test_exercise328() {
        let tree = Tree::Branch {
            left: Rc::new(Tree::Leaf { value: 1 }),
            right: Rc::new(Tree::Branch {
                left: Rc::new(Tree::Leaf { value: 2 }),
                right: Rc::new(Tree::Branch {
                    left: Rc::new(Tree::Leaf { value: 3 }),
                    right: Rc::new(Tree::Leaf { value: 3 }),
                }),
            }),
        };
        assert_eq!(tree.size_via_fold(), 7);
        assert_eq!(tree.depth_via_fold(), 3);
        let expect = Tree::Branch {
            left: Rc::new(Tree::Leaf {
                value: "1".to_string(),
            }),
            right: Rc::new(Tree::Branch {
                left: Rc::new(Tree::Leaf {
                    value: "2".to_string(),
                }),
                right: Rc::new(Tree::Branch {
                    left: Rc::new(Tree::Leaf {
                        value: "3".to_string(),
                    }),
                    right: Rc::new(Tree::Leaf {
                        value: "3".to_string(),
                    }),
                }),
            }),
        };
        assert_eq!(tree.map_via_fold(|x| x.to_string()), expect);
    }
}
