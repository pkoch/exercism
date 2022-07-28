use std::{iter::FromIterator, mem};

struct Node<T> {
    element: T,
    prev: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    tail: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { tail: None }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.tail, None)
    }

    pub fn len(&self) -> usize {
        let mut res = 0;
        let mut curr_node = &self.tail;
        while let Some(prev_node) = curr_node {
            res += 1;
            curr_node = &prev_node.prev;
        }
        res
    }

    pub fn push(&mut self, element: T) {
        self.tail = Some(Box::new(Node {
            element,
            prev: mem::take(&mut self.tail),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::take(&mut self.tail) {
            None => None,
            Some(old_tail_node) => {
                self.tail = old_tail_node.prev;
                Some(old_tail_node.element)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.tail.as_deref().map(|n| &n.element)
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
        iter.into_iter().fold(SimpleLinkedList::new(), |mut a, i| {
            a.push(i);
            a
        })
    }
}

impl<T> IntoIterator for SimpleLinkedList<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let v = self.rev();
        IntoIter{node: v.tail}
    }
}

pub struct IntoIter<T> {
    node: Option<Box<Node<T>>>
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let old_node = mem::take(&mut self.node);
        if let None = old_node {
            return None;
        }
        let old_box = old_node.unwrap();
        let res = Some(old_box.element);
        self.node = old_box.prev;
        res
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(ll: SimpleLinkedList<T>) -> Self {
        ll.into_iter().collect()
    }
}
