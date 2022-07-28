use std::{iter::FromIterator, mem};

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

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
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
        self.head = Some(Box::new(Node {
            element,
            next: mem::take(&mut self.head),
        }));
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
        IntoIter{node: v.head}
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
        self.node = old_box.next;
        res
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(ll: SimpleLinkedList<T>) -> Self {
        ll.into_iter().collect()
    }
}
