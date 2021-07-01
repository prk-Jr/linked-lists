#[allow(unused_imports)]
use std::{mem, ptr::replace};
pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        /*   match self.head.take() {
            None => None,
            Some(node) => {
                let x = node.as_ref().elem;
                self.head = node.next;
                // node.drop();
                Some(x)
            }
        } */

        self.head.take().map(|x| {
            self.head = x.next;
            x.elem
        })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn second() {
        println!("Second.rs");
        use super::List;
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
