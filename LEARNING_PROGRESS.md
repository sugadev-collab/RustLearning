# Learning Progress

## 2026-07-25 - BookPreview Struct Lifetime Small Exercise

### Demonstrated successfully
- Created `BookPreview<'a>` with borrowed fields `title: &'a str` and `author: &'a str`.
- Implemented `impl<'a> BookPreview<'a>` and used an associated `new` function with `&'a str` parameters.
- Created owned `String` values for `book_title` and `book_author`.
- Built a preview by borrowing from both owned strings.
- Printed the preview fields directly, so there were no unread-field warnings.
- Moved `book_title` after the preview's last use and correctly connected this to Rust ending the borrow after its last actual use.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all completed successfully with no warnings.

### Still practicing
- Use the term "non-lexical lifetime" for this behavior: the borrow can end after the last use, before the end of the surrounding `{ }` scope.
- The preview value is not necessarily dropped immediately after the last print. More precisely, the borrow of `book_title` through `preview.title` is no longer active because `preview.title` is not used again.
- If `preview.title` were used after moving `book_title`, Rust would reject the move of `book_title` because the stored reference would still need to remain valid.

### Review result
Approved as clean. The code correctly demonstrates a lifetime-parameterized struct borrowing from owned `String` values and then moving an owner only after the relevant borrow's last use.

### Next concept
Borrowed structs and explicit helper scopes.

## 2026-07-25 - Struct Lifetime Annotation Review After Big Exercise

### Demonstrated successfully
- Revisited `Preview<'a>` and correctly used `impl<'a> Preview<'a>` to define methods for a lifetime-parameterized struct.
- Connected `impl<'a> Preview<'a>` with the idea of declaring a generic parameter before using it, similar in shape to Java/C++ generics syntax.
- Built a `Preview` that borrows `&note.title` and `&note.body[0..12]` from an owned `Note`.
- Used the preview before mutating `note.body`, then successfully mutated the body afterward with `replace_range`.
- Completed the big recap exercise path through owned structs, borrowed preview structs, UTF-8 inspection, valid emoji slicing, `Box<Note>`, box moves, and `Debug`.
- `cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all completed successfully.

### Still practicing
- `impl<'a> Preview<'a>` is not Java method generics exactly; `'a` is a generic lifetime parameter that describes how long borrowed references must remain valid.
- The borrow stored inside a lifetime-parameterized struct lasts until the struct's last use, or until the struct is dropped if it may still be used later.
- The owner of borrowed data must remain alive and unmoved while any stored reference to it can still be used. If the owner would go out of scope or be moved first, Rust rejects the code at compile time.
- A lifetime annotation does not make the owner live longer and does not manually end a borrow; it gives the compiler a rule to check.
- Clean warning-level issues next: unused variables, fields only read through derived `Debug`, and Clippy's `needless_return`.

### Review result
Approved for the big recap exercise as functionally correct and formatted. There are still warning/style issues, and the lifetime mental model should get one focused cleanup pass before moving on.

### Next concept
Struct lifetime annotations for borrowed fields.

## 2026-07-24 - Big Exercise 01 And Struct Lifetime Review

### Demonstrated successfully
- Completed the larger text-note recap in `src/main.rs`.
- Created an owned `Note` with `String` fields and an associated `new` function.
- Created `Preview<'a>` with borrowed `&'a str` fields and a generic lifetime parameter.
- Built a `Preview` that borrows from `note.title` and a slice of `note.body`.
- Used the preview before mutating `note.body`, then mutated the body afterward with `replace_range`.
- Correctly used `char_indices()` to choose a valid UTF-8 byte slice for the robot emoji.
- Put a `Note` into `Box<Note>`, moved the box to another binding, derived `Debug`, and printed the unboxed note.
- `cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all completed successfully.

### Still practicing
- Cleaning warning-level issues: unused imports, unused variables, and fields only read through derived `Debug`.
- `Preview<'a>` does not require `'static`; it can borrow from a normal local owner as long as the owner lives long enough.
- A lifetime annotation does not tell Rust the exact lifetime of UTF-8 bytes; it connects borrowed fields to the owner so the compiler can reject dangling references.
- Moving `note` into `Box::new(note)` transfers ownership. The value is placed in heap storage owned by the box; it is not a normal `Copy` duplication.
- Dereferencing a `Box<Note>` with `*note_box_2` moves the `Note` value out of the box when `Note` is not `Copy`.
- When a struct like `NameHolder` still contains a borrow of `text`, moving `text` into `text_2` is rejected at the move line because that would invalidate the already-existing borrow.

### Review result
Partially approved. The big exercise is functionally complete and formatted, but the code still has warnings and the lifetime explanation needs one more focused pass.

### Next concept
Struct lifetime annotations for borrowed fields.

## 2026-07-21 - Struct Lifetime Annotation Follow-Up

### Demonstrated successfully
- Added a lifetime parameter to the struct: `NameHolder<'a>`.
- Used the borrowed field type `name: &'a str`.
- Created a `NameHolder` that borrows from a local `String`.
- Correctly remembered that `&String` can coerce to `&str`.
- `cargo check`, `cargo clippy`, and `cargo run` all passed.
- Requested the first larger recap exercise, so `BIG_EXERCISE_01_TEXT_NOTE_RECAP.md` was created.

### Still practicing
- `'a` is a generic lifetime parameter, not a Java-style type generic.
- `NameHolder<'a>` means: this holder contains a reference, and the referenced data must live at least as long as the holder uses it.
- Coercion from `&String` to `&str` gives a borrowed view into the string's UTF-8 text; it does not copy out the bytes.

### Review result
Not approved yet. The code is correct, but the explanation says the meaning of `'a` is still unclear.

### Next concept
Struct lifetime annotations for borrowed fields.

## 2026-07-21 - Struct Borrowed Field First Review

### Demonstrated successfully
- Created a `NameHolder` struct with a borrowed string field idea.
- Created a `String` owner named `text`.
- Tried to store a borrowed view of `text` inside `NameHolder`.
- Hit the expected compiler error `E0106: missing lifetime specifier`, which is the main point of this lesson.

### Still practicing
- A struct field written as `&str` needs an explicit lifetime parameter because the struct may store that reference beyond the expression where it is created.
- `&text` can be coerced from `&String` to `&str`, but the `&str` points at the UTF-8 text bytes owned by the `String`, not at the `String` value structure itself.
- The lifetime annotation does not make the borrowed text live longer; it describes the rule that `NameHolder` must not outlive the text it borrows.

### Review result
Not approved yet. The compiler error is expected, but the next step is to add the lifetime parameter and correct the borrowing explanation.

### Next concept
Struct lifetime annotations for borrowed fields.

## 2026-07-21 - Box Move Ownership Review

### Demonstrated successfully
- Created a `Box<String>` and moved it from `first` into `second`.
- Printed the new owner `second` successfully.
- Left the invalid `first` usage commented out.
- Correctly explained that `Box<T>` does not implement `Copy`, so `let second = first;` transfers ownership instead of making a shallow copy.
- Asked a strong follow-up question about whether mutable references can be copied and why that would be unsafe.
- `cargo check`, `cargo clippy`, and `cargo run` all passed.

### Still practicing
- A move copies some bits at the machine level, but in Rust's ownership model the old binding becomes unusable.
- Types that own unique resources, such as `Box<T>` and `String`, are usually not `Copy`.
- Mutable references `&mut T` are exclusive borrows and are not `Copy`.

### Review result
Approved for this exercise. The code correctly shows ownership transfer from one `Box<T>` binding to another.

### Next concept
Structs that hold references and why lifetimes become visible there.

## 2026-07-21 - Box Pointer Formatting Review

### Demonstrated successfully
- Correctly kept `Box<i8>` and `Box<String>` examples compiling and running.
- Correctly commented that the local `Box<T>` bindings are stack-local owner values while the boxed values are heap allocated.
- Used pointer formatting with `{:p}` on `Box<T>` to inspect the pointer address held by the box.
- Observed that normal display formatting `{}` prints the inner value, while pointer formatting `{:p}` prints an address.
- `cargo check`, `cargo clippy`, and `cargo run` all passed.

### Still practicing
- For `Box<String>`, there are two heap allocations to distinguish: the boxed `String` value, and the separate UTF-8 byte buffer owned by that `String`.
- `{:p}` on `Box<String>` prints the address of the boxed `String` value, not the address of the string's UTF-8 byte buffer.
- Use precise wording such as "String value" instead of "String object" when describing Rust, because Rust does not use Java-style objects in the same way.

### Review result
Approved for this exercise. The code shows boxed values, normal formatting, and pointer formatting correctly.

### Next concept
Moving `Box<T>` and ownership transfer.

## 2026-07-20 - Box Ownership Question Review

### Demonstrated successfully
- Created and printed `Box<i8>` and `Box<String>` values.
- Correctly stated that `age_box` and `name` bindings are local variables and that boxed data is heap allocated.
- Asked useful comparison questions between Rust `Box<T>`, Java object references, and C++ stack/heap object creation.
- Kept the code compiling cleanly; `cargo check`, `cargo clippy`, and `cargo run` all passed.

### Still practicing
- `Box<T>` is an owning smart pointer type, not a Java-style class object reference.
- In Rust, the variable's type is `Box<i8>` because the variable owns the box; `&Box<i8>` would mean borrowing the box.
- Rust placement is determined by the owning type and context: local values are usually on the stack, `Box::new(value)` puts `value` on the heap, and values inside `Vec` live in the vector's heap buffer.
- For `Box<String>`, the local `Box` owner is on the stack, the `String` value is heap allocated by the box, and the `String` also owns a separate heap buffer for its UTF-8 bytes.

### Review result
Not approved yet. The code works, but the comments/questions show `Box<T>` ownership and stack/heap placement still need one more focused pass.

### Next concept
Continue `Box<T>` as an owning smart pointer.

## 2026-07-19 - Box First Exercise Review

### Demonstrated successfully
- Created a `Box<i8>` with `Box::new(23_i8)`.
- Created a `Box<String>` with `Box::new(String::from("Rust"))`.
- Printed boxed values successfully.
- Noticed an important behavior: printing a `Box<T>` with `{}` can show the inner value rather than a memory address.
- Kept the code compiling cleanly; `cargo check`, `cargo clippy`, and `cargo run` all passed.

### Still practicing
- `Box<T>` is the owner value; a local `Box<T>` binding is usually on the stack and points to the boxed `T` on the heap.
- For `Box<i8>`, the `Box` owner is on the stack and the `i8` value is on the heap.
- For `Box<String>`, the `Box` owner is on the stack, the `String` value is on the heap, and the `String`'s UTF-8 byte buffer is also heap allocated.
- `println!("{}", boxed_value)` works because formatting for `Box<T>` forwards to the inner `T`; this is not because the inner value is `Copy`.
- Memory addresses are printed with pointer formatting such as `{:p}`, not normal display formatting `{}`.

### Review result
Not approved yet. The code works, but the comments need correction around stack/heap placement and why `println!` displays the boxed value.

### Next concept
Continue `Box<T>` ownership and dereferencing.

## 2026-07-17 - String Capacity Review

### Demonstrated successfully
- Created a mutable `String`.
- Printed `.len()` and `.capacity()` before mutation.
- Used `push_str` to append more text.
- Printed `.len()` and `.capacity()` again after mutation.
- Observed through `cargo run` that both length and capacity changed from `3` to `12` in this specific example.
- Kept the code compiling cleanly; `cargo check`, `cargo clippy`, and `cargo run` all passed.
- Improved the explanation that a `String` value stores pointer, length, and capacity while the UTF-8 bytes live in a heap buffer.
- Corrected the reallocation model: when capacity is exceeded, the byte buffer may be copied to a new place and the `String` value's pointer, length, and capacity are updated.

### Still practicing
- Describing `String` as storing UTF-8 bytes, not a char array.
- Remembering that a local `String` value is usually on the stack while its byte buffer is on the heap.
- Capacity may be equal to length in one example, but it is allowed to be larger than length.
- If reallocation happens, the starting pointer may change; it is not guaranteed to stay the same.
- Java `ArrayList` is a useful analogy: it may allocate a bigger backing array and copy references/elements, while the `ArrayList` object remains the object the variable refers to.
- A local `String` value is usually stored on the stack, but a `String` value can also live inside heap-allocated owners such as `Box<String>` or `Vec<String>`.
- Manual heap allocation in safe Rust is usually expressed through owner types such as `Box`, `Vec`, and `String`, rather than direct `new`/`delete` style memory management.

### Review result
Approved for this exercise. The code and explanation correctly show length, capacity, heap buffer growth, and pointer/length/capacity updates.

### Next concept
Heap allocation with `Box<T>`.

## 2026-07-16 - String Length Count Review

### Demonstrated successfully
- Used `.len()` on a `String` and printed the result.
- Used `.chars().count()` on the same `String` and printed the result.
- Correctly suspected that `.len()` returns byte length.
- Observed through `cargo run` that the Tamil text has byte length `35` and Rust `char` count `13`.
- Correctly explained that the current string has 11 three-byte Tamil Rust `char` values plus 2 one-byte ASCII values.
- Kept the code compiling cleanly; `cargo check`, `cargo clippy`, and `cargo run` all passed.

### Still practicing
- Saying `.chars().count()` counts Rust `char` values, meaning Unicode scalar values, not necessarily human-visible written characters.
- Using precise wording: UTF-8 is the encoding, while Rust `char` means a Unicode scalar value.
- Keeping comments concise enough that the main idea is easy to scan.

### Review result
Approved for this exercise. The code and explanation correctly show byte length versus Rust `char` count.

### Next concept
String capacity with `.capacity()` and why `String` may reserve more bytes than it currently uses.

## 2026-07-15 - Tuple Destructuring Review

### Demonstrated successfully
- Used tuple destructuring in a closure parameter with `|(i, ch)|`.
- Used tuple destructuring in another closure with `|(i, b)|`.
- Used tuple destructuring in a `for` loop with `for (i, ch) in text_tam.char_indices()`.
- Avoided tuple field access like `.0` and `.1`, making the iterator output easier to read.
- Kept the safe string slice `&text_tam[0..3]` from the previous exercise.
- Kept the code clean; `cargo check`, `cargo clippy`, and `cargo run` all passed.

### Still practicing
- Choosing descriptive variable names such as `char_index`, `byte_index`, `byte`, and `ch` when comparing similar iterator outputs.
- Remembering that destructuring does not create a key-value pair; it unpacks an ordered tuple into named variables.
- Output formatting can be cleaned up later so each result appears on its own line.

### Review result
Approved for this exercise. Tuple destructuring was used correctly in both closures and `for` loops.

### Next concept
String length with `.len()` versus character count with `.chars().count()`.

## 2026-07-15 - Char Indices Slice Review

### Demonstrated successfully
- Used `.chars().enumerate()` to show character-count positions.
- Used `.bytes().enumerate()` to show byte positions and raw UTF-8 byte values.
- Used `.char_indices()` to show valid byte-start positions for each Rust `char`.
- Created a safe string slice with `&text_tam[0..3]`, using byte boundaries found from the Unicode inspection.
- Correctly observed that `&text_tam[0..2]` panics because byte position `2` is inside the first Tamil scalar value.
- Kept the code compiling and running cleanly; `cargo check`, `cargo clippy`, and `cargo run` all passed.

### Still practicing
- Rust ranges like `0..3` include byte positions `0`, `1`, and `2`, but stop before byte position `3`.
- The end of a string slice range must also be a valid UTF-8 boundary, even though it is not included in the slice.
- Tuple destructuring can make iterator code easier to read than using `.0` and `.1`.

### Review result
Approved for this exercise. The code correctly connects `.char_indices()` output to a valid string slice.

### Next concept
Tuple destructuring in `for` loops and closures.

## 2026-07-15 - Char Indices Progress Review

### Demonstrated successfully
- Used `.chars().enumerate()` to print character-count positions with `char` values.
- Used `.bytes().enumerate()` to print byte positions with raw UTF-8 byte values.
- Used `.char_indices()` to print valid byte-start positions with `char` values.
- Observed the important difference: Tamil scalar values start at byte positions such as `0`, `3`, `6`, while the space and ASCII `!` occupy one byte.
- Kept the code compiling cleanly; `cargo check`, `cargo clippy`, and `cargo run` all passed.

### Still practicing
- Connecting `.char_indices()` output to actual safe string slicing.
- Remembering that `.chars().enumerate()` positions are not valid string slice indexes unless the string is ASCII-only.
- Using tuple destructuring such as `|(index, value)|` or `for (index, value) in ...` instead of accessing tuple fields with `.0` and `.1`.

### Review result
Not approved yet. The iterator inspection part is correct, but the exercise still needs one safe slice using byte positions from `.char_indices()`.

### Next concept
Continue `.char_indices()` for valid string slice boundaries.

## 2026-07-14 - Bytes And Chars Loop Review

### Demonstrated successfully
- Replaced debug-printing of `.chars()` and `.bytes()` with iterator consumption using `.for_each(...)`.
- Printed each Unicode scalar value from Tamil text with `.chars()`.
- Printed each raw UTF-8 byte value from the same text with `.bytes()`.
- Observed that the Tamil text produces fewer `char` items than bytes.
- Kept the code compiling cleanly; `cargo check`, `cargo clippy`, and `cargo run` all passed.

### Still practicing
- Using `.enumerate()` to print positions, not only values.
- Understanding that the indexes from `.chars().enumerate()` are character-count positions, not byte positions.
- Understanding that Rust string slicing uses byte positions, so `.bytes().enumerate()` and `.char_indices()` are especially important.
- Choosing clear output formatting so byte/character comparisons are easy to read.

### Review result
Reviewed without editing `src/main.rs`. The code is correct for printing values, but the exercise should be extended to show positions.

### Next concept
`.char_indices()` for valid string slice boundaries.

## 2026-07-14 - Bytes And Chars First Review

### Demonstrated successfully
- Used `.chars()` and `.bytes()` on an ASCII `String`.
- Used `.chars()` and `.bytes()` on Tamil text to compare Unicode text with raw UTF-8 bytes.
- Correctly noticed that standard ASCII characters take one byte each in UTF-8.
- Kept the code compiling cleanly; `cargo check` and `cargo clippy` both passed.

### Still practicing
- `.chars()` and `.bytes()` return iterators; debug-printing the iterator is useful for inspection, but normal code often loops over it or collects it.
- Tamil letters/marks are not "3 chars"; many Tamil Unicode scalar values are encoded as 3 bytes in UTF-8.
- `char` in Rust means a Unicode scalar value, not necessarily one complete human-visible character.
- Output labels should match the value being printed, especially when comparing bytes and chars.

### Review result
Reviewed without editing `src/main.rs`. `cargo check`, `cargo clippy`, and `cargo run` all passed.

### Next concept
Using `.enumerate()` with `.chars()` and `.bytes()` to print positions.

## 2026-07-11

### Demonstrated successfully
- Created a minimal Rust binary project using Rust 2024 edition.
- Kept `main` simple and compilable.

### Still practicing
- Variables and `let` bindings.
- Mutability with `mut`.
- Writing and calling beginner-friendly functions.
- Using basic data types such as integers, floating-point numbers, booleans, and strings.

### Next concept
Variables, mutability, functions, and basic data types.

## 2026-07-11 - Struct Exercise Review

### Demonstrated successfully
- Attempted to define a custom `Person` struct.
- Attempted to write an associated constructor function with `impl Person`.
- Used owned `String` values and tried to build a string step by step with `push_str`.

### Still practicing
- Struct literal syntax, especially matching field names correctly.
- The difference between field shorthand and explicit `field: value` syntax.
- Printing custom structs with the `Debug` trait and `{:?}`.
- Keeping numeric types consistent, such as `i32` compared with `f32`.
- Windows Rust linker/toolchain setup.

### Next concept
Struct initialization and field syntax.

## 2026-07-11 - Second Struct Review

### Demonstrated successfully
- Fixed the previous struct literal errors in `Person::new`.
- Used explicit field initialization correctly in `Book::new`, including `pages: total_page`.
- Used `#[derive(Debug)]` to make a custom struct printable with debug formatting.
- Used `format!` to build a `String` from values.
- Used an `if` expression inside `println!`.

### Still practicing
- Attributes such as `#[derive(Debug)]`.
- Macros such as `println!` and `format!`.
- Struct field visibility and the `pub` keyword.
- String slice lifetimes such as `&'static str`.

### Review result
Approved for the current chapter. `cargo check`, `cargo clippy`, and `cargo run` all passed.

### Next concept
Rust attributes and macros.

## 2026-07-11 - Human Warning Cleanup Review

### Demonstrated successfully
- Printed every meaningful `Human` field directly with `{}`.
- Removed the previous unread-field compiler warning.
- Kept `#[derive(Debug)]` and used `{:?}` as extra debug output.
- Formatted the code so `cargo fmt --check` passes.

### Still practicing
- Remembering that normal output and debug output serve different purposes.
- Understanding macros such as `println!` beyond basic use.
- Choosing between `String`, `&str`, and `&'static str` for struct fields.

### Review result
Approved as clean. `cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all passed.

### Next concept
String vs string slice in struct fields.

## 2026-07-12 - String And Static Comment Review

### Demonstrated successfully
- Used one `String` field and one `&'static str` field in a struct.
- Correctly created an owned `String` with `.to_string()`.
- Corrected the string-slice comment toward immutable reference plus static memory.
- Kept the code formatted; `cargo fmt --check` passed.

### Still practicing
- Distinguishing Rust's `'static` lifetime from Rust `static` items.
- Distinguishing Rust `const`/`static` from Java `static`/`final`.
- Explaining memory locations carefully without mixing stack, heap, and static storage.

### Review result
Approved for this exercise. `cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all passed.

### Next concept
String ownership and borrowing in function parameters.

## 2026-07-12 - Ownership And Borrowing Function Review

### Demonstrated successfully
- Wrote one function that takes ownership with a `String` parameter.
- Wrote one function that borrows text with an `&str` parameter.
- Correctly noticed that using `text` after `owner(text)` is invalid because ownership moved.
- Correctly reasoned that borrowing before moving can work because the owner is still alive at that time.
- Kept the code clean; `cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all passed.

### Still practicing
- Understanding exactly what a move does.
- Understanding `Copy`, especially that shared references like `&T` are `Copy`.
- Understanding automatic coercion from `&String` to `&str`.
- Understanding that `'static` lifetime does not mean only Rust `static` variables can be borrowed.

### Review result
Approved as clean code. Conceptually, ownership and borrowing should be practiced more before moving to deeper lifetimes.

### Next concept
Ownership moves vs borrowing.

## 2026-07-12 - Ownership Comment Review

### Demonstrated successfully
- Called `borrow(&text)` before `owner(text)` and correctly recognized that borrowing first can work.
- Correctly explained that `owner(text)` moves ownership because `String` is not `Copy`.
- Correctly reasoned that a moved `String` cannot be used afterward unless ownership is somehow returned.

### Still practicing
- Describing borrows precisely: a borrow does not take and give back ownership; it temporarily accesses a value.
- Returning ownership from a function using a return type such as `String`.
- Understanding when borrows end.
- Keeping formatting clean with `cargo fmt`.

### Review result
`cargo check`, `cargo clippy`, and `cargo run` passed. `cargo fmt --check` only reported extra blank lines.

### Next concept
Returning ownership from functions.

## 2026-07-12 - Returning Ownership Review

### Demonstrated successfully
- Changed `owner` so it takes a `String` and returns a `String`.
- Called `borrow(&text)` before moving ownership.
- Used `let text = owner(text)` to regain ownership under a new binding with the same name.
- Printed the returned `String` successfully after ownership came back.
- Kept the code clean; `cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all passed.

### Still practicing
- Variable shadowing with `let`.
- Difference between shadowing and mutable reassignment.
- Difference between a mutable binding and passing `&mut` to another function.

### Review result
Approved as clean. The main concept is correct; the comment question should become the next lesson.

### Next concept
Variable shadowing vs mutability.

## 2026-07-12 - Shadowing And Mutability Answer Review

### Demonstrated successfully
- Used shadowing to bind `x` first as an `f32` and then as a `String`.
- Recognized that the shadowed bindings are immutable.
- Used `let mut y` to represent a value that could be reassigned or mutably borrowed.

### Still practicing
- Shadowing does not change the old value; it creates a new binding with the same name.
- The old shadowed binding is hidden by the name `x` in the same scope.
- Moving the new `x` into `z` does not make the old `x` accessible again.
- `mut` should be used only when the variable is actually reassigned or mutably borrowed.

### Review result
`cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` passed, but the compiler reported warnings for unused variables and unnecessary `mut`.

### Next concept
Using shadowing and mutability without warnings.

## 2026-07-12 - Shadowing Follow-Up

### Demonstrated successfully
- Asked whether an old shadowed value can still be used after a new binding with the same name.
- Connected shadowing with ownership, references, and scope.

### Still practicing
- A binding is the relationship between a name and a value.
- Shadowing in the same scope hides the old binding by name.
- A reference or another variable made before shadowing can still access the old value if it remains valid.
- Inner block shadowing temporarily hides the outer binding, then the outer binding becomes visible again after the block.

### Review result
Conceptual lesson, no code changes requested.

### Next concept
Scope and shadowing.

## 2026-07-12 - Scope And Shadowing Exercise Review

### Demonstrated successfully
- Printed the first same-scope `x` before shadowing it with a new `x`.
- Used a mutable `y`, changed its value, and printed the changed value.
- Used an inner block to shadow `z`, then showed that the outer `z` is visible again after the block.
- Tested assignment to an outer mutable variable from inside an inner block.

### Still practicing
- Difference between shadowing with `let` and assignment to an existing mutable binding.
- Understanding compiler warning `value assigned to a is never read`.
- Reading a value before overwriting it when the first value matters.

### Review result
`cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` passed. There is one compiler warning because `a` is initialized as `"lmao"` and overwritten before that first value is read.

### Next concept
Mutable assignment vs shadowing.

## 2026-07-12 - Mutable Assignment And Shadowing Cleanup

### Demonstrated successfully
- Used shadowing with `let name = ...` to create a new binding with the same name.
- Used mutable assignment with `let mut age = ...` followed by `age = ...`.
- Printed before and after values so the code has no unused-value warnings.
- Kept the code clean; `cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all passed.

### Still practicing
- Explaining shadowing as a new binding rather than a changed value.
- Understanding when mutation requires only `mut` and when another function needs `&mut`.

### Review result
Approved as clean.

### Next concept
Mutable references with `&mut`.

## 2026-07-12 - Mutable Reference Exercise Review

### Demonstrated successfully
- Wrote a function that accepts a mutable reference with `&mut f32`.
- Used dereferencing with `*initial_income` to change the caller's original value.
- Passed a mutable reference from `main` using `&mut income`.
- Printed the value before and after mutation.
- Kept the code clean; `cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all passed.

### Still practicing
- Deciding whether a function should mutate through `&mut`, return a new value, or both.
- Understanding why dereferencing is needed to modify through a reference.
- Learning the borrowing rules around `&mut`, especially one mutable borrow at a time.

### Review result
Approved as clean.

### Next concept
One mutable reference at a time.

## 2026-07-12 - Mutable Reference Lifetime Review

### Demonstrated successfully
- Created a mutable reference to `income` and used it to mutate the original value.
- Created a second mutable reference after the first one's last use.
- Observed that sequential mutable borrows can be valid when they do not overlap.
- Kept the code clean; `cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all passed.

### Still practicing
- A mutable borrow is about one active mutable reference at a time, not simply one per curly-brace scope.
- Rust can end a borrow after its last use, even before the surrounding block ends.
- Creating two mutable references before using the first can make the borrows overlap and cause a compiler error.
- Compiler borrow errors are compile-time errors, not runtime panics.

### Review result
Approved as clean. The main confusion is borrow lifetime versus lexical scope.

### Next concept
Borrow lifetime and non-lexical lifetimes.

## 2026-07-12 - Non-Lexical Lifetime Exercise Review

### Demonstrated successfully
- Used sequential mutable references to mutate `income` without overlapping borrows.
- Used an immutable borrow after mutable borrows had ended.
- Created a new mutable borrow after the immutable borrow's last use.
- Correctly recognized that borrow lifetime can end at the last actual use, not only at the end of the curly-brace scope.
- Kept the code clean; `cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all passed.

### Still practicing
- Saying "the borrow has a non-lexical lifetime" rather than "the reference is non-lexical."
- Remembering that if a reference is used later, its borrow must remain valid until that later use.
- Understanding that NLL is inferred by the compiler from how references are used.

### Review result
Approved as clean. The current mental model is close and should be refined with overlapping immutable/mutable examples.

### Next concept
Mixing immutable and mutable borrows.

## 2026-07-12 - Immutable Then Mutable Borrow Review

### Demonstrated successfully
- Created an immutable borrow of `income` and used it before taking a mutable borrow.
- Mutated `income` after the immutable borrow's last use.
- Correctly commented out the invalid case where the immutable borrow would be used after mutation starts.
- Kept the code clean; `cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all passed.

### Still practicing
- Saying the borrow ends rather than saying the reference is destroyed.
- Explaining that the compiler tracks when each reference is last used.
- Understanding that immutable and mutable borrows may both appear in one scope if their active lifetimes do not overlap.

### Review result
Approved as clean.

### Next concept
Borrow scopes with helper blocks.

## 2026-07-12 - Helper Block Borrow Scope Review

### Demonstrated successfully
- Used a helper block `{ ... }` to make an immutable borrow's scope explicit.
- Printed through the immutable borrow before leaving the helper block.
- Mutated the value after the helper block ended.
- Kept the code clean; `cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all passed.

### Still practicing
- Understanding when helper blocks are useful even though non-lexical lifetimes can often infer shorter borrows.
- Keeping the difference clear between lexical block scope and inferred borrow lifetime.
- Preparing to understand why returned references need lifetime reasoning.

### Review result
Approved as clean.

### Next concept
Returning references safely.

## 2026-07-12 - Returning Reference Review

### Demonstrated successfully
- Wrote a function that takes `&String` and returns `&str` borrowed from the input.
- Relied on Rust's coercion from `&String` to `&str` in the return expression.
- Called the function and printed the returned borrowed string slice.
- Kept the code clean; `cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all passed.

### Still practicing
- Understanding that a returned reference must point to data owned somewhere else that stays alive long enough.
- Avoiding temporary owned values when learning returned references, unless the reference is used immediately.
- Prefer accepting `&str` over `&String` for read-only string input in many beginner APIs.
- Understanding why storing a reference returned from a temporary `String` would fail.

### Review result
Approved as clean. The code is accepted because the temporary `String` is used only for the duration of the `println!` statement.

### Next concept
Avoiding references to temporaries.

## 2026-07-12 - Temporary Reference Review

### Demonstrated successfully
- Created a named `String` owner before borrowing from it.
- Stored the returned `&str` in a variable while the owner `String` was still alive.
- Correctly identified that borrowing from `&"Hi lol".to_string()` and storing the result is different from using it immediately.
- Kept the code clean; `cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all passed.

### Still practicing
- The lifetime of temporary owned values such as `"Hi lol".to_string()`.
- Difference between a string literal's `'static` lifetime and a temporary `String` created from it.
- Why a temporary can be borrowed for one expression but not stored for later use through a returned reference.

### Review result
Approved as clean.

### Next concept
Temporary values and returned references.

## 2026-07-12 - Temporary Reference Follow-Up

### Demonstrated successfully
- Correctly used a named `String` owner before storing a returned `&str`.
- Correctly explained that `echos(&text)` is valid because `text` owns the data and stays alive while `echo` is used.
- Correctly recognized that `echos(&"Hi lol".to_string())` creates a temporary owned `String`.
- Correctly identified that storing a reference into that temporary is invalid because the temporary is dropped before the later `println!`.

### Still practicing
- Saying precisely that Rust prevents the dangling reference at compile time, rather than allowing erased-memory access.
- Formatting comments cleanly so `cargo fmt --check` passes.
- Building occasional larger exercises that combine earlier concepts, once the needed concepts have been practiced.

### Review result
`cargo check` and `cargo clippy` passed. `cargo fmt --check` only reported trailing spaces in commented-out lines.

### Next concept
Function parameter types: `&String` vs `&str`.

### Bigger exercise rhythm
- After several small focused lessons, use one larger recap exercise that combines earlier concepts.
- Bigger exercises should appear when the learner has enough pieces to practice integration, not after every topic.
- Bigger exercises should still be reviewed in small chunks, with no more than three important issues at a time.
- Bigger recap exercises may include one small unfamiliar concept labeled as a research challenge, so the learner can search, learn, and apply it in the project.

## 2026-07-12 - `&str` Parameter Review

### Demonstrated successfully
- Changed the function parameter from `&String` to `&str`.
- Called the function with a named `String` by passing `&text`, relying on deref coercion to `&str`.
- Called the function directly with a string literal.
- Called the function with a temporary `String` inside the same `println!` statement and understood why the temporary lasts long enough there.
- Added another string-literal example and recognized that string literals have a long-lived `'static` lifetime.

### Still practicing
- Explaining that string literals do not have a `String` owner; their text is stored in the program's static memory.
- Saying that `println!` is a macro, and the temporary lives until the end of the statement, not because `println!` is a normal function.
- Recognizing Clippy's `unnecessary_to_owned` warning: converting a literal to `String` is unnecessary when a function accepts `&str`.

### Review result
`cargo check`, `cargo run`, and `cargo fmt --check` passed. `cargo clippy` passed but reported one style/improvement warning for the intentional lesson example: unnecessary use of `.to_string()`.

### Next concept
String slices from owned `String` values.

## 2026-07-12 - String Slice Range Review

### Demonstrated successfully
- Created an owned `String`.
- Created two `&str` slices from the owned `String` using byte ranges.
- Printed the owner and both borrowed slices successfully.
- Used ASCII text, which makes the beginner range examples valid and predictable.

### Still practicing
- A `&str` slice does not own the string data; it borrows part of existing UTF-8 text.
- Plain `str` is an unsized string-slice type, so beginners usually use it through `&str`.
- A Rust reference is pointer-like internally, but safe references are not the same as C/C++ raw pointers.
- A `String` owns its heap allocation; a `&str` only points at borrowed text plus a length.

### Review result
`cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all passed.

### Next concept
String slices and UTF-8 boundaries.

## 2026-07-12 - String Slice Ownership Follow-Up

### Demonstrated successfully
- Recognized that a reference cannot be the owner of the data it points to.
- Asked the right question about `let slice = "lol";`: the variable owns the reference value, not the literal text.
- Tested what happens when a `String` has an active borrowed slice.
- Correctly suspected that reassigning the `String` while the slice is still used should be rejected.

### Still practicing
- `&text[0..2]` creates a borrowed view into the same string data; it does not create a new separate `String`.
- Reassigning a `String` replaces/drops the old owned buffer, so any slice still pointing into the old buffer would become invalid.
- Rust rejects that invalid situation at compile time if the slice is used later.
- `let slice = "lol"` stores a `&'static str`; the literal text lives in static program memory.

### Review result
`cargo check`, `cargo clippy`, and `cargo run` passed with one warning: `mut` is unnecessary while the reassignment is commented out. `cargo fmt --check` reported formatting differences.

### Next concept
Borrowed slices and owner mutation.

## 2026-07-12 - String Mutation And UTF-8 Questions

### Demonstrated successfully
- Correctly reasoned that reassigning a `String` changes which buffer the `String` owns.
- Correctly connected that reassignment with the old buffer being dropped when no owner remains.
- Asked why `replace` exists if it returns a new `String` instead of mutating the original.
- Tested a non-ASCII string slice and encountered the important UTF-8 boundary rule.

### Still practicing
- `str::replace` is non-mutating; it returns a new allocated `String`.
- Mutating a `String` can be done with methods such as `push_str`, `clear`, `truncate`, and `replace_range`, but only when no conflicting borrow is active.
- Rust string indexes are byte indexes, not character indexes.
- String slicing must use valid UTF-8 character boundaries.
- Rust does not support assigning to `text[0]` because a character may take more than one byte.

### Review result
`cargo check` and `cargo clippy` passed with warnings for unnecessary `mut` and unused return value from `replace`. `cargo fmt --check` reported formatting differences. `cargo run` panicked because byte range `0..2` ends inside the first Tamil character.

### Next concept
String mutation methods and UTF-8 boundaries.

## 2026-07-13 - `replace_range` And Borrow End Review

### Demonstrated successfully
- Created a `String` owner and borrowed a `&str` slice from it.
- Printed the borrowed slice before mutating the owner.
- Used `replace_range` to mutate the original `String` after the slice's last use.
- Kept the example ASCII-only, so the byte range `0..2` was valid.
- Correctly commented that using the slice after mutation would keep the immutable borrow active and make the mutation invalid.

### Still practicing
- Keeping commented-out examples aligned with current variable names.
- Understanding byte indexes more deeply, especially with non-ASCII text.
- Choosing between non-mutating methods like `replace` and mutating methods like `replace_range`.

### Review result
Approved as clean. `cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all passed.

### Next concept
Inspecting strings with `bytes()` and `chars()`.

## 2026-07-12 - Documentation Comment Error Review

### Demonstrated successfully
- Wrote comments explaining overlapping and sequential mutable borrows.
- Correctly identified that overlapping `&mut` borrows should be invalid.
- Noticed a documentation-comment compiler error and asked about it.

### Still practicing
- Difference between normal comments `//` and documentation comments `///`.
- Avoiding unused imports while experimenting.
- Remembering that code inside comments is invisible to the compiler, so variables used only in commented code do not count.

### Review result
`cargo check`, `cargo clippy`, `cargo run`, and `cargo fmt --check` all failed because of E0585: a documentation comment did not document an item. The compiler also warned about an unused `Mutex` import and unnecessary `mut` on `income`.

### Next concept
Normal comments vs documentation comments.

## 2026-07-12 - Documentation Comment Follow-Up

### Demonstrated successfully
- Fixed the E0585 documentation-comment error by placing `///` before a real item.
- Used normal comments `//` for learning notes inside `main`.
- Kept the borrow-overlap explanation in comments.

### Still practicing
- A doc comment documents the next real item, not commented-out code.
- If a function is commented out, Rust does not see it, so a preceding `///` may attach to a later item such as `main`.
- Formatting comments cleanly with `cargo fmt`.

### Review result
`cargo check`, `cargo clippy`, and `cargo run` passed. `cargo fmt --check` failed only because of comment indentation/trailing whitespace.

### Next concept
Borrow lifetime and non-lexical lifetimes.

## 2026-07-12 - Learning Path Note

### Goal
- Work steadily toward intermediate Rust, not only isolated beginner exercises.

### Tutoring workflow
- The learner may write questions or self-answers inside Rust code comments.
- Future reviews should check those comments for correctness and teach missing depth.
- Each new topic should begin with a short teacher-style introduction: what it is, why it matters, and what to focus on.

### Current observation
- The current `String` vs `&'static str` exercise compiles, passes Clippy, and runs.
- `cargo fmt --check` reports formatting differences.
- The comment about `String` owning text is on the right track.
- The comment about `&'static str` being a reference to text on the stack is misleading for string literals; string literals live in the program's static memory.

### Next concept
String vs string slice in struct fields.

## 2026-07-11 - Human Debug Exercise Review

### Demonstrated successfully
- Reused the `#[derive(Debug)]` idea with a different struct instead of copying the previous `Story` example.
- Created a `Human::new` associated function that accepts parameters and returns `Self`.
- Used field shorthand correctly inside the struct literal.
- Printed one field with `{}` and the whole struct with `{:?}`.

### Still practicing
- Understanding that derived `Debug` output does not count as directly reading fields for Rust's dead-code warning.
- Using all meaningful struct fields directly in program output or behavior.
- Formatting Rust code with `cargo fmt`.
- Understanding `&'static str`; keep this for later, because lifetimes are a bigger topic.

### Review result
The program compiles and runs, and `cargo clippy` passes. It is not fully clean yet because there is a compiler warning for unread fields, and `cargo fmt --check` reports formatting differences.

### Next concept
Compiler warnings and using struct fields directly.

## 2026-07-11 - Cleaned Story Review

### Demonstrated successfully
- Cleaned the previous exercise code after approval while keeping the project compilable.
- Created a small `Story` struct with a string-slice field.
- Used an associated `new` function to construct a struct value.
- Used `#[derive(Debug)]` and printed the struct with `{:?}`.

### Still practicing
- What `#[derive(Debug)]` asks the compiler to generate.
- How macros such as `println!` differ from normal functions.
- Why string literals can have the type `&'static str`.
- Where default values belong in Rust struct design.

### Review result
Approved as a clean baseline. `cargo check`, `cargo clippy`, and `cargo run` all passed.

### Next concept
Rust attributes and macros.
