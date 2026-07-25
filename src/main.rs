struct OwnedBook {
    title: String,
}

impl OwnedBook {
    fn new(title: String) -> Self {
        Self { title }
    }

    fn preview(&self) -> BorrowedBook<'_> {
        BorrowedBook::new(&self.title)
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
    let owned = OwnedBook::new(sample_title);

    let borrowed = owned.preview(); // the preview borrows from self so we dont need to explicitly pass the owned object's reference, insted for easy we pass the instence's self reference to create the preview, and its like java this pointer
    println!("Borrowed: {}", borrowed.title);
}
