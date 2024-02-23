use std::marker::PhantomData;
use std::ptr::NonNull;

type Link<T> = Option<NonNull<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

#[derive(Debug)]
pub struct DList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    phantom: PhantomData<T>,
}

impl<T> DList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        while self.pop_head().is_some() {}
    }

    pub fn cursor_mut(&mut self) -> CursorMut<'_, T> {
        CursorMut {
            list: self,
            curr: None,
            index: None,
        }
    }

    pub fn push_head(&mut self, value: T) {
        unsafe {
            let new_head = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                value,
                next: None,
                prev: None,
            })));

            if let Some(old_head) = self.head {
                (*old_head.as_ptr()).prev = Some(new_head);
                (*new_head.as_ptr()).next = Some(old_head);
            } else {
                self.tail = Some(new_head);
            }

            self.head = Some(new_head);
            self.len += 1;
        }
    }

    pub fn pop_head(&mut self) -> Option<T> {
        unsafe {
            self.head.map(|head| {
                let boxed_head = Box::from_raw(head.as_ptr());
                let value = boxed_head.value;

                self.head = boxed_head.next;

                if let Some(new_head) = self.head {
                    (*new_head.as_ptr()).prev = None;
                } else {
                    self.tail = None;
                }

                self.len -= 1;
                value
            })
        }
    }

    pub fn push_tail(&mut self, value: T) {
        unsafe {
            let new_tail = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                value,
                next: None,
                prev: None,
            })));

            if let Some(old_tail) = self.tail {
                (*old_tail.as_ptr()).next = Some(new_tail);
                (*new_tail.as_ptr()).prev = Some(old_tail);
            } else {
                self.head = Some(new_tail);
            }

            self.tail = Some(new_tail);
            self.len += 1;
        }
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        unsafe {
            self.tail.map(|tail| {
                let boxed_tail = Box::from_raw(tail.as_ptr());
                let value = boxed_tail.value;

                self.tail = boxed_tail.prev;

                if let Some(new_tail) = self.tail {
                    (*new_tail.as_ptr()).next = None;
                } else {
                    self.head = None;
                }

                self.len -= 1;
                value
            })
        }
    }

    pub fn head(&self) -> Option<&T> {
        unsafe { self.head.map(|head| &(*head.as_ptr()).value) }
    }

    pub fn head_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.map(|head| &mut (*head.as_ptr()).value) }
    }

    pub fn tail(&self) -> Option<&T> {
        unsafe { self.tail.map(|tail| &(*tail.as_ptr()).value) }
    }

    pub fn tail_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.map(|tail| &mut (*tail.as_ptr()).value) }
    }
}

pub struct CursorMut<'a, T> {
    curr: Link<T>,
    list: &'a mut DList<T>,
    index: Option<usize>,
}

impl<'a, T> CursorMut<'a, T> {
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    pub fn move_next(&mut self) {
        if let Some(curr) = self.curr {
            unsafe {
                self.curr = (*curr.as_ptr()).next;

                if self.curr.is_some() {
                    *self.index.as_mut().unwrap() += 1;
                } else {
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            self.curr = self.list.head;
            self.index = Some(0);
        }
    }

    pub fn move_prev(&mut self) {
        if let Some(curr) = self.curr {
            unsafe {
                self.curr = (*curr.as_ptr()).prev;

                if self.curr.is_some() {
                    *self.index.as_mut().unwrap() -= 1;
                } else {
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            self.curr = self.list.tail;
            self.index = Some(self.list.len() - 1);
        }
    }

    pub fn current(&mut self) -> Option<&mut T> {
        unsafe { self.curr.map(|node| &mut (*node.as_ptr()).value) }
    }

    pub fn peek_next(&mut self) -> Option<&mut T> {
        unsafe {
            self.curr
                .and_then(|node| (*node.as_ptr()).next)
                .map(|next| &mut (*next.as_ptr()).value)
        }
    }

    pub fn peek_prev(&mut self) -> Option<&mut T> {
        unsafe {
            self.curr
                .and_then(|node| (*node.as_ptr()).prev)
                .map(|prev| &mut (*prev.as_ptr()).value)
        }
    }

    pub fn split_before(&mut self) -> DList<T> {
        if let Some(curr) = self.curr {
            unsafe {
                let old_len = self.list.len;
                let old_idx = self.index.unwrap();
                let prev = (*curr.as_ptr()).prev;

                let new_len = old_len - old_idx;
                let new_head = self.curr;
                let new_tail = self.list.tail;
                let new_idx = Some(0);

                let output_len = old_idx;
                let output_head = self.list.head;
                let output_tail = prev;

                if let Some(prev) = prev {
                    (*curr.as_ptr()).prev = None;
                    (*prev.as_ptr()).next = None;
                }

                self.list.len = new_len;
                self.list.head = new_head;
                self.list.tail = new_tail;
                self.index = new_idx;

                DList {
                    head: output_head,
                    tail: output_tail,
                    len: output_len,
                    phantom: PhantomData,
                }
            }
        } else {
            std::mem::replace(self.list, DList::new())
        }
    }

    pub fn split_after(&mut self) -> DList<T> {
        if let Some(curr) = self.curr {
            unsafe {
                let old_len = self.list.len;
                let old_idx = self.index.unwrap();
                let next = (*curr.as_ptr()).next;

                let new_len = old_idx;
                let new_head = self.list.head;
                let new_tail = self.curr;
                let new_idx = Some(old_idx);

                let output_len = old_len - old_idx;
                let output_head = next;
                let output_tail = self.list.tail;

                if let Some(next) = next {
                    (*curr.as_ptr()).next = None;
                    (*next.as_ptr()).prev = None;
                }

                self.list.len = new_len;
                self.list.head = new_head;
                self.list.tail = new_tail;
                self.index = new_idx;

                DList {
                    head: output_head,
                    tail: output_tail,
                    len: output_len,
                    phantom: PhantomData,
                }
            }
        } else {
            std::mem::replace(self.list, DList::new())
        }
    }

    pub fn splice_before(&mut self, mut input: DList<T>) {
        unsafe {
            if input.is_empty() {
                return;
            } else if let Some(curr) = self.curr {
                if let Some(0) = self.index {
                    (*curr.as_ptr()).prev = input.tail.take();
                    (*input.tail.unwrap().as_ptr()).next = Some(curr);
                    self.list.head = input.head.take();

                    *self.index.as_mut().unwrap() += input.len;
                    input.len = 0;
                } else {
                }
            } else {
                *self.list = input;
            }
        }
    }
}

impl<T> Drop for DList<T> {
    fn drop(&mut self) {
        while self.pop_head().is_some() {}
    }
}

pub struct IntoIter<T>(DList<T>);

impl<T> IntoIterator for DList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_head()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len, Some(self.0.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_tail()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.0.len
    }
}

pub struct Iter<'a, T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    phantom: PhantomData<&'a T>,
}

impl<T> DList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> IntoIterator for &'a DList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.head.map(|head| unsafe {
                self.len -= 1;
                self.head = (*head.as_ptr()).next;
                &(*head.as_ptr()).value
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.tail.map(|tail| unsafe {
                self.len -= 1;
                self.tail = (*tail.as_ptr()).prev;
                &(*tail.as_ptr()).value
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

pub struct IterMut<'a, T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    phantom: PhantomData<&'a mut T>,
}

impl<T> DList<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> IntoIterator for &'a mut DList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.head.map(|head| unsafe {
                self.len -= 1;
                self.head = (*head.as_ptr()).next;
                &mut (*head.as_ptr()).value
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.tail.map(|tail| unsafe {
                self.len -= 1;
                self.tail = (*tail.as_ptr()).prev;
                &mut (*tail.as_ptr()).value
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop_head() {
        let mut list = DList::new();
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_head(), None);
        assert_eq!(list.len(), 0);

        list.push_head(1);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_head(), Some(1));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_head(), None);
        assert_eq!(list.len(), 0);

        list.push_head(2);
        list.push_head(3);

        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_head(), Some(3));
        assert_eq!(list.pop_head(), Some(2));

        list.push_head(4);
        list.push_head(5);

        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_head(), Some(5));
        assert_eq!(list.pop_head(), Some(4));
        assert_eq!(list.pop_head(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn push_pop_tail() {
        let mut list = DList::new();
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_tail(), None);
        assert_eq!(list.len(), 0);

        list.push_tail(1);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_tail(), Some(1));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_tail(), None);
        assert_eq!(list.len(), 0);

        list.push_tail(2);
        list.push_tail(3);

        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_tail(), Some(3));
        assert_eq!(list.pop_tail(), Some(2));

        list.push_tail(4);
        list.push_tail(5);

        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_tail(), Some(5));
        assert_eq!(list.pop_tail(), Some(4));
        assert_eq!(list.pop_tail(), None);
        assert_eq!(list.len(), 0);
    }

    // #[test]
    // fn peek_head() {
    //     let mut list = DList::new();
    //     assert!(list.peek_head().is_none());
    //
    //     list.push_head(1);
    //     list.push_head(2);
    //     assert_eq!(&*list.peek_head().unwrap(), &2);
    // }
}
