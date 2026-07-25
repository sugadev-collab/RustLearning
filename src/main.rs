struct OwnedBook {
    title: String,
}

impl OwnedBook {
    fn new(title: String) -> Self {
        Self { title }
    }
}

struct BorrowedBook<'a> {
    title: &'a str,
}

impl<'a> BorrowedBook<'a> {
    fn new(title: &'a str) -> Self {
        Self { title }
    }
}

fn main() {
    let sample_title = "Harry Spotter 8".to_string();
    let owned = OwnedBook::new(sample_title); // here the ownership passed

    let borrowed = BorrowedBook::new(&owned.title); // here the borrows happens
    println!("Borrowed: {}", borrowed.title);
}
