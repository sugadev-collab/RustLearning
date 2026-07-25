struct BookPreview<'a> {
    title: &'a str,
    author: &'a str,
}

impl<'a> BookPreview<'a> {
    fn new(title: &'a str, author: &'a str) -> Self {
        Self { title, author }
    }
}

fn main() {
    let book_title = "Harry spotter".to_string();
    let book_author = "Gym Bros".to_string();

    let preview = BookPreview::new(&book_title, &book_author);

    println!(
        "Book title: {}, Book author: {}",
        preview.title, preview.author
    );

    let move_title = book_title;

    println!("Moved title: {}", move_title); // this is allowed bcs of compiler's smart feature , lexical lifetime
    // when the preview instence lastly used it will be dropped from that if not used again, so we are allowed to move the raw title bcs there is no borrow is happen to that title when i moved the title.
}
