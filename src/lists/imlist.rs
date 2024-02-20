use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

pub struct ImList<T> {
    head: Link<T>,
    len: usize,
}

impl<T> ImList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.value)
    }

    pub fn tail(&self) -> Self {
        Self {
            head: self.head.as_ref().and_then(|head| head.next.clone()),
            len: if self.len > 0 { self.len - 1 } else { 0 },
        }
    }

    pub fn prepend(&self, value: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                value,
                next: self.head.clone(),
            })),
            len: self.len + 1,
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> ImList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

impl<T> Drop for ImList<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();

        while let Some(rc) = curr {
            if let Ok(mut node) = Rc::try_unwrap(rc) {
                curr = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepend() {
        let list = ImList::new().prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));
        assert_eq!(list.len(), 3);

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));
        assert_eq!(list.len(), 2);

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));
        assert_eq!(list.len(), 1);

        let list = list.tail();
        assert_eq!(list.head(), None);
        assert_eq!(list.len(), 0);

        let list = list.tail();
        assert_eq!(list.head(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn iter() {
        let list = ImList::new().prepend(1).prepend(2).prepend(3);
        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
