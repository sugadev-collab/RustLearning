# Big Exercise 01: Text Note Recap

This is your first larger Rust recap exercise. Do it when you have time. It does not block the normal small lessons.

## Goal

Build a small text-note program that shows you understand ownership, borrowing, string slicing, UTF-8 byte positions, mutation, structs, lifetimes, and `Box<T>`.

Use only `src/main.rs`. Do not use external crates. ## but i need external file so i can do this excersise and big excersise in parallel

## Part 1: Owned Note

Create a struct named `Note` with these fields:
+++

- `title: String`
- `body: String`

Create an associated function:

```rust
impl Note {
    fn new(title: String, body: String) -> Self {
        // your code
    }
}
```

In `main`, create one `Note` with a title and a body.

Print the title and body using normal `{}` formatting.

## Part 2: Borrowed Preview

Create another struct named `Preview<'a>` with these fields:

- `title: &'a str`
- `first_word: &'a str`

Create a `Preview` value that borrows from your `Note`.

Use an ASCII first word first, so slicing is simple, for example:

```text
Rust language is fast
```

Print the preview.

Add comments explaining:

- What owns the title and body?
- What does `Preview<'a>` borrow from?
- Why does `Preview<'a>` need a lifetime?

## Part 3: Mutation After Last Borrow Use

After printing the preview, mutate the original note body using `replace_range`.

Example idea:

```rust
note.body.replace_range(0..4, "Safe");
```

Print the mutated body.

Then add a commented-out line showing that using the old preview after mutation would be invalid.

Add a comment explaining why mutation is allowed only after the preview's last use.

## Part 4: UTF-8 Inspection

Create a second body string containing at least one non-ASCII word, for example Tamil text or any Unicode text you choose.

Print:

- `.len()`
- `.chars().count()`
- `.bytes().enumerate()`
- `.char_indices()`

Then create one valid string slice using byte positions from `.char_indices()`.

Add a comment explaining why byte positions and character-count positions are different.

## Part 5: Boxed Note

Put one `Note` inside a `Box<Note>`.

Print:

- The boxed note's title
- The pointer address held by the box using `{:p}`

Move the boxed note into a second variable.

Add a commented-out line showing why the first variable cannot be used after the move.

## Research Challenge

Look up or experiment with this small question:

Can a struct that contains `String` derive `Debug`?

If yes, use `#[derive(Debug)]` on `Note` and print the full boxed note with `{:?}`.

## Review Checklist

Before giving it to me, run:

```text
cargo check
cargo clippy
cargo run
```

It is okay if you get stuck. Bring me your code and I will review no more than three important problems at a time.
