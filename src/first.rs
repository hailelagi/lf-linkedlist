// first.rs
// List a = Empty | Elem a (List a)

// [node, ptr] -> [node, ptr] -> [node, ptr] -> [node, ptr]

#[derive(Debug)]
pub struct Node {
    pub elem: i32,
    pub next: List,
}

#[derive(Debug)]
pub enum List {
    Empty,
    More(Box<Node>),
}
