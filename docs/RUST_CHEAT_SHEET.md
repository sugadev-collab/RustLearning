# Rust Learning Cheat Sheet

This is a quick-memory sheet for Rust features and standard-library APIs already encountered in the exercises. It should be updated as new lessons are approved.

> Rust language features are syntax built into Rust, such as `match`, borrowing, and the `?` operator. Types and functions such as `Vec`, `Option`, and `std::fs::read_to_string` come from Rust's standard library.

## How To Read Rust Names And Symbols

| Syntax | Plain-English meaning |
|---|---|
| `T` | A placeholder for some value type. For example, `Vec<T>` means a vector containing values of type `T`. |
| `E` | A placeholder for an error type. In `Result<T, E>`, `T` is the success type and `E` is the failure type. |
| `&T` | A shared reference: temporarily borrow a `T` without owning or changing it. |
| `&mut T` | A mutable reference: temporarily borrow a `T` and allow it to be changed. |
| `&[T]` | A borrowed slice: a view over zero or more neighboring `T` values. |
| `::` | Access something belonging to a module or type, such as `std::fs` or `String::new()`. |
| `.` | Call a method on a value or access one of its fields, such as `text.len()`. |
| `->` | Introduces a function's return type. |
| `<...>` | Supplies type information to a generic type, such as `Vec<String>`. |
| `!` after a name | Usually indicates a macro, such as `println!`, `vec!`, or `format!`. |
| `?` | Extract the success/present value, or return early on failure/absence. |
| `_` | Infer or ignore something, depending on where it appears. |

Examples:

- `Vec<String>` means “an owned growable collection of owned strings.”
- `Option<&str>` means “there may be a borrowed string slice.”
- `Result<Vec<String>, std::io::Error>` means “either a vector of strings was produced, or an input/output error occurred.”

## Cargo Commands

```powershell
cargo check                       # Compile-check without running
cargo fmt --check                 # Check formatting
cargo clippy -- -D warnings       # Treat every Clippy warning as an error
cargo test                        # Run the project's tests
cargo run --quiet                 # Build and run with less Cargo output
cargo run -- data/quests_copy.txt # Pass a file path as argument 1
```

The `--` separates Cargo's options from arguments passed to your Rust program.

## Common Types

| Type | Meaning | Example |
|---|---|---|
| `u32` | Non-negative 32-bit integer | `let xp: u32 = 100;` |
| `bool` | `true` or `false` | `let completed = true;` |
| `String` | Owned, growable UTF-8 text | `String::from("Quest")` |
| `&str` | Borrowed view of UTF-8 text | `let name: &str = "Quest";` |
| `Vec<T>` | Owned, growable collection | `Vec<String>` |
| `Option<T>` | A value may exist | `Some(value)` or `None` |
| `Result<T, E>` | An operation may succeed or fail | `Ok(value)` or `Err(error)` |

“Owned” means the current variable is responsible for the value. “Borrowed” means code temporarily accesses a value owned somewhere else.

## `String` And `&str`

```rust
let mut text = String::from("Rust");
text.push_str(" quest");       // Mutates the owned String
let length = text.len();       // Borrows text through &self
let borrowed: &str = &text;    // Borrow the String as a string slice
```

Useful methods encountered:

- `String::new()` creates an empty owned string.
- `String::from("...")` and `"...".to_string()` create owned strings.
- `.push_str(...)` appends borrowed text and needs a mutable `String`.
- `.len()` returns the byte length, not the number of human-readable characters.
- `.trim()` returns a borrowed `&str` without leading/trailing whitespace.
- `.lines()` creates an iterator over the lines of a string.

Byte slicing such as `&text[0..3]` is only safe when the indexes are valid UTF-8 boundaries. It was used with ASCII; Unicode-safe slicing still needs to be learned.

## `Vec<T>`

```rust
let mut quests = vec![String::from("Learn Result")];
quests.push(String::from("Learn traits"));

if let Some(first) = quests.first() {
    println!("{first}");
}

if let Some(quest) = quests.get_mut(0) {
    quest.push_str(" well");
}
```

Useful operations encountered:

- `vec![...]` constructs a vector.
- `Vec::from([...])` converts an array into a vector.
- `.push(value)` adds an item.
- `.first()` returns `Option<&T>`.
- `.get_mut(index)` returns `Option<&mut T>` without panicking on a bad index.
- `.pop()` removes the final item and returns `Option<T>`.

## Borrowing And Method Receivers

```rust
fn read(value: &String) {}        // Shared borrow: read only
fn change(value: &mut String) {}  // Mutable borrow: may change it
fn consume(value: String) {}      // Ownership moves into the function
```

Inside an `impl` block:

```rust
impl Quest {
    fn new() -> Quest { /* associated function: no self */ }
    fn title(&self) -> &str { /* reads this value */ }
    fn complete(&mut self) { /* mutates this value */ }
    fn into_xp(self) -> u32 { /* consumes this value */ }
}
```

- `Quest::new()` is an associated function, similar to a Java static factory method.
- `quest.title()` automatically borrows `quest` for an `&self` receiver.
- `quest.complete()` automatically mutably borrows it for an `&mut self` receiver.
- A method taking `self` consumes the value unless the type is copied.
- Mutability belongs to a binding: `let mut value = ...`.

Prefer slices for read-only collection parameters when a function does not need a specific `Vec`:

```rust
fn report(quests: &[Quest]) {}
```

## `Option<T>`

Use `Option` instead of a null value.

```rust
let note: Option<String> = Some(String::from("Practice again"));

match note {
    Some(text) => println!("{text}"),
    None => println!("No note"),
}
```

Single-pattern handling:

```rust
if let Some(text) = &note {
    println!("{text}");
}

let display = note.as_deref().unwrap_or("No note");
```

- `Some(value)` means the optional value is present.
- `None` means no value is present.
- `if let` handles one interesting pattern without writing a full `match`.
- `.as_deref()` changes a borrowed `Option<String>` into `Option<&str>` without taking ownership of the string.
- `.unwrap_or(fallback)` returns the value inside `Some`, or uses `fallback` for `None`.

The `?` operator also works with `Option` when the current function returns `Option`:

```rust
fn first_title(titles: &[String]) -> Option<&str> {
    let first = titles.first()?; // None causes an immediate return of None
    Some(&first[..])
}
```

## `Result<T, E>`

Use `Result` when an operation can fail and the failure reason matters.

```rust
fn claim_reward(completed: bool) -> Result<u32, String> {
    if completed {
        Ok(100)
    } else {
        Err(String::from("Quest is not completed"))
    }
}
```

Handle both outcomes with `match`:

```rust
match claim_reward(true) {
    Ok(xp) => println!("Claimed {xp} XP"),
    Err(error) => println!("Could not claim: {error}"),
}
```

Propagate an error with `?`:

```rust
fn reward_message() -> Result<String, String> {
    let xp = claim_reward(true)?; // Err causes an immediate return
    Ok(format!("Claimed {xp} XP"))
}
```

- `Ok(value)` means the operation succeeded and contains `value`.
- `Err(error)` means the operation failed and contains information about why.
- `format!(...)` builds and returns a `String`; unlike `println!`, it does not print it.

`?` is called the **question mark operator** and is commonly described as the **error-propagation operator**. It is not a ternary operator.

| Context | Continue with | Return early with |
|---|---|---|
| `Result<T, E>` | value inside `Ok(value)` | `Err(error)` |
| `Option<T>` | value inside `Some(value)` | `None` |

## Pattern Matching

```rust
match status {
    QuestStatus::Planned => "planned",
    QuestStatus::Active => "active",
    QuestStatus::Completed => "completed",
}
```

Patterns encountered:

```rust
0..=9             // Inclusive range pattern
1 | 3 | 5         // Alternative patterns
value if value > 5 // Match guard
value @ 1..=5     // Bind the matched value while also testing a pattern
_                 // Match anything and ignore the value
other             // Match anything and bind it to `other`
```

Rust checks exhaustive `match` expressions, so all possible variants or values must be handled.

## Loops And Iterators

Choose iteration based on ownership:

```rust
for quest in &quests {}      // Borrows each item as &T; Vec remains usable
for quest in &mut quests {}  // Borrows each item as &mut T; Vec remains usable
for quest in quests {}       // Moves each T and consumes the Vec
```

Equivalent explicit iterator methods:

- `.iter()` yields `&T`.
- `.iter_mut()` yields `&mut T`.
- `.into_iter()` yields owned `T` for an owned vector and consumes it.

Iterator tools encountered:

- `.map(...)` transforms every item.
- `.filter(...)` keeps items satisfying a condition.
- `.collect::<Vec<_>>()` consumes an iterator and builds a vector.
- `.sum()` consumes an iterator and totals its items.
- `.zip(...)` pairs items from two iterators.
- `.for_each(...)` consumes an iterator and runs a closure for each item.

Most iterator adapters such as `map` and `filter` are lazy: work happens when a consumer such as `collect`, `sum`, or `for_each` is called.

## Files And Paths

```rust
fn load_text(path: &str) -> Result<String, std::io::Error> {
    let text = std::fs::read_to_string(path)?;
    Ok(text)
}
```

- `std` means Rust's standard library, which is available without adding an external crate.
- `std::fs` is the standard-library file-system module.
- `std::fs::read_to_string(path)` reads an entire text file and returns `Result<String, std::io::Error>`.
- `std::env` provides information about the program's environment.
- `std::env::current_dir()` returns the program's current working directory inside a `Result`.
- `std::io::Error` is the standard error type used for failures involving files and other input/output operations.
- With `cargo run` from the project root, `"data/quests.txt"` is relative to the project root, not to `src/`.

Writing a complete text file:

```rust
fn save_lines(path: &str, lines: &[String]) -> Result<(), std::io::Error> {
    let text = lines.join("\n");
    std::fs::write(path, text)?;
    Ok(())
}
```

- `join("\n")` builds one owned `String` with newline separators.
- `std::fs::write` creates the file if needed. If it already exists, it truncates and replaces its contents; it does not append.
- `Result<(), E>` means success carries no additional value, while failure carries error `E`.

For large files or line-by-line processing, Rust also has streaming wrappers:

- `std::fs::File` owns an open operating-system file handle.
- `std::io::BufReader<File>` adds buffered reading, similar in purpose to Java's `BufferedReader`.
- The `std::io::BufRead` trait provides line-oriented operations.
- `std::io::BufWriter<File>` buffers writes.
- These owners close their file handles automatically when dropped; Rust's RAII/`Drop` model replaces Java's try-with-resources pattern for ordinary local ownership.

## Command-Line Arguments

```rust
match std::env::args().nth(1) {
    Some(path) => println!("Path: {path}"),
    None => println!("Usage: program <path>"),
}
```

- `args()` yields owned `String` values.
- Item `0` is normally the program name, so `.nth(1)` requests the first user argument.
- `.nth(1)` returns `Option<String>` because the argument might not exist.
- Prefer validation with `match` or another deliberate `Option` method instead of `unwrap()` in user-facing code.

## Basic Modules

```rust
// main.rs
mod storage;
storage::save_quest_titles(path, titles);

// storage.rs
pub fn save_quest_titles(...) { ... }
```

- `mod storage;` includes `src/storage.rs` as a module.
- Items are private by default; `pub` exposes an item to its parent module.
- `storage::name` is an explicit module path.
- Inside a child module, `super::name` refers to an item in its parent module.

## Focused Unit Test

```rust
#[cfg(test)]
mod tests {
    use super::format_quest_titles;

    #[test]
    fn formats_two_titles() {
        assert_eq!(format_quest_titles(/* input */), /* expected */);
    }
}
```

- `#[cfg(test)]` includes the module only in test builds.
- `#[test]` marks a function for `cargo test`.
- `assert_eq!(actual, expected)` fails the test when the values differ.

## Line Parsing Recipe

Clear loop version:

```rust
let mut titles = Vec::new();

for line in text.lines() {
    let trimmed = line.trim();
    if !trimmed.is_empty() {
        titles.push(trimmed.to_string());
    }
}
```

Approved iterator-chain version:

```rust
let titles: Vec<String> = text
    .lines()
    .map(|line| line.trim())
    .filter(|line| !line.is_empty())
    .map(|line| line.to_string())
    .collect();
```

Filtering happens before `to_string()`, so discarded empty lines do not allocate owned strings. If the iterator item is `&str`, `filter` passes `&&str` to its predicate because it borrows each item while deciding whether to keep it; method-call auto-dereferencing makes `line.is_empty()` work naturally.

## Quick Ownership Check

When unsure, ask:

1. Who owns this value now?
2. Am I passing `T`, `&T`, or `&mut T`?
3. Does this loop or method borrow the value or consume it?
4. Will I need the original value again afterward?
