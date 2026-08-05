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
- Optimize for durable understanding rather than a fixed-day or "intermediate in 30 days" target. Keep important concepts exercise-based and unhurried; lower-priority details may be explained briefly in chat.
- For big exercises, give the full requirements file at once and let the learner build the whole attempt independently. Do not require approval after each numbered part unless the learner asks for step-by-step help.
- Root cleanup on 2026-07-28 moved large learning docs into `docs/`; no source code was edited for that cleanup.

## Latest Approved Exercise

- Completed the full Quest Review consolidation exercise independently.
- `Quest::new` accepts only title, reward XP, and note, and always initializes the status as `Planned`.
- Demonstrated `self`, `&self`, and `&mut self` receivers, safe vector mutation, `Option`, exhaustive status matching, a borrowed `&[Quest]` report, and `filter`/`map`/`sum`.
- Reused `quests` after the XP report, proving that the slice/iterator analysis did not consume the vector.
- Used a separate quest for the consuming method so the main vector remained available.
- `cargo check`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and `cargo run --quiet` all passed on 2026-08-05.

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
- Full Quest Review consolidation passed independently once using associated functions, all three receiver forms, vector mutation, optional data, slices, and a non-consuming iterator report.
- `if let ... else` was used correctly in the checkpoint, but the learner had to look up the syntax; reinforce later with `Option`.
- Slice parameters `&[T]` were introduced through Clippy and successfully applied.
- String byte slicing was guarded by `.len()` and used only with ASCII; revisit Unicode-safe handling later.
- Method receiver `&self` has been demonstrated once in `impl Quest`.
- Returning borrowed string data from methods has been demonstrated once with `fn tag_label(&self) -> &str`.
- Method receiver `&mut self` has been demonstrated once with `fn complete(&mut self)`.
- Method receiver `self` has been demonstrated once with `fn into_reward_xp(self) -> u32`.
- `Vec::get_mut(index)` has been demonstrated once for safely borrowing one mutable element.
- Calling an `&self` method through an `&mut T` reference has been questioned once; reinforce reborrowing later.
- Reborrowing/reading through `&mut String` has been demonstrated once with `&text_ref[0..6]`.
- Borrow lifetimes ending after last use has been demonstrated once with `slice` then a later `text_ref.push_str(...)`.
- Double references such as `&&mut String`, auto-deref, and indexing through references are still active questions; simplify with smaller examples later.
- Mutability belongs to bindings, not values, has been demonstrated once with `let text` then `let mut text_mut = text`.
- Learner asked whether mutability moves with ownership; answered that ownership moves the value, while `mut` belongs to the new binding.
- Method-call auto-borrowing for `&mut self` has been demonstrated once with `text.push_str(...)`.
- Wording to reinforce: `push_str` is a method, and the temporary borrow ends after the method call, not because of a visible local parameter variable in user code.
- Explicit receiver syntax has been demonstrated once with `String::push_str(&mut text, ...)`.
- Method-call auto-borrowing for `&self` has been demonstrated once with `text.len()` and compared with `String::len(&text)`.
- Distinguishing receiver methods from associated functions (the closest Rust equivalent to Java static methods) has been demonstrated once; reinforce later inside `impl` blocks.
- Lifetime elision for returning data borrowed from `&self` should be explained soon.
- String literals as `&'static str` were used implicitly as a fallback return value.
- Learner asked why `Vec::from([..])` avoids Clippy's `useless_vec` warning while `vec![..]` does not.
- Learner wants a compact refresher on Rust operators and bitwise operators later.
- Expression style and `match` readability are improving, but should be reinforced through small exercises.
- Watch small spelling/readability issues such as `lenght`.

## Current Next Concept

`Result<T, E>` for operations that can succeed or fail.

## Next Small Exercise

Write a small `claim_reward(status: QuestStatus, reward_xp: u32) -> Result<u32, String>` function. Return `Ok(reward_xp)` only for `Completed`; return a useful `Err(String)` for `Planned` and `Active`. Call it once for each status and use `match` to print the success or error. Let the learner attempt it before providing a complete solution.

## Roadmap Snapshot

- Overall progress toward intermediate Rust: about 40%.
- Ownership/borrowing foundation: about 60% toward the target level.
- Independently reusable mastery: about 30-35%; several newer concepts still need repetition in larger programs.
- For the learner's remaining nine-day focused period, spend exercise time on high-value concepts and explain lower-priority details briefly in chat.
- The 2026-08-05 Quest Review consolidation checkpoint is complete and approved.
- Teach `Result` next, followed by error propagation and basic file input carefully; then extend or revisit the project with those concepts.
- The stable roadmap is `docs/RUST_LEARNING_ROADMAP.md`; the current exercise specification is `docs/QUEST_REVIEW_PROJECT.md`.
- In the first chat on 2026-10-09, remind the learner about the project-planning checkpoint; if there is no chat that day, remind them in the first later chat. Reassess readiness, then create/update a comprehensive real-project and AI-handoff plan; do not start the project based only on the date.
