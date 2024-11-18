use first::List;

pub mod first;
pub mod second;
pub mod weird_lists;

fn main() {
    println!("lists-lists-lists!!!");
    let mut list = List::new();

    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);


    println!("{:#?}", list);
}
