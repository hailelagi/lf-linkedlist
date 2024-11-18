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

impl Drop for List {
    fn drop(&mut self) {
        /*
        // NOTE: you can't actually explicitly call `drop` in real Rust code;
        // we're pretending to be the compiler!
        self.head.drop(); // tail recursive - good!
        */

        let mut cur_link = mem::take(&mut self.head);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::take(&mut boxed_node.next);
        }
    }
}

/*
impl Drop for Link {
    fn drop(&mut self) {
        match *self {
            Link::Empty => {} // Done!
            Link::More(ref mut boxed_node) => {
                boxed_node.drop(); // tail recursive - good!
            }
        }
    }
}

impl Drop for Box<Node> {
    fn drop(&mut self) {
        self.ptr.drop(); // uh oh, not tail recursive!
        deallocate(self.ptr);
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.next.drop();
    }
}

*/

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
            Link::More(item) => Some(item.elem),
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::take(&mut self.head),
        });

        self.head = Link::More(new_node);
    }

    fn pop(&mut self) -> Option<i32> {
        match mem::take(&mut self.head) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_new_list() {
        let list = List::new();
        assert!(matches!(list, List { head: _ }));
    }

    #[test]
    fn test_push_n_pop_list() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));

        list.push(5);

        assert_eq!(list.pop(), Some(5));
    }
}
