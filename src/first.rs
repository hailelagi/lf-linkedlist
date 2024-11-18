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

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}
