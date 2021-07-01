use std::{cell::RefCell, rc::Rc};

type ListType<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Start<T> {
    pub length: u32,
    head: ListType<T>,
    tail: ListType<T>,
}

impl<T> Start<T> {
    #[allow(dead_code)]
    fn empty() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }
    #[allow(dead_code)]

    fn push(&mut self, value: T) {
        let new_node = Node::new_node(value);
        let new = new_node.clone();
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new_node),
            None => self.head = Some(new_node),
        }

        self.length += 1;
        self.tail = Some(new);
    }
    #[allow(dead_code)]

    fn pop(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
        match self.head.take() {
            Some(old) => {
                self.head = old.borrow_mut().next.take();
                self.length -= 1;
                if self.head.is_none() {
                    self.tail = None;
                }
                Some(old)
            }
            None => {
                self.tail = None;
                None
            }
        }
        /*  self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
                // Some(head)
            } else {
                self.tail.take();
                // None
            }
            self.length -= 1;
            return head;
        }) */
    }
}

#[derive(Debug)]

struct Node<T> {
    value: T,
    next: ListType<T>,
}

impl<T> Node<T> {
    fn new_node(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Self {
            value: value,
            next: None,
        }))
    }
}

mod test {
    #[test]
    fn second() {
        println!("Second.rs");
        use super::Start;
        let mut list = Start::empty();
        list.pop();
        println!("list pop: {:#?}", list);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        println!("list push 1,2,3: {:#?}", list);

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        println!("list push 4,5: {:#?}", list);
        list.pop();
        println!("list pop: {:#?}", list);
        list.pop();
        println!("list pop: {:#?}", list);
        list.pop();
        println!("list pop: {:#?}", list);
        list.pop();
        println!("list pop: {:#?}", list);
        list.pop();
        println!("list pop: {:#?}", list);
    }
}
