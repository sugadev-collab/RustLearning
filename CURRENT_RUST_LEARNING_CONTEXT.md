# Current Rust Learning Context

This compact file is the first file to read at the start of a Rust review or lesson.
Use the larger `docs/LEARNING_PROGRESS.md` and `docs/RUST_DOUBTS_AND_QUESTIONS.md` only for targeted lookup.

## Workflow Preference

- Keep token usage low by reading only the compact context first.
- Avoid dumping the full large progress/doubts files into chat.
- Keep long notes and recap files under `docs/` so the project root stays focused on code and Cargo files.
- If this file grows too large, archive older detail into a dated `docs/RUST_CONTEXT_ARCHIVE_YYYY_MM_DD.md` file and rewrite this file to the small current state.
- Do not read archive files by default; use them only for targeted lookup.
- Git commits are allowed when useful, but keep them small and do not stage generated build artifacts.
- Root cleanup on 2026-07-28 moved large learning docs into `docs/`; no source code was edited for that cleanup.

## Latest Approved Exercise

- Converted `describe_quest(quest: &Quest)` into a method `Quest::describe(&self)`.
- Called the method with `quest.describe()` from both full and completed-only loops.
- Kept the Quest Log Analyzer behavior unchanged.
- Used `&self` correctly to borrow quest data without consuming it.
- Renamed `Inprogress` to idiomatic `InProgress`.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` passed.

## Current Weak Spots

- Match ergonomics with borrowed enum data is improving, but should be revisited later with `Option<T>` and `Result<T, E>`.
- `Option<T>` is improving: the learner can now return and match both `Some(...)` and `None`.
- The wildcard pattern `_` and named fallback bindings have each been demonstrated once; reinforce them later in enum and `Option` matches before calling them fully learned.
- Inclusive range patterns with `..=` have been demonstrated once.
- Alternative patterns with `|` have been demonstrated once.
- Match guards with `if` have been demonstrated once, including the importance of arm order.
- Binding inside patterns with `@` has been demonstrated once; learner asked what `@` represents, so explain again briefly next time.
- Simple unit enums and exhaustive matching over enum variants have been demonstrated once.
- Enums with attached data have been demonstrated once; reinforce how destructuring moves owned fields such as `String`.
- Borrowing an enum with attached data has been demonstrated once; reinforce match ergonomics and what type the inner bindings have.
- Struct-like enum variants have been demonstrated more than once; move forward instead of repeating this topic again soon.
- `if let` for a single enum variant has been demonstrated once; reinforce next with `Option<T>`.
- `if let` with `Option<T>` has been demonstrated once using a borrowed `&Option<String>`; next clarify by-value versus by-reference ownership behavior.
- `Option<String>` by value versus `&Option<String>` has been demonstrated once; reinforce the clearer pattern of owning the variable and borrowing it at call sites.
- `if let ... else` with `Option<String>` has been demonstrated once.
- `while let` with `Vec::pop()` and `Option<T>` has been demonstrated once.
- Borrowed `for` iteration with `for number in &numbers` has been demonstrated once; learner showed the vector remains usable after the loop.
- Consuming `for` iteration has been demonstrated once; reinforce that the whole `Vec` is moved into `IntoIter`, rather than becoming an accessible empty vector.
- Mutable borrowed iteration with `for number in &mut numbers` has been demonstrated once; reinforce that `number` has type `&mut i32` and requires dereferencing to change the underlying element.
- Explicit borrowed iterator methods `.iter()` and `.iter_mut()` have been demonstrated once; reinforce that they are method forms of shared and mutable borrowing.
- Explicit consuming iterator method `.into_iter()` has been demonstrated once with `Vec<String>`; reinforce that the vector binding moves into the iterator and cannot be used afterward.
- Lazy `.map()` and terminal `.for_each()` have been demonstrated once; reinforce that adapters wrap and own earlier iterators without immediately traversing them.
- `.collect::<Vec<_>>()` has been demonstrated once.
- Concrete iterator adapter types implementing the `Iterator` trait are still new.
- `.zip()` has been demonstrated once.
- Borrowed iterator input versus consumed iterator input is improving: `zip(lengths)` consumes, `zip(lengths.iter())` and `zip(&lengths)` borrow.
- `.sum()` has been demonstrated once as an iterator consumer/reducer.
- `.filter()` has been demonstrated once.
- In `.iter().filter(|num| ...)`, the closure parameter can feel like `&&i32`; revisit iterator item references when needed.
- Variable shadowing with `let numbers = ...` was used correctly and can be reinforced later.
- Big checkpoint passed once using structs/enums, `Option`, `match`, borrowing, slices, and iterator chains together.
- `if let ... else` was used correctly in the checkpoint, but the learner had to look up the syntax; reinforce later with `Option`.
- Slice parameters `&[T]` were introduced through Clippy and successfully applied.
- String byte slicing was guarded by `.len()` and used only with ASCII; revisit Unicode-safe handling later.
- Method receiver `&self` has been demonstrated once in `impl Quest`.
- Returning borrowed string data from methods is a good next step.
- Learner asked why `Vec::from([..])` avoids Clippy's `useless_vec` warning while `vec![..]` does not.
- Learner wants a compact refresher on Rust operators and bitwise operators later.
- Expression style and `match` readability are improving, but should be reinforced through small exercises.
- Watch small spelling/readability issues such as `lenght`.

## Current Next Concept

Methods returning borrowed data.

## Next Small Exercise

Add a method `fn tag_label(&self) -> &str` to `impl Quest`.
It should return the tag text when `tag` is `Some`, and `"No tag"` when it is `None`.
Use this method inside `describe(&self)` instead of writing the `if let` directly there.
