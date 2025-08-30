use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }
    pub fn push_front(&mut self, elem:T)
    {
        let new_head = Node::new(elem);
        

    }
}


