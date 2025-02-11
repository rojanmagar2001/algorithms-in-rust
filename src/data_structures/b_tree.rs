use std::fmt::Debug;

struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
}

pub struct BTree<T> {
    root: Node<T>,
    props: BTreeProps,
}

struct BTreeProps {
    degree: usize,
    max_keys: usize,
    mid_key_index: usize,
}

impl<T> Node<T>
where
    T: Ord,
{
    fn new(degree: usize, keys: Option<Vec<T>>, children: Option<Vec<Node<T>>>) -> Self {
        Self {
            keys: match keys {
                Some(keys) => keys,
                None => Vec::with_capacity(degree - 1),
            },
            children: match children {
                Some(children) => children,
                None => Vec::with_capacity(degree),
            },
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl BTreeProps {
    pub fn new(degree: usize) -> Self {
        Self {
            degree,
            max_keys: degree - 1,
            mid_key_index: (degree - 1) / 2,
        }
    }
}

impl<T> BTree<T>
where
    T: Ord + Copy + Debug + Default,
{
    pub fn new(branch_factor: usize) -> Self {
        let degree = 2 * branch_factor;
        Self {
            root: Node::new(degree, None, None),
            props: BTreeProps::new(degree),
        }
    }
}
