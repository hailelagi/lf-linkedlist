// first.rs
// List a = Empty | Elem a (List a)

/**
* We're allocating a node that just says "I'm not actually a Node"
* One of our nodes isn't heap-allocated at all.
* numa-awareness
// [node, ptr] -> [node, ptr] -> [node, ptr] -> [node, ptr]
**/
/* list.rs
pub enum List {
    Empty,
    Elem(Box<List>),
}
*/

/*
pub enum TwoList {
    Empty,
    ElemThenEmpty(i32),
    ElemThenNotEmpty(i32, Box<List>),
}
*/
use std::mem;

#[derive(Debug, Default)]
enum Link {
    #[default]
    Empty,
    More(Box<Node>),
}
 
#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(Debug)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn peek(&self) -> Option<i32> {
        match &self.head {
            Link::Empty => None,
            Link::More(item) => Some(item.elem)
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::take(&mut self.head),
        });

        self.head = Link::More(new_node);
    }

    fn pop(&mut self) -> Option<()> {
        match self.head {
            Link::Empty => todo!(),
            Link::More(_) => todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list() {
        let list = List::new();
        assert!(matches!(list, List { head: Link::Empty }));
    }

    #[test]
    fn test_push_list() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        if let Link::More(node) = &list.head {
            assert_eq!(node.elem, 4);
            if let Link::More(node) = &node.next {
                assert_eq!(node.elem, 3);
                if let Link::More(node) = &node.next {
                    assert_eq!(node.elem, 2);
                    if let Link::More(node) = &node.next {
                        assert_eq!(node.elem, 1);
                        assert!(matches!(node.next, Link::Empty));
                    } else {
                        panic!("Expected node with elem 1");
                    }
                } else {
                    panic!("Expected node with elem 2");
                }
            } else {
                panic!("Expected node with elem 3");
            }
        } else {
            panic!("Expected node with elem 4");
        }
    }
}
