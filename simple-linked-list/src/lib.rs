use std::{iter::FromIterator, ptr::null};

pub struct Node<T>{

    data: Option<T>,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {

    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    length: usize,
    
}

impl<'a, T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: (None), tail: (None), length: (0) }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {

        match self.head {
            None => true,
            _ => false
        }

    }

    pub fn len(&self) -> usize {
        
        self.length

    }

    pub fn push(&mut self, _element: T) {

        let mut _new_node = Box::new(Node { data: (Some(_element)), next: (self.head.take())});

        self.head = Some(_new_node);

        self.length += 1;


    }

    pub fn pop(&mut self) -> Option<T> {

        if self.head.is_none() {
            return None;
        }

        let popped_node = self.head.take().unwrap();
        let _popped_data = popped_node.data.as_ref().unwrap();

        self.length -= 1;

        self.head = popped_node.next;
        return popped_node.data;
        

    }

    pub fn peek(&self) -> Option<&T> {

        if self.head.is_none() {
            return None;
        }

        let peeked = self.head.as_ref().unwrap();
        peeked.data.as_ref()
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<'a, T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
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

impl<'a, T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        unimplemented!()
    }
}
