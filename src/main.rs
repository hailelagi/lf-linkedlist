pub mod fifth;
pub mod first;
pub mod fourth;
pub mod second;
pub mod third;

pub mod stacked_borrows;
pub mod weird_lists;

fn main() {
    // println!("lists-lists-lists!!!");
    // let mut first_list = first::List::new();

    // first_list.push(1);
    // first_list.push(2);
    // first_list.push(3);
    // first_list.push(4);

    // println!("{:#?}", first_list);

    stacked_borrows::pointer_mess();
    stacked_borrows::basic_borrow();
}
