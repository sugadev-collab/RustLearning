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

    {
        let preview = BookPreview::new(&book_title, &book_author);

        println!(
            "Book title: {}, Book author: {}",
            preview.title, preview.author
        );
    } // the preview's scope ends when the helper block ends, bcs the stack based ownership ends. and also the preview insctence in stack so (stack owner and stack instence are same right? and i had this qustion before, and in heap the owner is just a pointer and the value on heap but instck the owner directly hold the value right?) it get dropped in the end of thew scope
    //so this helper block can be usefull from accedently using the preview agin after moved right?

    let move_title = book_title;

    println!("Moved title: {}", move_title); // this is allowed bcs of compiler's smart feature , lexical lifetime
    // when the preview instence lastly used it will be dropped from that if not used again, so we are allowed to move the raw title bcs there is no borrow is happen to that title when i moved the title.
}
