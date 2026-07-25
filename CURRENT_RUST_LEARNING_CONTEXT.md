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

- Added tuple-like enum variant `Resize(u32, u32)`.
- Kept `handle(message: &Message)` exhaustive and matched `Message::Resize(height, width)`.
- Created one owned `Message::Resize(800, 600)` value and called `handle(&resize)` twice.
- This proved the same enum value can be inspected repeatedly through shared borrowing.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` passed.

## Current Weak Spots

- Match ergonomics with borrowed enum data is improving, but should be revisited later with `Option<T>` and `Result<T, E>`.
- Tuple-like enum variant field order matters; names like `height` and `width` are created by the pattern, not stored as field names.
- The next step is learning how Rust represents "maybe a value" without `null`.

## Current Next Concept

`Option<T>` as Rust's safe replacement for nullable values.

## Next Small Exercise

Create a function `find_message(id: u32) -> Option<Message>`.
Return `Some(Message::Text(...))` for one id and `None` for another.
In `main`, call the function twice and use `match` to handle both `Some(message)` and `None`.
