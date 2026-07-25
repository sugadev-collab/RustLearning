//Part 1
#[derive(Debug)]
struct Note {
    //Note's instence own the title and value, and titile and value owns their own strings
    title: String,
    body: String,
}

impl Note {
    fn new(title: String, body: String) -> Self {
        return Self { title, body };
    }
}

// part_2
struct Preview<'a> {
    title: &'a str,
    first_word: &'a str,
}

// impl Preview<'static> {
//     fn new(title: &'static str, first_word: &'static str) -> Self {
//         return Self { title, first_word };
//     }
// }

// fn main() {
//     // let text = String::from("Rust");

//     // let holder = NameHolder { name: &text }; // ok i got it the &String -> &str is rust's normal behaviour and it called coercion, and you told that early.
//     // //so here we get the actual utf-8 bytes from coercian right?

//     // println!("Holder name: {}", holder.name);

//     //part_1
//     let  note_1 =  Note::new(
//     "Death Note".to_string(),
//     "Death Note is a dark thriller anime abouta smart high school student who finds a magic notebook that kills anyone whose name is written in it, leading to a dangerous game of cat-and-mouse with a genius detective.".to_string()
//     );

//     println!("...........Part 1............");
//     println!("Note: {}", note_1.title);
//     println!("Summary: {}", note_1.body);

//     //part_2

//     // let preview = Preview::new(&note_1.title, &note_1.body[0..31]); here i cant create the preview value with borrowed value from note_1
//     //bcs the preview's properties need static string variables so the borrowed owner should be static

//     // static  title_from_note = note_1.title.clone(); //this also didnt works

//     let title_ = "Rust";

//     // let mut body_from_note= String::new().as_mut_str();
//     //   note_1.body[0..31].clone_into(&mut body_from_note.to_string());  //this also didnt works

//     //so i just use normal text

//     let first_word_ = "Rust language is fast";

//     let preview = Preview::new(title_, first_word_); //Preview<'a> borrows from two variables title_, first_word_ and it borrows a static str variable.
//     //Preview needs lifetime bcs without it compiler dont know how long the borrowed str value can last without get droped.

//     println!("...........Part 2............");
//     println!("Title: {}", preview.title);
//     println!("First word: {}", preview.first_word);
// }

// above i keep the mistake i made
impl<'a> Preview<'a> {
    fn new(title: &'a str, first_word: &'a str) -> Self {
        Self { title, first_word }
    }
}

#[derive(Debug)]
struct NameHolder<'a> {
    name: &'a str,
}

impl<'a> NameHolder<'a> {
    fn new(name: &'a str) -> Self {
        Self { name }
    }
}

fn main() {
    println!("...............Part 1...............");
    println!();
    let mut note = Note::new("Rust".to_string(), "Rust language is fast".to_string());

    println!("Title: {}, Note: {}", note.title, note.body); //title and body's bytes are owned by the note's member variables.

    println!();
    println!("...............Part 2...............");
    println!();

    let preview = Preview::new(&note.title, &note.body[0..12]); // here we gets a &str slice reference fo title and body

    println!(
        "Title: {}, first_word: {}",
        preview.title, preview.first_word
    ); // without a lifetime the rust complier couldnt know 
    //the &str's underlaying bytes life time at compiletime, so rust need to know that at compiletime. so it can garentee the &str reference wont point to an erased memory address in its lifetime

    println!();
    println!("...............Part 3...............");
    println!();

    note.body.replace_range(0..4, "C++");

    println!("Muted body: {}", note.body);

    // println!(                               // invalid bcs only one mutable or unlimited unmutable reference can be in the same time,
    //     "Title: {}, first_word: {}",        // and the bcs immutable value of the note live inside preview if i use this, without using it the lexical lifetime ends for thr preview
    //     preview.title, preview.first_word   // usind the preview here done not end the lifetime of preview after borrowing mutable reference
    // );

    println!();
    println!("...............Part 4...............");
    println!();

    let text = String::from("🤖 is helping me to learn rust.");

    println!("Length of text: {}", text.len());
    println!("Char count of text: {}", text.chars().count());
    println!();
    println!("Chars.");
    text.chars().enumerate().for_each(|(i, value)| {
        print!(" index: {} Value: {},", i, value);
    });

    println!();
    println!("Char indices.");
    text.char_indices().for_each(|(i, value)| {
        print!(" index: {} Value: {},", i, value);
    });
    println!();
    println!("Sliced byte.");
    let emoji = &text[0..4];
    println!("Emoji: {}", emoji);
    //here the char count position and char indecies position will be differ, bcs the char().count() returns the count of the total chars exist.
    //but the indecies points out the every char's byte position so the non ascii chars are using unicode value and it can have 1 to 4 bytes for one char.

    println!();
    println!("...............Part 5...............");
    println!();

    let note_box = Box::new(note); // ok i have a qustion is this note is actually getting copied or ownership will change? i thing ownership will change bcs note dosnt have copy trait.
    // but can you expline how the note value on the stack will move to heap?
    println!("Boxed note's title: {}", note_box.title);
    println!("Boxed note's pointer: {:p}", note_box);

    let note_box_2 = note_box;
    println!("Boxed note_2's title: {}", note_box_2.title);
    // println!("Moved value: {}", note_box.title); // this cause error bcs the owner ship moved to note_box_2 and note_box become shaddowed.

    //Research chellenge.
    println!();
    println!("...............Research chellenge...............");
    println!();
    let new_note = *note_box_2; // here i think the  value moved to stack? or is it just in heap? but how without a pointer it's owner can in stack but the value in heap, so i think the value moved to heap.
    println!("Whole note: {:?}", new_note);

    println!();
    println!("...............Small Excersise...............");
    println!();

    let text = String::from("Lol");
    let name_holder = NameHolder::new(&text);
    println!("Name holder before move: {:?}", name_holder);

    let text_2 = text;
    // println!("Name holder before move: {:?}", name_holder); i thought the compiler will show error in this print statement  such as the life time ends but
    // intrustingly it showed in the text_2 variable's initialization.
}
