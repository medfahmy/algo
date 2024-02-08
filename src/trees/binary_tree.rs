pub struct Node<T> {
    value: T,
    parent: Option<Box<Node<T>>>,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> where T: PartialEq + PartialOrd {
    pub fn new(value: T) -> Self {
        Node {
            value,
            parent: None,
            left: None,
            right: None,
        }
    }

    pub fn inorder_walk(&self) -> Vec<&T> {
        let mut v = Vec::new();

        if let Some(left) = &self.left {
            v.append(&mut left.inorder_walk());
        }

        v.push(&self.value);

        if let Some(right) = &self.right {
            v.append(&mut right.inorder_walk());
        }

        v
    }

    pub fn min_rec(&self) -> &T {
        if let Some(left) = &self.left {
            left.min_rec()
        } else {
            &self.value
        }
    }

    pub fn max_rec(&self) -> &T {
        if let Some(right) = &self.right {
            right.max_rec()
        } else {
            &self.value
        }
    }

    pub fn min_iter(&self) -> &T {
        let mut node = self;

        while let Some(left) = &node.left {
            node = left;
        }

        &node.value
    }

    pub fn max_iter(&self) -> &T {
        let mut node = self;

        while let Some(right) = &node.right {
            node = right;
        }

        &node.value
    }

    pub fn search_iter(&self, value: T) -> Option<&Node<T>> {
        let mut node = self;

        while node.value != value {
            if value < node.value {
                node = node.left.as_deref()?;
            } else {
                node = node.right.as_deref()?;
            }
        }

        Some(node)
    }

    pub fn search_rec(&self, value: T) -> Option<&Node<T>> {
        if value == self.value {
            Some(self)
        } else if value < self.value {
            self.left.as_ref().map(|left| left.search_rec(value))?
        } else {
            self.right.as_ref().map(|right| right.search_rec(value))?
        }
    }

    pub fn succ(&self) -> &T {
        todo!()
        // if let Some(right) = self.right {
        //     right.min_iter()
        // } else {
        //     let mut node = self.parent;
        //     while let Some(parent) = node && self
        // }
    }

    pub fn pred(&self) {}
}

impl<T> From<&[T]> for Node<T> {
    fn from(value: &[T]) -> Node<T> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_tree() -> Node<usize> {
        let mut root = Node::new(6);

        let mut left = Node::new(4);
        let left_left = Box::new(Node::new(2));
        let left_right = Box::new(Node::new(5));
        left.left = Some(left_left);
        left.right = Some(left_right);

        let mut right = Node::new(7);
        right.right = Some(Box::new(Node::new(8)));

        root.left = Some(Box::new(left));
        root.right = Some(Box::new(right));

        root
    }

    #[test]
    fn inorder() {
        let root = new_tree();
        assert_eq!(root.inorder_walk(), [&2, &4, &5, &6, &7, &8]);
    }

    #[test]
    fn search() {
        let root = new_tree();
        let node = root.search_iter(4).unwrap();
        assert_eq!(node.left.as_ref().unwrap().value, 2);
        assert_eq!(node.right.as_ref().unwrap().value, 5);

        let node = root.search_rec(4).unwrap();
        assert_eq!(node.left.as_ref().unwrap().value, 2);
        assert_eq!(node.right.as_ref().unwrap().value, 5);
    }

    #[test]
    fn min_max() {
        let root = new_tree();

        assert_eq!(root.min_rec(), &2);
        assert_eq!(root.max_rec(), &8);

        assert_eq!(root.min_iter(), &2);
        assert_eq!(root.max_iter(), &8);
    }
}
