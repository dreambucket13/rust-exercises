use std::{iter::FromIterator};

#[derive(Clone)]
struct Node<T: Clone + Copy> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T: Clone + Copy> {

    head: Option<Box<Node<T>>>,
    length: usize,
    
}

impl<T: Clone + Copy> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: (None), length: (0) }
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

        let mut _new_node = Box::new(Node { data: (_element), next: (self.head.take())});

        self.head = Some(_new_node);

        self.length += 1;


    }

    pub fn pop(&mut self) -> Option<T> {

        if self.head.is_none() {
            return None;
        }

        let popped_node = self.head.take().unwrap();

        self.length -= 1;

        self.head = popped_node.next;
        
        Some(popped_node.data)
    }

    pub fn peek(&self) -> Option<&T> {

        if self.head.is_none() {
            return None;
        }

        let peeked = self.head.as_ref().unwrap();
        let peeked_data = &peeked.as_ref().data;
        Some(peeked_data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        
        let mut reversed = SimpleLinkedList::new();

        //we borrow a reference to the head node and then clone elements.   
        //we don't want to destroy the original linked list by using pop()

        let mut node = &self.head;

        while node.is_some() {

            let element = node.as_ref().unwrap().data.clone();
            reversed.push(element);
            node = &node.as_ref().unwrap().next;
            
        }

        reversed

    }
}

impl<T: Clone + Copy> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {

        let mut list = SimpleLinkedList::new();

        for element in _iter {
            list.push(element);
        }

        list
        
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

impl<T: Clone + Copy> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(_linked_list: SimpleLinkedList<T>) -> Vec<T> {
        
        let mut list = Vec::new();

        //we borrow a reference to the head node and then clone elements.   
        //we don't want to destroy the original linked list by using pop()

        let mut node = &_linked_list.head;

        while node.is_some() {

            let element = node.as_ref().unwrap().data.clone();
            list.insert(0, element);
            node = &node.as_ref().unwrap().next;
            
        }

        list

    }
}
