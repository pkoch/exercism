use std::{iter::FromIterator, mem};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn len(&self) -> usize {
        1 + match self.next {
            None => 0,
            Some(ref next) => next.len(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self{head: None}
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.head, None)
    }

    pub fn len(&self) -> usize {
        match self.head {
            None => 0,
            Some(ref node) => node.len(),
        }
    }

    pub fn push(&mut self, element: T) {
        let old_head = mem::take(&mut self.head);
        self.head = Some(Box::new(Node{element, next: old_head}));
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::take(&mut self.head) {
            None => None,
            Some(old_head_node) => {
                self.head = old_head_node.next;
                Some(old_head_node.element)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|n| &n.element)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut old = self;
        let mut new = Self::new();
        while !old.is_empty() {
            new.push(old.pop().unwrap())
        }
        new
    }
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(SimpleLinkedList::new(), |mut a, i| {a.push(i); a})
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut new = Vec::with_capacity(linked_list.len());
        while !linked_list.is_empty() {
            new.insert(0, linked_list.pop().unwrap())
        };
        new
    }
}
