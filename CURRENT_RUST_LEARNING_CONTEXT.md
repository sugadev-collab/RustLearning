# Current Rust Learning Context

This compact file is the first file to read at the start of a Rust review or lesson.
Use the larger `LEARNING_PROGRESS.md` and `RUST_DOUBTS_AND_QUESTIONS.md` only for targeted lookup.

## Workflow Preference

- Keep token usage low by reading only the compact context first.
- Avoid dumping the full large progress/doubts files into chat.
- If this file grows too large, archive older detail into a dated `RUST_CONTEXT_ARCHIVE_YYYY_MM_DD.md` file and rewrite this file to the small current state.
- Do not read archive files by default; use them only for targeted lookup.
- Git commits are allowed when useful, but keep them small and do not stage generated build artifacts.

## Latest Approved Exercise

- Borrowed enum match exercise using `fn handle(message: &Message)`.
- `handle(&msg)` can be called twice because the caller keeps ownership of the enum.
- Matching on `&Message` borrowed the inner data instead of moving it out.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` passed.

## Current Weak Spots

- Match ergonomics with borrowed enum data should be practiced more.
- In `match message` where `message: &Message`, patterns like `Message::Text(text)` work because Rust automatically handles reference patterns in common cases.
- The inner binding such as `text` is borrowed from the enum; it is not an owned `String`.

## Current Next Concept

Match ergonomics with borrowed enum data.

## Next Small Exercise

Add another enum variant:

```rust
Resize(u32, u32)
```

Then update `handle(&Message)` to match it and print width/height.
Create one `let msg = Message::Resize(800, 600);` and call `handle(&msg)` twice.
