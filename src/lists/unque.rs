use std::ptr;

type Link<T> = *mut Node<T>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

pub struct Que<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Que<T> {
    pub fn new() -> Self {
        Que {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn enque(&mut self, value: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                value,
                next: ptr::null_mut(),
            }));

            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }

            self.tail = new_tail;
        }
    }

    pub fn deque(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(head.value)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.value) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.value) }
    }
}

impl<T> Drop for Que<T> {
    fn drop(&mut self) {
        while self.deque().is_some() {}
    }
}

pub struct IntoIter<T>(Que<T>);

impl<T> IntoIterator for Que<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.deque()
    }
}


pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> Que<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }

    pub fn iter_mut(&self) -> IterMut<'_, T> {
        unsafe {
            IterMut {
                next: self.head.as_mut(),
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a Que<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Que<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.value
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.value
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enq_deq() {
        let mut q = Que::new();
        assert_eq!(q.deque(), None);

        q.enque(1);
        q.enque(2);
        q.enque(3);

        assert_eq!(q.deque(), Some(1));
        assert_eq!(q.deque(), Some(2));

        q.enque(4);
        q.enque(5);

        assert_eq!(q.deque(), Some(3));
        assert_eq!(q.deque(), Some(4));
        assert_eq!(q.deque(), Some(5));
        assert_eq!(q.deque(), None);

        q.enque(6);
        q.enque(7);
        assert_eq!(q.deque(), Some(6));
        assert_eq!(q.deque(), Some(7));
        assert_eq!(q.deque(), None);
    }

    #[test]
    fn into_iter() {
        let mut q = Que::new();
        q.enque(1);
        q.enque(2);
        q.enque(3);

        let mut iter = q.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut q = Que::new();
        q.enque(1);
        q.enque(2);
        q.enque(3);

        let mut iter = q.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut q = Que::new();
        q.enque(1);
        q.enque(2);
        q.enque(3);

        let mut iter = q.iter_mut();

        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn peek() {
        let mut q = Que::new();
        q.enque(1);
        q.enque(2);
        q.enque(3);

        assert_eq!(q.peek(), Some(&1));
        q.deque();
        assert_eq!(q.peek(), Some(&2));
        q.deque();
        assert_eq!(q.peek(), Some(&3));
        q.deque();
        assert_eq!(q.peek(), None);
    }

    #[test]
    fn peek_mut() {
        let mut q = Que::new();
        q.enque(1);
        q.enque(2);
        q.enque(3);

        assert_eq!(q.peek_mut(), Some(&mut 1));
        q.deque();
        assert_eq!(q.peek_mut(), Some(&mut 2));
        q.deque();
        assert_eq!(q.peek_mut(), Some(&mut 3));
        q.deque();
        assert_eq!(q.peek_mut(), None);
    }
}
