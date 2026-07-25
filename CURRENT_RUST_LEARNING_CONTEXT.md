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

- Created `find_message(id: u32) -> Option<Message>`.
- Used `match id` to return `Some(Message::...)` for ids `0`, `1`, `2`, and `3`, and `None` for all other ids.
- In `main`, looped over `0..5_u32` and matched each `Option<Message>`, exercising both `Some(m)` and `None`.
- Used direct `Some(...)` and `None` instead of `Option::Some(...)` and `Option::None`.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` passed.

## Current Weak Spots

- Match ergonomics with borrowed enum data is improving, but should be revisited later with `Option<T>` and `Result<T, E>`.
- `Option<T>` is improving: the learner can now return and match both `Some(...)` and `None`.
- The wildcard pattern `_` in `match` is a current question.
- Expression style and `match` readability are improving, but should be reinforced through small exercises.

## Current Next Concept

Wildcard patterns with `_` and match exhaustiveness.

## Next Small Exercise

Create a `describe_number(n: u32)` function using `match`.
Handle `0`, `1`, `2`, and `_`; print a different message for each arm.
