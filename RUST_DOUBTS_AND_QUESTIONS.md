# Rust Doubts And Questions

This file keeps useful learning context between chats. It is not a strict task list.
Questions can be answered naturally during lessons, one at a time.

## Active Doubts To Revisit

### Attributes and macros
- Is `#[derive(Debug)]` like a C++ preprocessor directive?
- What is a Rust macro?
- How does the `format!` macro work?
- What is the difference between normal comments `//` and documentation comments `///`?
- Why does Rust give E0585 when a documentation comment does not document anything?

### Visibility
- Are struct fields private by default?
- When do fields or functions need the `pub` keyword?

### Strings
- How does `push_str` work?
- When should I use `String` and when should I use `&str`?
- What exactly is returned when a function returns a `String`?
- What is `&'static str`, and why do string literals have that type?
- Is a string literal / `&'static str` stored on the stack, or somewhere else?
- How is Rust's `'static` lifetime different from a Java `static` variable?
- Can a `String` be passed to a function that expects `&str` by using `&text`?
- Why can `&String` sometimes behave like `&str`?
- Why is `fn echos(text: &String) -> &str` accepted without writing lifetimes?
- Why does `echos(&"Hi".to_string())` work inside `println!`, but storing the result can fail?
- Why is `&str` often preferred over `&String` for function parameters?
- What is the lifetime of the temporary `String` in `&"Hi lol".to_string()`?
- Why does Clippy warn about `unnecessary_to_owned` when using `.to_string()` before passing to `&str`?
- What is plain `str`, and why do we usually see it as `&str`?
- Does a string slice own text, or does it only borrow text from another owner?
- In `let slice = "lol";`, who owns the `"lol"` text?
- Does `&text[0..2]` create a new separate word/string, or only borrow part of `text`?
- Why does `str::replace` return a new `String` instead of modifying the original?
- How can a `String` be mutated safely?
- What is a byte position vs a character position in a Rust string?
- Why can slicing a non-ASCII string with `0..2` panic?
- When a `String` grows beyond capacity, is the `String` value recreated, does the starting pointer stay the same, and are the UTF-8 bytes copied?
- Is the `String` value itself on the stack or heap, and can Rust manually create values on the heap?
- Why does `println!("{}", boxed_value)` print the value inside a `Box<T>` instead of a memory address, even without writing `*boxed_value`?
- What exactly is `Box<T>`: a class, struct-like thing, pointer, wrapper, or owner?
- How should Rust `Box<T>` be compared with Java `new Something()` and C++ stack/heap object creation?
- When `Box::new(note)` is used, is the `Note` copied, or does ownership move into heap storage?
- When `let new_note = *note_box_2;` is used, does the `Note` move back out of the box, and where does the value live afterward?

### Ownership and borrowing
- What exactly happens when a `String` is moved into a function parameter?
- Why can I no longer use a `String` after passing it to a function that takes ownership?
- Which types implement `Copy`, and do references implement `Copy`?
- How are Rust traits similar to and different from Java interfaces?
- If a function takes a `String`, can it return that same `String` so the caller owns it again?
- When does an immutable borrow like `borrow(&text)` end?
- What is variable shadowing with `let text = ...`, and how is it different from reassigning a mutable variable?
- Does `let mut text` automatically allow other functions to modify it, or only when I pass `&mut text`?
- After `let x = ...` shadows an older `x`, is the old value changed, dropped immediately, or just hidden?
- If the newer shadowed `x` is moved, can the older shadowed `x` be used again?
- If I take a reference to the old `x` before shadowing, can I still use that reference after a new `x` is bound?
- How does shadowing behave differently in the same scope compared with an inner block scope?
- Why does Rust warn `value assigned to a is never read` when a mutable variable is overwritten before using its first value?
- Why do I need `*value` to change data through an `&mut` reference?
- Should a function that mutates through `&mut` also return the changed value?
- Why does Rust allow only one mutable reference at a time?
- Why can I create a second `&mut` reference after the first one was used?
- Is a borrow's lifetime always the same as the `{ }` block scope?
- Why is "two mutable references" a compiler error instead of a runtime panic?
- What exactly does "non-lexical lifetime" mean?
- If an immutable reference is used after a mutable reference is created, why does that fail?
- Are Rust references basically pointers, and how are safe references different from C/C++ raw pointers?
- Why can reassigning a `String` fail while a slice borrowed from it is still used later?
- Why can't Rust strings be mutated with `text[0] = "H"` like an array?
- Why can moving `text` fail at `let text_2 = text` when a `NameHolder` still borrows from `text`, even before using `name_holder` again?
- For a struct like `Preview<'a>` or `NameHolder<'a>`, does the borrow last until the borrowed struct is dropped, or only until its last use?
- What happens if the owner of data borrowed by `Preview<'a>` or `NameHolder<'a>` goes out of scope before the borrowed struct?
- Is `impl<'a> Something<'a>` like Java method generics, and where exactly is that analogy useful vs misleading?

### Struct defaults
- Can I write a default field value inside a struct definition, such as `starting_sentence: &'static str = "Once upon a time"`?

### Constants and statics
- What is the difference between Rust `const`, Rust `static`, Java `static`, and Java `final`?
- Can one instance overwrite a `static` value, and how is that different in Rust?

### Expressions
- How does `if` work as an expression in Rust?

## Reviewed But Not Yet Fully Mastered

### Struct initialization
- Struct literals use exact field names.
- Shorthand only works when the variable name matches the field name.
- Expressions such as `100 - age` are valid as field values, for example after `field_name:`.

### Formatting output
- `{:?}` means debug formatting.
- Custom structs can use `#[derive(Debug)]` to get beginner-friendly debug printing.

### Comments and documentation
- `//` is a normal comment for notes inside code.
- `///` is a documentation comment and must document a following item such as a function, struct, enum, module, or trait.
- If the comment is only explaining an experiment inside `main`, use `//`, not `///`.
- A doc comment attaches to the next real Rust item; commented-out code does not count as an item.

### Compiler warnings and formatting
- Printing fields directly makes Rust count them as used; printing only with derived `Debug` does not.
- `cargo fmt --check` verifies standard Rust formatting without changing files.

### String vs string slice
- `String` owns its text and can come from converting a string literal with `.to_string()`.
- A string literal has type `&'static str`; it is an immutable reference to text stored for the whole program.
- Rust's `'static` lifetime is not the same idea as a Java class-level `static` field.

### Ownership and borrowing in function parameters
- A function parameter of type `String` takes ownership of the `String`.
- After ownership moves into the function, the original variable cannot be used unless the value is returned or cloned.
- A function parameter of type `&str` borrows text and does not take ownership.
- `&String` can often be coerced to `&str`, as long as the `String` is still alive.
- Shared references like `&T` are `Copy`; mutable references like `&mut T` are more restricted and should be learned later.
- An immutable borrow used in a function call normally ends after its last use, so moving the owner afterward can be valid.
- If a function owns a `String` and does not return it, the `String` is dropped at the end of that function.
- A function can return a moved `String`, giving ownership back to the caller.
- `let text = owner(text)` is shadowing: it creates a new binding named `text` from the returned value.
- `mut` allows reassignment or mutation by the owner; another function can only mutate through an explicit mutable reference such as `&mut text`.
- Shadowing creates a new binding; it does not change the old value.
- After a new binding shadows an old one in the same scope, the old one cannot be reached by that same variable name.
- If the new binding is moved, that does not make the old shadowed binding available again.
- A reference or another variable created before shadowing can still refer to the old value if the old value has not been moved and the reference remains valid.
- If shadowing happens only inside an inner block, the outer binding becomes accessible again after the block ends.
- Assignment to an existing `mut` binding changes/replaces that binding's current value; it is different from shadowing with `let`.
- If a `String` value in a mutable binding is replaced before being read, the old `String` is dropped and Rust may warn about the unused assignment.
- A warning-free example should read or print values before replacing them if the old value matters.
- Shadowing can reuse a name without `mut`; mutable assignment needs `let mut`.
- `&mut T` lets a function temporarily mutate a value owned by the caller.
- To write through a mutable reference, use dereferencing, such as `*value += 1`.
- A function that mutates through `&mut` often returns `()` unless the returned value has a separate purpose.
- Rust allows one active mutable reference to a value at a time.
- A mutable borrow can end after its last use, even before the surrounding `{ }` block ends.
- Two sequential mutable borrows can be valid if the first is no longer used when the second starts.
- Two overlapping mutable borrows are rejected at compile time.
- Non-lexical lifetimes mean the compiler can infer a borrow's lifetime from actual use, not only from the surrounding curly-brace scope.
- The reference itself is not "non-lexical"; the inferred borrow lifetime can be non-lexical.
- If a reference is used later, the borrow stays active until that later use.
- Immutable borrows and mutable borrows can appear in the same block if their active lifetimes do not overlap.
- Prefer saying "the borrow ends" instead of "the reference is destroyed" when explaining NLL.
- Helper blocks `{ ... }` can make a borrow's scope explicit and easy to reason about.
- A helper block is sometimes unnecessary because of NLL, but it is useful for beginner clarity.
- A returned reference must be tied to data that lives long enough outside the function.
- A reference returned from a function can borrow from an input reference.
- Temporaries can be borrowed for one expression, but storing a reference to a temporary for later use is not allowed.
- For read-only string input, `&str` is usually more flexible than `&String`.
- `"Hi lol"` is a string literal with `'static` lifetime, but `"Hi lol".to_string()` creates a new temporary owned `String`.
- A temporary `String` used as a function argument normally lives until the end of that statement/expression.
- A returned reference into a temporary `String` cannot be stored for later use because the temporary owner is dropped too soon.
- A `String` owns its string data; a `&str` borrows UTF-8 text from somewhere else.
- A `&str` is pointer-like, but it does not own or free the text it points to.
- Plain `str` is unsized, so it is usually used behind a reference as `&str`.
- Safe Rust references are guaranteed by the compiler to be valid for their lifetime; raw pointers are a later, unsafe topic.
- In `let slice = "lol";`, `slice` owns/copies the reference value, while the literal text itself lives in static program memory.
- `&text[0..2]` borrows part of `text`; it does not allocate or create a new owned `String`.
- Reassigning a `String` replaces/drops its old buffer, so Rust forbids doing that while a borrowed slice into the old buffer will still be used.
- `str::replace` is non-mutating and returns a new allocated `String`; the original is unchanged unless the returned value is assigned somewhere.
- Rust string ranges use byte indexes and must land on valid UTF-8 character boundaries.
- A Unicode character may take multiple bytes, so Rust does not support direct `text[0] = ...` string mutation.
- To mutate a `String`, use methods such as `push_str`, `clear`, `truncate`, `insert_str`, or `replace_range`, while respecting borrow rules and UTF-8 boundaries.
- `replace_range` mutates the original `String`; it is valid after a borrowed slice's last use, but not while that slice will still be used later.
- The latest approved exercise used ASCII text, printed a borrowed slice first, then mutated the owner with `replace_range(0..2, "Hey")`.
- `.bytes()` inspects the raw UTF-8 byte values, while `.chars()` inspects Unicode scalar values; a Rust `char` is not always the same as one human-visible written character.
- `.chars().enumerate()` gives character-count positions, not byte positions; `.char_indices()` gives valid byte positions for each Unicode scalar value.
- A tuple is an ordered group of values, such as `(index, value)`; it is not the same concept as a key-value pair, even if some tuples are used in a key-value-like way.
- In a string slice range like `0..3`, the end `3` is exclusive but still must be a valid UTF-8 boundary; `0..2` fails for a 3-byte Tamil scalar value because byte `2` is inside that value.
- A `String` owns UTF-8 bytes in a heap allocation; the `String` value tracks pointer, length, and capacity. When it grows beyond capacity, Rust may allocate a new heap buffer and free the old one, while the `String` variable remains the owner.
- During `String` reallocation, the heap buffer may move and the pointer may change. Existing bytes are copied/moved into the new buffer; the old heap buffer is freed. The local `String` variable/value is updated rather than conceptually deleted and recreated.
- A local `String` binding usually stores the `String` value on the stack, while its UTF-8 byte buffer is on the heap. The `String` value itself can live on the heap if placed inside something like `Box<String>` or `Vec<String>`.
- `Box<T>` formatting with `{}` or `{:?}` usually forwards to the inner `T` when `T` implements the matching formatting trait; use pointer formatting such as `{:p}` when intentionally inspecting an address.
- `{:p}` on `Box<T>` prints the pointer address held by the box. For `Box<String>`, this is the address of the boxed `String` value; the string's UTF-8 bytes are in a separate heap buffer owned by that `String`.
- Mutable references `&mut T` are not `Copy`; moving a mutable reference transfers the exclusive borrow. A struct containing an `&mut T` cannot safely be `Copy`, because that would create two active mutable references to the same value.
- A struct field like `name: &str` needs a named lifetime, for example `struct NameHolder<'a> { name: &'a str }`. The annotation means the holder cannot outlive the borrowed string data.
- When `&String` is used where `&str` is expected, Rust can coerce it to a borrowed view of the string's UTF-8 text; the `&str` is not borrowing the `String` value structure directly.
- `NameHolder<'a>` uses a generic lifetime parameter. It is similar in syntax to generics, but it is not a type like Java generics; it describes how long borrowed data must remain valid.
- `impl<'a> Preview<'a>` means "declare the lifetime parameter `'a` for this impl block, then use it with `Preview<'a>` and method arguments/returns." This is syntactically similar to declaring a generic parameter, but `'a` is a lifetime relationship, not a runtime object or Java type parameter.
- A borrow stored in a struct remains active at least until the struct's last use. If the struct might be used later, Rust treats the borrow as still active; after its last use, non-lexical lifetimes can allow later mutation or moves of the owner.
- If an owner would be dropped or moved while a borrowed struct can still use a reference into it, Rust rejects the code at the owner move/drop point so the borrowed struct can never dangle.
- In the `BookPreview<'a>` exercise, moving `book_title` after printing the preview is allowed because the borrow through `preview.title` has reached its last use. This is non-lexical lifetime behavior, not a "lexical lifetime" ending at the block boundary.
- It is slightly misleading to say the preview instance is dropped immediately after its last use. The safer wording is: Rust can end the active borrow after the last use of the reference, even though the variable's lexical scope may continue.

### Toolchain setup
- A previous `cargo run` or `rustc main.rs` command may have failed with a missing linker on Windows.
- Current toolchain observed on 2026-07-11: `stable-x86_64-pc-windows-msvc`.
- `link.exe` and `cl.exe` were not found on PATH, but `gcc.exe` was found at `C:\w64devkit\w64devkit\bin\gcc.exe`.
- On the later review, `cargo run` succeeded, so keep this as a watch item instead of the next lesson.

## Workflow Notes

- The learner wants a structured path toward intermediate Rust, including fundamentals revisited more deeply over time.
- Long-term Rust goal: systems-level programming such as operating systems and compilers; learner also expects to learn C and C++ for that path.
- Near-term career goal: get a software engineering job using Spring Boot and React, while keeping Rust backend/microservice practice as a useful extra tool and real-world Rust practice.
- The learner wants occasional larger recap exercises when ready, combining concepts already studied instead of only doing continuous tiny exercises.
- Use bigger recap exercises after several focused lessons, and review them in small chunks with no more than three important issues at a time.
- Bigger recap exercises no longer need to wait for weekends. They may be given whenever useful, optionally saved in a separate file, and the learner can complete them later when time allows.
- If a bigger exercise is unfinished, continue normal lessons/reviews instead of blocking progress; review the bigger exercise whenever the learner brings it back.
- Bigger recap exercises may include one small unfamiliar concept labeled as a research challenge, so the learner can search, learn, and apply it before the review.
- First bigger recap exercise has been created at `BIG_EXERCISE_01_TEXT_NOTE_RECAP.md`; it can be completed anytime and should not block normal small lessons.
- Treat code comments as possible questions, doubts, or self-answers to review.
- When the learner explains something in comments, classify it as correct, incomplete, or misleading and teach the missing part.
- At the start of each new topic, introduce what it is, why it matters, and what to focus on.
- Do not approve an exercise or move to the next exercise when the current target concept is still incomplete, unless the learner explicitly asks to move on anyway.
- The learner may clean comments and experiment code after a review is approved.
- Important questions should be saved here before cleanup so future chats can continue smoothly.
- Important learning/project files may be staged and committed to Git when useful so future chats can recover the learning history from the repository. Do not commit generated build artifacts.
- Current cleaned code baseline was approved after `cargo check`, `cargo clippy`, and `cargo run` passed.
- A later `Human` exercise transferred the `derive(Debug)` idea, but had unused-field warnings and formatting differences.
- The `Human` exercise was then cleaned so all fields were read directly and `cargo fmt --check` passed.
- The `String` vs `&'static str` exercise passed all checks after the learner corrected the comment direction.
- The ownership-vs-borrowing function exercise passed all checks; comments revealed useful questions about moves, `Copy`, `&String` to `&str`, and `'static`.
- A later ownership comment review showed good understanding of moving `String` and borrowing before moving, with more practice needed on returning ownership and borrow end points.
- The return-ownership exercise passed all checks; next topic should clarify variable shadowing versus mutability.
- A shadowing-vs-mutability answer review showed progress, with more practice needed on old bindings being hidden rather than changed.
- A follow-up shadowing question clarified that an old binding is hidden by name, not converted into the new value; references or separate saved bindings can still access old data when valid.
- A scope-and-shadowing exercise showed correct inner-block shadowing and mutable reassignment; one warning remains about overwriting a value before reading it.
- The mutable assignment vs shadowing cleanup passed all checks with no warnings.
- The first `&mut` exercise passed all checks; next lesson should focus on the rule that only one mutable reference can be active at a time.
- A mutable-reference follow-up showed that the learner is ready to learn borrow lifetimes and non-lexical lifetimes.
- A documentation-comment error E0585 appeared because `///` was used as an ordinary inline note inside `main`.
- The documentation-comment follow-up passed compile checks; `/// this is doc` currently documents `main` because the `just_mut` function below it is commented out.
- The non-lexical lifetime exercise passed all checks; next lesson should practice mixing immutable and mutable borrows.
- The immutable-then-mutable borrow exercise passed all checks; next lesson can use helper blocks to make borrow scopes explicit.
- The helper-block borrow-scope exercise passed all checks; next lesson can introduce returning references safely.
- The returning-reference exercise passed all checks; the next lesson should clarify temporaries and why returned references must not outlive their owner.
- The temporary-reference exercise passed all checks using a named `String`; the remaining concept is the lifetime of `&"Hi lol".to_string()`.
- The string mutation exercise passed all checks on 2026-07-13; next lesson should use `bytes()` and `chars()` to make byte positions vs character positions visible.

## Low Priority Conversation Questions

- Explain small Rust terms slowly as they appear instead of treating them as urgent topics.
