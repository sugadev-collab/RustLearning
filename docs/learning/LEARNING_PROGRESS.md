# Learning Progress

# 2026-10-31 - Complete Command Argument Parsing Fix Approved

### Demonstrated successfully
- Reviewed committed change `f93ad50` in `src/main.rs`: changed the `Complete` branch from `arg.nth(2)` to `arg.next()`, correctly retrieving the next argument after the command name has been consumed.
- Both `Start` and `Complete` parser branches now use immediate next-item retrieval.

### Still practicing
- Actual CLI execution was not run by the tutor; parsing correctness does not guarantee successful loading or status transitions.
- `Add` still uses `enumerate()` after the executable and command have already been consumed, discarding title and XP. Its command executor currently overwrites stored quests with a one-item vector, so do not run it against real data yet.

### Review result
Approved the focused Complete parser correction, not the entire Quest Tracker.

### Next concept
Collecting remaining iterator values into a vector and using zero-based `get()` for required and optional arguments.

### Next small exercise
Rewrite only `command_identify`'s Add argument extraction to accept title, XP, and optional note from the remaining iterator. Demonstrate the parsed values without invoking the data-overwriting Add executor.

# 2026-10-31 - Start Command Argument Parsing Fix Approved

### Demonstrated successfully
- Committed `a52f1d` ("nth(2) problem fixed"), changing `command_identify`'s `Start` branch from `arg.nth(2)` to `arg.next()` after consuming the command.
- Restored ordinary CLI dispatch in `main()` and removed the temporary iterator practice function.
- GitHub diff inspection confirms the focused parser correction and reinforces that `nth()` advances from the current position.

### Still practicing
- The `Complete` branch uses the same incorrect `arg.nth(2)`; the `Add` branch has a separate argument-position problem.
- File loading still has mismatched paths and potential data-loss behavior. Do not confuse parsing success with complete command success.
- Reviewer inspected the committed source/diff; Cargo was unavailable in the reviewer environment, and no runtime result was independently verified.

### Review result
Approved the Start argument-retrieval fix, not the whole command parser or the entire Quest Tracker.

### Next concept
Reinforce sequential command-line argument consumption via `Iterator::next()`.

### Next small exercise
Apply the same lesson to `Complete` only. Run `cargo check` and `cargo run -- complete 1`, then share changed lines and command output.

# 2026-10-31 - Iterator Positioning Checkpoint

### Demonstrated successfully
- Read the standard-library explanation of `Iterator::nth` and experimented with repeated calls on a vector iterator.
- Correctly predicted that `nth(1)`, `nth(1)`, `next()` over `[10, 20, 30, 40, 50]` produces `Some(20)`, `Some(40)`, `Some(50)`.
- Explained that `nth` advances the current iterator rather than indexing the original collection on every call.

### Still practicing
- Apply this concept to the `command_identify` CLI parsing bug. The user's earlier observation that `nth(2)` returned `None` after a prior `nth(1)` was due to insufficient elements remaining, not single-use iterators.
- Do not count the Quest Tracker parser as fixed until code and representative command output have been reviewed.

### Review result
Approved conceptual iterator positioning checkpoint; no source edits by tutor.

### Next concept
Using `Args::next()` for sequential command-line argument parsing.

### Next small exercise
Fix only the Start command's argument retrieval in `command_identify` and test `cargo run -- start 1`, allowing unrelated file I/O errors to remain for later lessons.

# 2026-10-28 - Cross-Function Result Propagation Approved

### Demonstrated successfully
- Implemented `double_xp(text: &str) -> Result<u32, std::num::ParseIntError>` by calling `parse_xp(text)?` and returning `Ok(number * 2)`.
- In `main`, used `println!("Result: {:?}", double_xp("bad"))` to inspect the error. The snippet correctly propagates parse failure from `parse_xp` to `double_xp`, skipping multiplication on error.

### Still practicing
- Reuse `?` in actual Quest Tracker operations and reason through error ownership, logging, and persistence safety.
- Numeric multiplication can overflow for very large `u32` values; defer checked arithmetic until needed.
- The sample was reviewed from pasted code and not independently compiled or executed.

### Review result
Approved this focused `?` exercise, not the complete Quest Tracker.

### Next concept
Stateful iterators: `Iterator::next` versus `Iterator::nth`, especially on `std::env::Args`.

### Next small exercise
Predict and run successive `next()`/`nth(1)` calls on a three-item vector iterator, then identify why the real `command_identify` function skips a command argument.

# 2026-10-28 - Basic Question-Mark Error Propagation Approved

### Demonstrated successfully
- Implemented `fn parse_xp(text: &str) -> Result<u32, std::num::ParseIntError>` with `let number = text.parse::<u32>()?; Ok(number)`.
- Correctly placed `?` on a fallible parse, then wrapped the success value in `Ok`.
- Reviewed from the pasted function; tests/compilation not independently run.

### Still practicing
- Reinforce `?` across multiple helper functions, then apply it to safe project loading and validation.
- Remember `?` returns early from the enclosing `Result`-returning function on errors; it does not log or suppress them.

### Review result
Approved focused `?` usage; entire Quest Tracker remains under development.

### Next concept
Propagate errors across helper-function boundaries.

### Next small exercise
Implement `double_xp(text: &str) -> Result<u32, std::num::ParseIntError>` by calling `parse_xp(text)?`, multiplying the successful value by 2, and wrapping it in `Ok`. Test "100" and "bad".

# 2026-10-28 - Result Collection And Type Inference Reinforcement

### Demonstrated successfully
- Wrote a practice function collecting `Vec<Result<i32, &str>>` into `Result<Vec<i32>, &str>` and comparing error and all-success cases.
- Used `match` to handle the collected `Ok` and `Err` results.
- Explained that Rust uses static types and infers generic types from known element types. Clarified compile-time type knowledge versus runtime value knowledge and that a string literal has `&'static str` type.
- The exercise was reviewed from pasted source; no compiler or runtime verification was performed by the tutor.

### Still practicing
- Apply `Result` error flow to real helpers. Contrast preserving errors with log-and-skip read-only decisions and ensure mutable commands do not rewrite a partial load.
- New concept is `?` for early error propagation, returning the success value for use on the next line.

### Review result
Approved the focused `Result` collection and type-inference exercise, not the entire CLI project.

### Next concept
The `?` operator with `Result` error propagation.

### Next small exercise
Implement `parse_xp(text: &str) -> Result<u32, std::num::ParseIntError>` using `?` and test inputs "100" and "bad".

# 2026-10-24 - Modules And Implicit Return Reinforcement Approved

### Demonstrated successfully
- Read existing cross-module `Quest` code and wrote an independent temporary practice function using `Quest::new`, `Status::Active`, getters, and `quest::status_to_string`.
- Explained the practical distinction between `mod`, `pub`, `use`, `crate::`, and `super::`, with additional questions on nested file/module layout.
- Understood that a final `match` expression can supply a function return value without `return`; fixed `list() -> Option<Vec<Quest>>` so successful reads return `Some(quests)` and read errors return `None`.
- Commit reviewed: `7e2a6da` ("list fn defect fixed"). The fix to the return expression is approved; entire Quest Tracker Version 1 is not yet approved.

### Still practicing
- `filter_map` extracts `Some` values from the closure and skips `None`; it does not automatically unwrap `Result::Ok`. Conversion via `.ok()` or explicit matches is deliberate, and can discard errors.
- Learner prefers logging malformed quests and allowing read-only display to proceed where sensible. Safe persistence requires preventing `start` or `complete` from overwriting missing/skipped records; Version 1 requirements call for line-aware errors and preservation of original data on invalid input.
- Known remaining defects: mismatched absolute/relative data path, problematic `Args::nth` usage, `add` overwriting earlier quests, and risk of saving a filtered partial load.
- No local Cargo checks, tests, or program execution were run by the reviewing tutor; review was of committed GitHub source.

### Review result
Approved the focused module refresher and the `list()` return-value fix only. Keep learner-owned source untouched unless the learner explicitly requests edits.

### Next concept
Error-preserving iteration using `Result`: compare `filter_map` with `map` and `collect::<Result<Vec<_>, _>>()`.

### Next small exercise
Experiment with `Vec<Result<i32, &str>>` containing all `Ok` and one `Err`; collect into a single `Result<Vec<i32>, &str>` and explain what happens.

# 2026-08-08 - First Focused Unit Test Approved

### Demonstrated successfully
- Extracted file-content formatting into a private pure helper and kept the public save function using it.
- Placed tests in a `#[cfg(test)]` child module inside `storage.rs`.
- Used `super::format_quest_titles` to access the private helper in the parent `storage` module.
- Marked a test with `#[test]` and verified actual output with `assert_eq!`.
- Corrected the earlier mistake of putting `use super::...` in the crate root and attaching `#[cfg(test)]` directly to production functionality.

### Still practicing
- The learner adapted an external example successfully but does not yet understand `mod`, `super`, or `assert_eq!` deeply; reinforce them in the Quest Tracker before treating tests or module paths as mastered.
- Test names should become more behavior-specific as the project grows.
- Add edge cases only when they clarify real project behavior rather than collecting tests mechanically.

### Review result
Approved. One unit test passed. `cargo check`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and the normal argument-supplied program run also passed.

### Next concept
Project decomposition: assigning model, storage, and command-line responsibilities to clear modules.

# 2026-08-08 - First Source Module Approved

### Demonstrated successfully
- Created `src/storage.rs` and declared it from the crate root with `mod storage;`.
- Moved the file-save helper into the storage module without changing its behavior.
- Used `pub fn` to make the helper visible to its parent module.
- Called the helper through the explicit module path `storage::save_quest_titles(...)`.
- Preserved the owned command-line path and borrowed title slice across the module boundary.
- Added the new source file to Git tracking without staging unrelated working-tree changes.

### Still practicing
- Module declarations, module paths, and visibility have been demonstrated once and should recur naturally in the Quest Tracker.
- Keep distinguishing a private module declaration from a public item inside that module.
- Project code still needs focused tests before the full Quest Tracker readiness gate.

### Review result
Approved. `cargo check`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and both runtime argument cases passed.

### Next concept
Focused unit tests with `#[cfg(test)]`, `#[test]`, and `assert_eq!`.

# 2026-08-08 - Command-Line Output Path Approved

### Demonstrated successfully
- Used `std::env::args().nth(1)` to request the first user-supplied command-line argument.
- Matched the resulting `Option<String>` without panicking.
- Kept ownership of the path `String` in the `Some(path)` arm and borrowed it as `&str` for the save helper.
- Gave the missing-argument case separate behavior instead of assuming an argument existed.
- Successfully tested both an argument-supplied run and a missing-argument run.

### Still practicing
- Exact standard-library API names may still require the cheat sheet or documentation; that is normal while the underlying `Option`, ownership, and error-handling concepts become durable.
- Explicit dereferencing, deref coercion, and method-call auto-dereferencing need recurring examples so they are not treated as the same mechanism.
- Avoid building a habit of `unwrap` in user-facing project code; prefer `match`, `?`, fallbacks, or deliberate validation depending on the situation.

### Review result
Approved. `cargo check`, `cargo clippy -- -D warnings`, and `cargo fmt --check` passed. `cargo run --quiet -- data/quests_copy.txt` saved successfully, and `cargo run --quiet` reported the missing path without panicking.

### Next concept
Basic Rust modules, `mod`, and visibility with `pub`.

# 2026-08-08 - Text File Writing Approved

### Demonstrated successfully
- Wrote a helper accepting a borrowed `&[String]` and returning `Result<(), std::io::Error>`.
- Used `titles.join("\n")` to build the complete file contents before writing.
- Used `std::fs::write(path, joined)?` to overwrite the destination and propagate I/O failure.
- Returned `Ok(())`, correctly using the unit type because a successful write has no additional value to return.
- Handled the returned `Result` with distinct `Ok(())` and `Err(error)` match arms.
- Investigated why calling `std::fs::write` inside a loop retained only the final title: every call truncated and replaced the file.

### Still practicing
- `Result`, `?`, and I/O errors are improving but should recur in the real project before being considered independently mastered.
- Returning a borrowed reference to a temporary error string compiled in this example, but branch-local printing or an owned `String` will usually make the lifetime and ownership clearer.
- File overwrite behavior is understood once; append mode and structured persistence can be introduced only when the project needs them.

### Review result
Approved. `cargo check`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and `cargo run --quiet` all passed, and `data/quests_copy.txt` contained all four expected titles.

### Next concept
Command-line arguments with `std::env::args` and `Option`-based input validation.

# 2026-08-07 - Iterator-Chain File Parsing Approved

### Demonstrated successfully
- Refactored `load_quest_titles` from a manual `Vec` and `for` loop into a lazy iterator chain.
- Used `.lines()` to yield borrowed `&str` items and `.map(...)` to trim them without allocating.
- Used `.filter(...)` before converting retained titles into owned `String` values.
- Used `.collect()` with the function's `Vec<String>` return type supplying the required type context.
- Corrected the first attempt, which allocated a `String` before filtering, so empty lines no longer allocate unnecessarily.

### Still practicing
- Iterator item types and adapter closure parameter types need repetition, especially that `filter` borrows each iterator item for its predicate.
- `map`, `filter`, and `collect` have now been used successfully in several focused examples, but should continue recurring in file and project work before being considered independently mastered.
- `Option`, `Result`, and `?` should continue recurring in the next file-output exercise.

### Review result
Approved. `cargo check`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and `cargo run --quiet` all passed.

### Next concept
Writing text files with `std::fs::write` and propagating I/O errors.

# 2026-08-07 - Project-Relative File Path Approved

### Demonstrated successfully
- Replaced the absolute file path with the project-relative path `data/quests.txt`.
- Printed `std::env::current_dir()` to observe that `cargo run` executes from the project root.
- Kept `load_quest_titles(...)`, `first_title(...)`, and the `match`-based file error handling working with the relative path.

### Still practicing
- `current_dir()` returns a `Result`, and the current `if let Ok(...)` handles only the success path. That is fine for a debug print, but future user-facing code should decide what to do on failure.
- Keep distinguishing paths relative to Cargo's working directory from paths relative to `src/`.

### Review result
Approved. `cargo check`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and `cargo run --quiet` all passed.

### Next concept
Iterator-chain collection with `map`, `filter`, and `collect`.

# 2026-08-07 - File Reading And `Option` Question Mark Approved

### Demonstrated successfully
- Created `data/quests.txt` with one quest title per line.
- Wrote `load_quest_titles(path: &str) -> Result<Vec<String>, std::io::Error>`.
- Used `std::fs::read_to_string(path)?` to propagate file read errors.
- Parsed file contents with `.lines()`, trimmed each line, skipped empty lines, and collected owned `String` values.
- Wrote `first_title(titles: &[String]) -> Option<&str>` using `titles.first()?`.
- Used `.unwrap_or(...)` to provide a fallback for `None`.
- Cleaned old unused Quest demo code so the current exercise is focused and Clippy-clean.

### Still practicing
- `?` has now been used once with `Result` and once with `Option`; keep reinforcing that it returns early with `Err(...)` or `None`.
- File paths are currently absolute; later exercises should prefer project-relative paths where possible.
- Iterator collection was implemented with a clear loop; later revisit the iterator-chain version after file I/O feels comfortable.

### Review result
Approved. `cargo check`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and `cargo run --quiet` all passed.

### Next concept
Project-relative file paths and `std::env::current_dir`.

# 2026-08-06 - `?` Error Propagation Exercise Approved

### Demonstrated successfully
- Added `reward_message(quest: &Quest) -> Result<String, String>`.
- Used `quest.claim_reward()?` to continue with XP on `Ok(...)` and return early on `Err(...)`.
- Built a success message after the `?` operator, proving that code after `?` only runs on the success path.
- Called `reward_message(...)` for every quest by iterating over `&quests`.
- Handled each returned `Result<String, String>` with one `match`.

### Still practicing
- `?` has now been demonstrated once; keep reinforcing that it is early error return, not a ternary operator.
- The current `match` stores either success or error text in a single `message` variable, which is valid; later exercises can make success and failure branches more visibly different.
- Continue keeping `Result`-returning helper functions small and easy to reason about.

### Review result
Approved. `cargo check`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and `cargo run --quiet` all passed.

### Next concept
Basic file reading with `std::fs::read_to_string`.

# 2026-08-06 - First `Result<T, E>` Reward Exercise Approved

### Demonstrated successfully
- Added `Quest::claim_reward(&self) -> Result<u32, String>`.
- Returned `Ok(self.reward_xp)` only when the quest status is `Completed`.
- Returned separate `Err(String)` messages for `Planned` and `Active`, showing that `Result` can explain different failure reasons.
- Updated the reward report to call `claim_reward()` once per quest and handle the returned `Result` with `match`.
- Demonstrated `Planned`, `Active`, and `Completed` statuses in the runtime output.

### Still practicing
- `Result<T, E>` has been demonstrated once; keep reinforcing when to return `Ok`, when to return `Err`, and how error messages represent the reason an operation failed.
- Small output polish remains separate from Rust correctness, such as leading spaces in messages and spelling in printed text.
- Continue preferring one evaluation plus `match` when the program needs both success and error handling.

### Review result
Approved. `cargo check`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and `cargo run --quiet` all passed.

### Next concept
Propagating errors with the `?` operator.

# 2026-08-05 - Quest Review Consolidation Approved

### Demonstrated successfully
- Implemented `Quest::new(title, reward_xp, note)` as an associated function that always initializes `status` to `QuestStatus::Planned`.
- Used `&self`, `&mut self`, and consuming `self` receivers for reading, mutation, and ownership transfer.
- Built and mutated a `Vec<Quest>` with `Vec::get_mut`, handled `Option<String>` with `if let`, and described every status with `match`.
- Passed a borrowed quest slice to a report function and used `iter().filter(...).map(...).sum()` to total completed XP.
- Used `quests` again after the XP report, proving that the slice and iterator chain borrowed rather than consumed the vector.
- Demonstrated the consuming receiver on a separate quest so the main vector remained usable.

### Still practicing
- These concepts are now consolidated in one independent program, but receiver selection, borrowed iterator item types, and `Option` handling should continue to recur in later projects before being considered mastered.
- The `Active` state is assigned and therefore satisfies the exercise; in a future user-facing program, state transitions should also be made observable when that behavior matters.
- Small text/readability cleanup can be done later, but it is not a Rust correctness issue.

### Review result
Approved. Every requirement in `docs/projects/QUEST_REVIEW_REQUIREMENTS.md` is present. `cargo check`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and `cargo run --quiet` all passed.

### Next concept
`Result<T, E>` for operations that can succeed or fail.

# 2026-08-04 - Quest Review Skeleton Approved

### Demonstrated successfully
- Translated an explicit data-model requirement into a three-variant `QuestStatus` enum.
- Created the requested `Quest` struct with `String`, `u32`, `QuestStatus`, and `Option<String>` fields.
- Added the custom `impl Quest` block and kept the phase intentionally free of behavior.

### Still practicing
- The enum and struct are currently unused, so dead-code warnings are expected until later phases construct and inspect quests.
- The complete Quest Review requirements combine associated functions, every receiver form, collections, `Option`, matching, borrowing, slices, and iterator reporting.
- For big exercises, the learner prefers receiving the complete specification and working independently instead of mandatory one-by-one approval gates.

### Review result
Phase 1 approved. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run --quiet` passed with only the two expected dead-code warnings.

### Next concept
Independent consolidation of associated functions, method receivers, collections, and borrowing.

# 2026-08-04 - Associated Function Versus Method Approved

### Demonstrated successfully
- Created a `String` with `String::new()`.
- Correctly identified `String::new()` as an associated function with no `self` receiver.
- Correctly identified `text.len()` as a method call whose receiver is borrowed as `&self`.
- Used `_len` intentionally to avoid an unused-variable warning during a concept-only exercise.

### Still practicing
- Reinforce the distinction later inside custom `impl` blocks, where both receiver-free associated functions and receiver methods can live together.
- Keep separating broad wording from precise wording: all functions inside an `impl` are associated with the type, but only receiver functions are called as methods.

### Review result
Approved. `cargo check`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and `cargo run --quiet` all passed.

### Next concept
Big consolidation exercise using existing concepts.

## 2026-08-03 - Immutable Method Auto-Borrowing Approved

### Demonstrated successfully
- Created an immutable `String` binding named `text`.
- Called `text.len()` using normal method-call syntax and implicit shared borrowing.
- Called `String::len(&text)` using the explicit receiver form.
- Printed equal results from both forms.
- Removed the unnecessary `mut` that Clippy identified in the first attempt.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- `String::len(&text)` is explicit receiver syntax for a method, not a receiver-free Java-style static method.
- An associated function such as `String::new()` has no `self` receiver and is the closer Rust equivalent to a Java static method.
- This distinction should be reinforced once before moving into error handling.

### Roadmap estimate and learning pace
- Overall progress toward intermediate Rust is now estimated at about 40%.
- Ownership/borrowing foundation is about 60% toward the expected intermediate level.
- Independently reusable mastery is about 30-35%, because enums, `Option`, iterators, lifetimes, and receiver forms have often been demonstrated only once.
- During the learner's remaining nine-day focused period, preserve depth for ownership, `Result`/error handling, traits, modules, testing, and program structure. Lower-priority syntax and library details can be explained briefly in chat and revisited when they appear naturally.
- Plan the next big exercise after associated functions versus methods plus a focused introduction to `Result`, `?`, and basic file input.

### Review result
Approved. The learner correctly demonstrated automatic shared borrowing for an `&self` method and its explicit receiver form.

### Next concept
Associated functions versus methods.

## 2026-08-03 - Explicit Mutable Receiver Call Approved

### Demonstrated successfully
- Created a mutable `String` binding named `text`.
- Called `text.push_str(" is")` using normal method-call syntax.
- Printed the string after the first mutation.
- Called `String::push_str(&mut text, " lol.")` using the explicit receiver form.
- Printed the string after the second mutation.
- Correctly commented that the first form passes the receiver borrow implicitly and the second form passes `&mut text` explicitly.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- Method-call syntax can hide receiver borrowing, such as `&self` or `&mut self`.
- The explicit receiver form makes the hidden borrow visible.
- Small spelling/readability issues remain in comments, such as `instence` and `implecitly`, but they do not affect compilation.

### Review result
Approved. The learner correctly connected method-call syntax with the explicit `&mut self` receiver argument.

### Next concept
Method-call auto-borrowing for `&self`.

## 2026-08-03 - Method Auto-Borrowing Approved

### Demonstrated successfully
- Created a mutable `String` binding named `text`.
- Called `text.push_str(" is lol")`.
- Correctly recognized that `push_str` uses a mutable receiver, written as `&mut self` in the method definition.
- Printed `text` after the method call, showing that the temporary mutable borrow ended.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- `push_str` is a method, not a free function.
- The borrow created by method-call syntax is temporary and ends after the method call.
- The user-facing call `text.push_str(...)` hides the explicit receiver argument that Rust conceptually passes as `&mut text`.

### Review result
Approved. The learner correctly demonstrated method-call auto-borrowing for an `&mut self` method.

### Next concept
Explicit method call form: `Type::method(&mut value, ...)`.

## 2026-08-03 - Mutability Belongs To Bindings Approved

### Demonstrated successfully
- Created an immutable `String` binding named `text`.
- Moved the owned `String` value into a new mutable binding named `text_mut`.
- Mutated the value through `text_mut` with `push_str`.
- Printed the mutated value.
- Commented that the old `text` binding is unavailable because ownership moved to `text_mut`.
- Improved the comment wording to say the new binding is mutable.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- `mut` is permission on a binding, not a permanent property of the `String` value.
- A method such as `push_str` temporarily borrows the receiver mutably during the method call.
- Generated-looking files such as `src/main.exe` and `src/main.pdb` appeared in the worktree; they should not be committed as learning source.

### Review result
Approved. The learner correctly separated ownership movement from binding mutability.

### Next concept
Method-call auto-borrowing for `&mut self`.

## 2026-08-03 - Borrow Ends After Last Use Approved

### Demonstrated successfully
- Created a `String`, moved it into a mutable binding, and borrowed it mutably inside a scoped block.
- Mutated through `text_ref` with `push_str`.
- Created `slice` with `&text_ref[0..6]` and printed it.
- Mutated through `text_ref` again after the last use of `slice`.
- Printed the final owner after the block.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- A borrow can end after its last use, not only at the closing brace.
- `mut` is attached to a binding such as `text_1`, not to the `String` value itself.
- Moving ownership transfers the value, but the new variable decides whether that binding is mutable.

### Review result
Approved. The learner correctly demonstrated that the temporary slice borrow ends before a later mutable use through the same reference.

### Next concept
Mutability belongs to bindings, not values.

## 2026-08-03 - Reborrowing Through Mutable Reference Approved

### Demonstrated successfully
- Cleaned the previous large Quest exercise out of `src/main.rs` after the approved concepts were preserved in the learning notes.
- Created a small scoped block around a mutable borrow.
- Bound `text_ref` as `&mut text`.
- Mutated the string through `text_ref` with `push_str`.
- Read a string slice through the same reference using `&text_ref[0..6]`.
- Printed the original `text` after the block, proving the mutable borrow had ended.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- Reading through a mutable reference is allowed when the code is only reading at that moment.
- A scoped block can make borrow lifetimes easier to see.
- String indexes like `0..6` are byte indexes; this example is ASCII-safe, but Unicode-safe slicing still needs a later lesson.

### Review result
Approved. The learner correctly demonstrated mutating through `&mut String`, then reading a slice through that same reference.

### Next concept
Borrow lifetimes inside a block: a borrow ends after its last use.

## 2026-08-03 - Consuming Method Receiver Approved

### Demonstrated successfully
- Added `fn into_reward_xp(self) -> u32` inside `impl Quest`.
- Returned `self.reward_xp` from a method that consumes the whole `Quest`.
- Created one temporary quest near the end of `main`.
- Called `temp_quest.into_reward_xp()` and printed the returned XP.
- Correctly noted that `temp_quest` is moved after the consuming method call.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- A `self` receiver takes ownership of the whole value, like passing the value into a normal function parameter.
- A field can be moved/read out during a consuming method because the method owns `self`.
- Double references such as `&&mut String` are legal but easy to overthink; next lesson should simplify reborrowing and auto-deref.
- Variables beginning with `_`, such as `_d`, intentionally avoid unused-variable warnings.

### Review result
Approved. The learner correctly demonstrated the consuming method receiver form.

### Next concept
Reborrowing and auto-deref with `&mut T`.

## 2026-08-03 - Mutable Method Receiver Approved

### Demonstrated successfully
- Added `fn complete(&mut self)` inside `impl Quest`.
- Made the quest vector mutable so one quest could be changed after creation.
- Used `quests.get_mut(1)` with `if let Some(quest)` to safely borrow one mutable element from the vector.
- Called `quest.complete()` to change that quest's status to `Status::Completed`.
- Correctly moved the mutation before the main report so the updated status appears in later output.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- `get_mut(index)` returns `Option<&mut T>` because the index might not exist.
- A mutable reference can call both `&mut self` methods and `&self` methods. Calling the `&self` method creates a temporary shared reborrow for that call.
- The extra `quest.describe()` inside the `if let` block is useful for experimentation, but it prints the same quest once before the normal report.

### Review result
Approved. The learner correctly used a mutable method receiver to change one quest through a mutable borrow.

### Next concept
Method receiver `self`: consuming a value through a method.

## 2026-08-01 - Methods Returning Borrowed Data Approved

### Demonstrated successfully
- Added `fn tag_label(&self) -> &str` inside `impl Quest`.
- Returned borrowed tag text from `Some(tag)` without cloning or moving the `String`.
- Returned the string literal `"No tag"` for `None`.
- Used `self.tag_label()` inside `describe(&self)` instead of writing the `if let` directly there.
- Kept the Quest Log Analyzer behavior correct.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- A method returning `&str` can return either a slice borrowed from `self` or a string literal, because a string literal has the long-lived type `&'static str`.
- `if let Some(tag) = &self.tag` makes `tag` a borrowed string value, not an owned moved `String`.
- This is a good place to reinforce lifetime elision later: Rust understands the returned borrowed tag is tied to `&self`.

### Review result
Approved. The learner correctly wrote a method that returns borrowed string data without unnecessary cloning.

### Next concept
Method receivers: `self`, `&self`, and `&mut self`.

## 2026-07-31 - Methods With Borrowed Receivers Approved

### Demonstrated successfully
- Moved the free function `describe_quest(quest: &Quest)` into `impl Quest`.
- Wrote the method as `fn describe(&self)`, borrowing the quest instead of consuming it.
- Called the method with `quest.describe()` from both the full quest loop and completed-only loop.
- Kept the program behavior the same after changing from a free function to a method.
- Renamed the enum variant from `Inprogress` to the more idiomatic `InProgress`.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- Method receivers: `self`, `&self`, and `&mut self`.
- Methods can group behavior with the data type, while free functions remain useful when behavior does not naturally belong to one type.
- Returning borrowed string data from methods should be introduced next using `Option<String>` and `&str`.

### Review result
Approved. The learner correctly converted a borrowed free function into an `impl` method.

### Next concept
Methods returning borrowed data.

## 2026-07-31 - Quest Log Analyzer Big Checkpoint Approved

### Demonstrated successfully
- Built a mixed checkpoint program using a `Status` enum, `Quest` struct, `String`, `Option<String>`, `match`, `if let ... else`, vectors, slices, borrowing, and iterator chains.
- Used `#[derive(PartialEq)]` to compare enum variants in filtering logic.
- Wrote `describe_quest(quest: &Quest)` and used borrowed access instead of consuming quests.
- Described every quest with `quests.iter()`.
- Printed both `Some(tag)` values and a fallback for `None`.
- Used a guarded string slice preview after checking `description.len() >= 5`.
- Used helper functions with slice parameters `&[Quest]` instead of `&Vec<Quest>`.
- Used `.iter().filter(...).collect::<Vec<_>>()` to collect completed quests by borrowing.
- Used `.iter().map(...).sum::<u32>()` to calculate completed XP.
- Printed `quests.len()` after filtering and summing, proving the original vector was still usable.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- Enum naming/style polish such as `InProgress` instead of `Inprogress`.
- String slicing is safe in the current ASCII exercise after a length check, but Unicode-safe slicing should be revisited later.
- Functions that only read a vector-like sequence should usually accept `&[T]`.
- `if let ... else` syntax was successfully used but was recently forgotten, so reinforce with more `Option` examples later.

### Review result
Approved. The learner completed the larger mixed exercise and fixed the review issues without editing assistance.

### Next concept
Methods and `impl` with borrowed receivers.

## 2026-07-28 - Iterator `filter` Approved

### Demonstrated successfully
- Created a `Vec<i32>` with `Vec::from([1, 2, 3, 4, 5, 6])`.
- Used `.iter().filter(...)` to keep even numbers while borrowing the original vector.
- Used `.collect::<Vec<_>>()` to collect the filtered borrowed items into a vector.
- Printed `[2, 4, 6]` as the filtered result.
- Printed the original vector afterward, proving borrowed filtering did not consume it.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- In `.iter().filter(|num| ...)`, `num` is a reference to the iterator item; for `Vec<i32>::iter()`, that makes it effectively `&&i32`.
- `&&` is logical AND for booleans; bitwise AND is single `&`.
- The learner requested a short later refresher on Rust operators and bitwise operators, without spending too much time on it.

### Review result
Approved. The exercise correctly demonstrates borrowed filtering and collection.

### Next concept
Mixed checkpoint exercise across earlier Rust fundamentals.

## 2026-07-28 - Iterator `sum` Approved

### Demonstrated successfully
- Created a `Vec<i32>` with `Vec::from([10, 20, 30])`.
- Used `numbers.iter().sum::<i32>()` to calculate a total while borrowing the vector.
- Printed the original vector afterward, proving `.iter().sum()` did not consume it.
- Created a second vector and used `.into_iter().sum::<i32>()` to intentionally consume it.
- Added a correct comment showing that the consumed vector cannot be printed afterward.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- Consumers such as `.sum()` drive an iterator to completion and return a final value.
- Whether the original collection survives depends on how the iterator was created: `.iter()` borrows, `.into_iter()` consumes.
- Variable shadowing with `let numbers = ...` was used correctly, but should be explained again if it becomes confusing.

### Review result
Approved. The exercise correctly demonstrates borrowed and consumed iterator consumers with `.sum()`.

### Next concept
Filtering iterator items with `.filter()`.

## 2026-07-28 - Borrowed Versus Consuming Iterator Inputs Approved

### Demonstrated successfully
- Used `numbers.iter()` to loop over borrowed `i32` values.
- Printed the `numbers` vector after the borrowed loop, showing that `.iter()` does not consume the vector.
- Used `words.into_iter()` to consume a `Vec<String>` and move each owned `String` into the loop variable.
- Added a correct comment explaining that `words` cannot be printed afterward because ownership moved into the iterator.
- Used `strings.iter().zip(lengths.iter())` to pair two borrowed iterators.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- `zip(&lengths)` works because a shared reference to a vector also implements `IntoIterator`, producing borrowed items like `lengths.iter()`.
- `zip` stops when the shorter iterator ends.
- Spelling in variable names and output labels, such as `lenght`, should be cleaned up as a small readability habit.

### Review result
Approved. The exercise correctly shows the difference between borrowed iterator input and consuming iterator input.

### Next concept
Iterator consumers and reducers with `.sum()`.

## 2026-07-28 - Iterator `zip` Approved

### Demonstrated successfully
- Reused a `Vec<String>` and a collected `Vec<usize>` of string lengths.
- Used `strings.iter().zip(lengths)` to pair each borrowed string with one length.
- Printed each `(string, length)` pair without a numeric range and without indexing.
- Demonstrated that `zip` can combine a borrowed iterator with a consumed vector iterator.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- `zip(lengths)` consumes the `lengths` vector because `Vec<T>` implements `IntoIterator`.
- `zip(lengths.iter())` would borrow `lengths` instead and yield `&usize`.
- `zip` stops when the shorter iterator ends, which makes it safer than manually indexing two vectors by the same range.
- The deeper call chain `for_each -> Enumerate::next -> Map::next -> Iter::next` should be revisited when studying the `Iterator` trait more deeply.

### Review result
Approved. The exercise correctly demonstrates pairing iterator items with `.zip()` and avoids parallel-vector indexing.

### Next concept
Borrowing versus consuming iterator inputs.

## 2026-07-28 - Iterator `map` and `collect` Approved

### Demonstrated successfully
- Borrowed a `Vec<String>` with `strings.iter()`.
- Used `.map(|word| word.len())` to lazily transform `&String` items into `usize` lengths.
- Used `.enumerate().for_each(...)` to drive the lazy iterator and print the results.
- Printed the original vector afterward, proving that shared iteration did not consume it.
- Collected a second mapped iterator into `Vec<usize>` with `.collect::<Vec<_>>()`.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- Iterator adapters such as `Map` and `Enumerate` are concrete types that implement the `Iterator` trait.
- Each adapter owns the previous iterator; construction is lazy and traversal begins when a consumer such as `for_each` or `collect` repeatedly calls `next()`.
- Indexing two vectors by the same range works here, but pairing iterator items directly is safer and clearer.

### Review result
Approved. The exercise correctly demonstrates lazy mapping, borrowed iteration, terminal consumption, and collecting mapped items.

### Next concept
Pairing iterator items with `.zip()`.

## 2026-07-28 - Explicit Consuming Iterator Method Approved

### Demonstrated successfully
- Created a `Vec<String>` with three owned strings.
- Used `strings.into_iter()` in a `for` loop.
- Printed each owned `String` yielded by the consuming iterator.
- Added a comment explaining that `strings` cannot be used after the loop because ownership moved into the iterator.
- `cargo check`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- `into_iter()` on `Vec<String>` consumes the vector and moves each `String` out.
- The original vector binding is invalid after the loop; it is not an empty vector that can still be printed.
- A later `println!("{:?}", strings)` would fail because there is no valid owner left to borrow from.

### Review result
Approved. The exercise correctly demonstrates consuming iteration with `.into_iter()`.

### Next concept
Iterator adapter: `.map()`.

## 2026-07-28 - Explicit Borrowed Iterator Methods Approved

### Demonstrated successfully
- Created a mutable `Vec<i32>` using `Vec::from([2_i32, 3_i32, 4_i32])`.
- Used `numbers.iter_mut()` to receive mutable references and update each element with `*num += 1`.
- Used `numbers.iter().enumerate()` to read each element with its index.
- Printed the whole vector afterward as `[3, 4, 5]`, showing borrowed iteration does not consume the vector.
- `cargo check`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and `cargo run` all passed.

### Still practicing
- `iter()` is shared borrowed iteration: each item is like `&T`.
- `iter_mut()` is mutable borrowed iteration: each item is like `&mut T`.
- `Vec::from([..])` can satisfy Clippy when the lesson intentionally wants a `Vec` but the starting values are fixed.

### Review result
Approved. The exercise correctly demonstrates explicit borrowed iterator methods.

### Next concept
Explicit consuming iterator method: `.into_iter()`.

## 2026-07-25 - Wildcard Match Exercise Approved

### Demonstrated successfully
- Created `describe_number(n: u32)` using `match`.
- Handled specific values `0`, `1`, `2`, and `3`.
- Used `_` as a wildcard fallback for all other `u32` values.
- Called the function in a `for i in 0..5_u32` loop.
- Verified that `_` handled the value `4` at runtime.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all completed successfully.

### Still practicing
- `_` means "match anything here, but do not bind the value to a name."
- A named fallback such as `other => ...` can be used when the actual unmatched value is needed.
- Match arm order matters because Rust checks arms from top to bottom.

### Review result
Approved as clean. The exercise correctly demonstrates wildcard patterns and match exhaustiveness.

### Next concept
Binding fallback values with a named variable instead of `_`.

## 2026-07-25 - Option Message Lookup Approved

### Demonstrated successfully
- Created `find_message(id: u32) -> Option<Message>`.
- Used `match id` to return different message variants for specific ids.
- Used `_ => None` as a fallback arm for all other ids.
- In `main`, looped over `0..5_u32` and matched the returned `Option<Message>`.
- Exercised both `Some(m)` and `None` branches at runtime.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all completed successfully.

### Still practicing
- Understanding that `_` is a wildcard pattern meaning "match anything here, but do not bind it to a variable."
- Recognizing why `match` must be exhaustive.
- Using fallback match arms intentionally instead of as mystery syntax.

### Review result
Approved as clean. The exercise correctly demonstrates `Option<T>` as Rust's safe replacement for nullable results.

### Next concept
Wildcard patterns with `_` and match exhaustiveness.

## 2026-07-25 - Option Message Lookup Review

### Demonstrated successfully
- Created `find_message(id: u32) -> Option<Message>`.
- Returned different `Some(Message::...)` variants for several ids.
- Returned `None` for ids outside the accepted range.
- Used `match` on the result of `find_message`.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all completed successfully.

### Still practicing
- The caller should exercise both branches of `Option`: one `Some(...)` case and one `None` case.
- `Some(...)` and `None` are normally written directly instead of `Option::Some(...)` and `Option::None`, though both forms are valid.
- A `match id { ... }` expression can be clearer than a long `if/else if` chain when matching exact values.

### Review result
Close, but not fully approved yet. The function is correct for this lesson, but `main` currently calls only one id that returns `Some(...)`, so the `None` branch is not practiced.

### Next concept
Calling and matching multiple `Option<T>` results, including both `Some` and `None`.

## 2026-07-25 - Resize Variant Borrowed Match Review

### Demonstrated successfully
- Added a tuple-like enum variant `Resize(u32, u32)` to `Message`.
- Updated `handle(message: &Message)` so the `match` remains exhaustive.
- Matched `Message::Resize(height, width)` while borrowing through `&Message`.
- Created one owned `Message::Resize(800, 600)` value and called `handle(&resize)` twice, proving the same enum value can be inspected repeatedly through shared borrowing.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all completed successfully with no warnings.

### Still practicing
- Match ergonomics: when matching a `&Message`, Rust lets patterns like `Message::Resize(height, width)` work without writing explicit reference patterns.
- The bindings from a borrowed match are borrowed values, not owned data moved out of the enum.
- Tuple-like enum variant field order matters; names like `height` and `width` are chosen by the match pattern, not stored field names.

### Review result
Approved as clean. The exercise correctly demonstrates adding a borrowed tuple-like enum variant and reusing the same enum value through shared references.

### Next concept
`Option<T>` as Rust's safe replacement for nullable values.

## 2026-07-25 - Borrowed Enum Match Exercise

### Demonstrated successfully
- Changed `handle(message: Message)` to `handle(message: &Message)`, so the function borrows the enum instead of taking ownership.
- Updated calls to pass references such as `handle(&msg)`.
- Called `handle(&msg)` twice on the same `Message::Text(String)` value, proving that borrowing lets the caller keep ownership.
- Kept the `match` exhaustive for `Quit`, `Text(String)`, and `Move { x, y }`.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all completed successfully with no warnings.

### Still practicing
- When matching on `&Message`, Rust's match ergonomics let patterns like `Message::Text(text)` work without manually writing `&Message::Text(...)`.
- In that borrowed match, `text` is borrowed from inside the enum; it is not moved out of `msg`.
- This concept is correct but should be revisited later because it is one of the bridges between pattern matching, borrowing, and lifetimes.

### Review result
Approved as clean. The exercise correctly demonstrates borrowing an enum so the same value can be inspected more than once.

### Next concept
Match ergonomics with borrowed enum data.

## 2026-07-25 - Enum Move Exercise Formatting Cleanup

### Demonstrated successfully
- Cleaned the previous enum move exercise enough for `cargo fmt --check` to pass.
- Kept the important ownership example: `handle(msg)` moves the `Message` enum into the function.
- Correctly kept the second `handle(msg)` commented out because using `msg` again after the move would be a compiler error.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all completed successfully with no warnings.

### Still practicing
- A function parameter like `handle(message: Message)` takes ownership of the enum value.
- If the enum's active variant contains owned data, such as `Text(String)`, moving the enum also moves that inner owned value.
- To inspect or print an enum without consuming it, the next step is to borrow the enum with `&Message` and match by reference.

### Review result
Approved as clean. The exercise now demonstrates enum ownership movement and passes formatting, compiler, Clippy, and run checks.

### Next concept
Borrowing enum values in `match` instead of moving them.

## 2026-07-25 - Enum Text Variant Move Review

### Demonstrated successfully
- Created a `Message::Text("Hello".to_string())` value and stored it in `msg`.
- Passed `msg` into `handle(msg)`, correctly moving the enum value into the function parameter.
- Commented out a second attempted use of `msg` and correctly explained that it would fail because `msg` was moved.
- Correctly connected that when the active enum variant is `Text(String)`, moving the enum also moves the inner owned `String`.
- `cargo check`, `cargo clippy`, and `cargo run` completed successfully.

### Still practicing
- `cargo fmt --check` failed only because of whitespace/trailing spaces in `src/main.rs`.
- A Rust enum value is not quantum-random: it is already one definite variant. The useful part of the analogy is that when matched, you handle the enum according to the active variant it currently has.
- `match` does not change which variant the enum is; it checks/destructures the variant and gives access to any data inside that variant.

### Review result
Conceptually approved, but not fully clean yet because formatting needs one small cleanup.

### Next concept
Borrowing enum values in `match` instead of moving them.

## 2026-07-25 - Message Enum Variants With Data Review

### Demonstrated successfully
- Created a `Message` enum with three variant shapes: `Quit`, `Text(String)`, and `Move { x: f32, y: f32 }`.
- Constructed all three variants in `main`.
- Wrote a `handle(message: Message)` function using exhaustive `match`.
- Correctly matched `Message::Text(text)` and used the moved `String` inside the match arm.
- Correctly matched the struct-like variant `Message::Move { x, y }` and destructured its named fields.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all completed successfully with no warnings.

### Still practicing
- `Message::Text("Lol".to_string())` creates a new `Message` value each time; `Text` acts like a variant constructor that stores a `String` inside the enum value.
- `Move { x: f32, y: f32 }` is a struct-like enum variant. It is not a separate named struct type, but it stores named fields inside that variant.
- A Rust enum value is exactly one variant at a time. Each variant can store no data, tuple-like unnamed data, or struct-like named data.
- Since `handle(message: Message)` takes ownership, passing `Message::Text(String)` moves the enum and its inner `String` into the function.

### Review result
Approved as clean. The exercise correctly demonstrates enum variants that carry data and matching/destructuring those variants.

### Next concept
Ownership when matching enum variants that contain `String`.

## 2026-07-25 - TrafficLight Enum Warning Cleanup

### Demonstrated successfully
- Renamed the enum variant from `Yello` to `Yellow`.
- Constructed and used all three variants: `Red`, `Yellow`, and `Green`.
- Kept the `match` exhaustive by handling every `TrafficLight` variant.
- Removed the previous dead-code warning for unused enum variants.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all completed successfully with no warnings.

### Still practicing
- Add short comments for key self-answers when asked, especially for concept checkpoints such as "Rust enum variants are values of the enum type, not plain integers."
- Output text can be polished later, for example `Slow down` instead of `Slowdown`.
- Rust enum variants without data are unit-like variants. Later, practice variants that carry data.

### Review result
Approved as clean. The first enum and exhaustive `match` exercise is now warning-free.

### Next concept
Enum variants that carry data.

## 2026-07-25 - TrafficLight Enum First Review

### Demonstrated successfully
- Created a `TrafficLight` enum with unit-like variants.
- Wrote an `action(light: TrafficLight)` function using `match`.
- Handled all enum variants in the `match`, so the match expression is exhaustive.
- Called `action(TrafficLight::Red)` and got the expected output.
- Correctly reasoned that `match` should handle every variant so a valid enum value cannot be ignored or lead to missing behavior.
- `cargo fmt --check` completed successfully.
- `cargo check`, `cargo clippy`, and `cargo run` completed successfully but reported a warning because some variants were intentionally not constructed.

### Still practicing
- The warning for `Yello` and `Green` means those variants are never used/constructed in the current program. It is not solved by assigning numbers such as `Red = 1`.
- Rust enums are not just integers like C/C++ enums and not Java enum singleton objects/classes. A Rust enum is a tagged union / sum type: a value is exactly one variant at a time.
- Fieldless/unit-like variants such as `TrafficLight::Red` are simple named variants. Rust can represent them compactly, often like a small integer internally, but the type system treats them as `TrafficLight`, not plain integers.
- Rust enum variants can also carry data, for example `Message::Text(String)` or `Shape::Circle { radius: f64 }`.

### Review result
Functionally correct, but not fully approved yet because the current code has a dead-code warning from commented-out variant calls.

### Next concept
Continue enums and exhaustive `match`.

## 2026-07-25 - Counter Method Receiver Forms Review

### Demonstrated successfully
- Created a `Counter` struct with an owned `i32` field.
- Added a `new` associated function returning `Self`.
- Added `show(&self)`, which reads the counter without taking ownership.
- Added `increment(&mut self)`, which mutates the counter through a temporary exclusive mutable borrow.
- Added `consume(self)`, which takes ownership of the counter.
- Created a mutable `Counter`, called `show`, then `increment`, then `show` again, then `consume`.
- Correctly commented that `counter` cannot be used after `consume()` because the value moved into the method.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all completed successfully with no warnings.

### Still practicing
- `consume(self)` does not manually "delete" the value in the C++ sense. It takes ownership; then the value is dropped automatically at the end of the method if not returned or moved elsewhere.
- `print!` does not add a newline, so output can run together. Use `println!` when each message should be on its own line.
- For methods, choose the receiver by intent: `&self` to read, `&mut self` to mutate while keeping ownership with the caller, and `self` to consume/take ownership.

### Review result
Approved as clean. The exercise correctly demonstrates all three common method receiver forms.

### Next concept
Enums and pattern matching with `match`.

## 2026-07-25 - Preview Method Lifetime Fix And Receiver Explanation

### Demonstrated successfully
- Fixed `OwnedBook::preview` by changing the return type from `BorrowedBook` to `BorrowedBook<'_>`.
- Kept the method borrowing from `&self.title`, so the returned `BorrowedBook` view is tied to the borrow of `self`.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all completed successfully with no warnings.
- Correctly explained that a method receiver written as `self` takes ownership of the instance.
- Correctly explained that `&self` borrows the instance without taking ownership.
- Correctly explained that `&mut self` lets the method change the instance through a mutable borrow while ownership remains with the caller after the borrow ends.

### Still practicing
- If a method takes `self`, the value is moved into the method. It is dropped at the end of the method only if the method does not return it or move it somewhere else.
- If a method takes `&mut self`, the method gets temporary exclusive access. The caller cannot use the owner during that borrow, but can use it again after the method call ends.
- The phrase "owner is safe" is good intuition, but the precise Rust wording is: ownership does not move; only a borrow is created temporarily.

### Review result
Approved as clean. The method-based borrowed view is now warning-free, and the receiver explanation is correct with minor precision improvements.

### Next concept
Method receiver forms: `self`, `&self`, and `&mut self`.

## 2026-07-25 - OwnedBook Preview Method Review

### Demonstrated successfully
- Added an `OwnedBook::preview(&self)` method that creates a `BorrowedBook` view from `self.title`.
- Correctly used `BorrowedBook::new(&self.title)` inside the method, so the borrowed view points into data owned by `self`.
- Created an `OwnedBook`, called `owned.preview()`, and printed the borrowed title.
- Correctly connected `&self` with the idea that a method can borrow the current instance, similar in spirit to Java's `this`, but checked by Rust's borrow rules.
- `cargo fmt --check` completed successfully.
- `cargo check`, `cargo clippy`, and `cargo run` completed successfully but reported one lifetime-syntax warning.

### Still practicing
- The return type `BorrowedBook` hides the lifetime parameter even though the returned value borrows from `&self`.
- Rust warns that `fn preview(&self) -> BorrowedBook` is confusing because the lifetime is elided on `&self` and hidden on `BorrowedBook`.
- Prefer writing `fn preview(&self) -> BorrowedBook<'_>` here. The `'_` says: "this returned `BorrowedBook` borrows for some lifetime inferred from the input borrow."
- The Java `this` analogy helps for "current instance", but Rust's `self` can be owned, immutably borrowed, or mutably borrowed depending on whether the method takes `self`, `&self`, or `&mut self`.

### Review result
Functionally correct, but not fully approved yet because the code has one warning. The next step is to make the returned borrowed type explicit with `BorrowedBook<'_>`.

### Next concept
Method receiver forms: `self`, `&self`, and `&mut self`.

## 2026-07-25 - OwnedBook vs BorrowedBook Review

### Demonstrated successfully
- Created `OwnedBook` with an owned `String` field.
- Created `BorrowedBook<'a>` with a borrowed `&'a str` field.
- Implemented beginner-friendly `new` associated functions for both structs.
- Moved `sample_title` into `OwnedBook::new(sample_title)`, correctly transferring ownership into `owned.title`.
- Created `BorrowedBook` by borrowing from `&owned.title`, showing that a borrowed struct can borrow from another owned struct's field.
- Printed the borrowed title directly.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all completed successfully with no warnings.

### Still practicing
- After `let owned = OwnedBook::new(sample_title);`, the original `sample_title` binding is no longer usable because its `String` moved into `owned.title`.
- `BorrowedBook::new(&owned.title)` borrows text owned by the `OwnedBook`, not by the old moved `sample_title` binding.
- When comparing progress, separate Rust surface syntax breadth from Rust memory/ownership depth. The learner is further along in ownership/borrowing depth than in total intermediate Rust breadth.

### Review result
Approved as clean. This exercise correctly demonstrates owned struct fields versus borrowed struct fields, including borrowing from an owned field after a move.

### Progress estimate clarification
- Overall intermediate Rust breadth: roughly 25-30%.
- Ownership/borrowing/string memory model depth: roughly 45-55% of the way toward the level expected of an intermediate learner.
- The next large growth area is not just learning Java-like constructs such as loops, enums, traits, and collections, but learning how Rust's ownership, borrowing, pattern matching, error handling, and iterators change how those constructs are designed.

### Next concept
Owned structs that create borrowed views with methods.

## 2026-07-25 - Borrowed Struct Helper Block Review

### Demonstrated successfully
- Used a helper block around `BookPreview::new(&book_title, &book_author)` so the borrowed `preview` definitely goes out of scope before `book_title` is moved.
- Printed both borrowed fields while the owners were still alive.
- Moved `book_title` after the helper block and printed the moved owner successfully.
- Correctly identified that helper blocks help prevent accidentally using a borrowed preview after moving or mutating the owner.
- `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all completed successfully with no warnings.

### Still practicing
- The helper block is a lexical scope: `preview` is dropped at the closing brace, so its borrowed fields cannot be used afterward.
- For this helper-block example, saying the borrow ends at the block boundary is correct. In the previous non-helper-block example, Rust used non-lexical lifetime analysis to end the borrow after the last use.
- A local variable such as `preview` directly owns its `BookPreview` value. Since `BookPreview` only contains references, it owns the reference values, not the `String` data they point into.
- For `String`, the local owner value stores pointer/length/capacity directly, while the UTF-8 bytes live in a heap buffer owned by that `String`.

### Review result
Approved as clean. The helper block correctly makes the borrowed preview's lifetime obvious before moving the owner.

### Progress estimate
Roughly 25-30% of the way toward intermediate Rust. The learner is moving out of raw beginner territory in ownership, borrowing, `String`/`&str`, UTF-8, `Box<T>`, and simple lifetime annotations, but still needs traits, enums/pattern matching, `Result`/`Option`, modules, collections/iterators, generics, error handling, testing, and larger program structure.

### Next concept
Structs with owned fields vs borrowed fields.

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
- Requested the first larger recap exercise, so `docs/exercises/TEXT_NOTE_RECAP.md` was created.

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

## 2026-07-28 - Mutable Borrowed `for` Iteration Approved

### Demonstrated successfully
- Created a mutable `Vec<i32>`.
- Used `for number in &mut numbers` to borrow its elements mutably.
- Used `*number *= 10` to modify each underlying `i32` through an `&mut i32`.
- Printed the whole vector afterward as `[20, 30, 40]`, proving the vector remained owned and usable.

### Still practicing
- State the types precisely: the vector is mutably borrowed for the loop, and each `number` is an `&mut i32`.
- Reinforce how `*number` dereferences the mutable reference to reach the element.
- A standalone `println!()` before every item creates extra blank lines; this is only an output-style observation.

### Review result
Approved. `cargo check`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
Explicit borrowed iterator methods: `.iter()` and `.iter_mut()`.

## 2026-07-27 - Consuming `for` Iteration Approved

### Demonstrated successfully
- Used `for number in numbers` to consume a `Vec<i32>` and print each element by value.
- Left `numbers.len()` commented out after the consuming loop because the `numbers` binding is no longer usable.
- Compared consuming iteration directly with an earlier `for number in &numbers` borrowed loop.

### Still practicing
- The whole `Vec` is moved into the iterator; Rust does not leave an accessible empty vector in `numbers`.
- The consuming iterator owns the vector's allocation while it yields elements and releases the allocation when it is dropped.
- Output spelling: `Lenght` should be `Length`.

### Review result
Approved. `cargo check`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and `cargo run` all passed. The code is correct; the ownership explanation in the comments needed clarification.

### Next concept
Mutable borrowed iteration with `for number in &mut numbers`.

## 2026-07-27 - Borrowed `for` Iteration Approved

### Demonstrated successfully
- Created a `Vec<i32>` with three numbers.
- Used `for number in &numbers` to iterate over borrowed elements.
- Printed each number in insertion order.
- Printed `numbers.len()` after the loop, proving the vector was not consumed and still had length `3`.

### Still practicing
- Distinguishing borrowing iteration from consuming iteration.
- Output spelling: `Lenght` should be `Length`.
- Next contrast: `for number in numbers` consumes the vector, so `numbers.len()` cannot be used afterward.

### Review result
Approved. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
Consuming iteration with `for number in numbers`.

## 2026-07-27 - `while let` With `Vec::pop()` Approved

### Demonstrated successfully
- Created a mutable `Vec<i32>`.
- Used `while let Some(number) = numbers.pop()` to loop over values returned from `pop()`.
- Correctly used the `Some(...)` case to bind each popped number.
- Let the loop stop naturally when `pop()` returned `None`.

### Still practicing
- Remembering that `Vec::pop()` removes from the end, so the printed order is reverse insertion order.
- Output spelling: `Nmuber` should be `Number`.
- Next contrast: `while let ... pop()` consumes/drains the vector, while `for number in &numbers` borrows and leaves the vector usable.

### Review result
Approved. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
Iterating with `for` without consuming a vector.

## 2026-07-27 - `if let ... else` With `Option<String>` Approved

### Demonstrated successfully
- Created a function taking `Option<String>`.
- Used `if let Some(user_name) = name { ... } else { ... }`.
- Called the function with both `Some(...)` and `None`.
- Correctly handled the present-value path and the missing-value fallback.

### Still practicing
- Naming/text precision: `print_username_or_gust` should be `print_username_or_guest`.
- Output spelling: `borowed` should be `borrowed`.
- Rust concept is correct; these are readability/style notes, not compiler errors.

### Review result
Approved. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
`while let` loops.

## 2026-07-27 - `Option<String>` Owned Versus Borrowed Approved

### Demonstrated successfully
- Created `print_username_owned(name: Option<String>)`.
- Created `print_username_borrowed(name: &Option<String>)`.
- Moved an owned `Some("Chat Gpt".to_string())` into the owned function.
- Reused the borrowed value twice with `print_username_borrowed(borrowed_username)`.
- Correctly noticed in the previous attempt that a moved `username` cannot be borrowed afterward.

### Still practicing
- The submitted borrowed value used `let borrowed_username = &Some("Gemini".to_string())`.
- This compiles because Rust extends the temporary lifetime for the reference binding, but the clearer learning pattern is:
  `let borrowed_username = Some("Gemini".to_string()); print_username_borrowed(&borrowed_username);`
- Small output spelling issue: `borowed` should be `borrowed`.

### Review result
Approved. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
`if let ... else`.

## 2026-07-27 - `if let` With Borrowed `Option<String>` Approved

### Demonstrated successfully
- Created `print_username(name: &Option<String>)`.
- Used `if let Some(user_name) = name` to handle only the `Some(...)` case.
- Called the function with a `Some("Chat Gpt".to_string())` value and with `None`.
- Verified that only the `Some(...)` call prints output.

### Still practicing
- The exercise requested `Option<String>` by value, but the submitted solution used `&Option<String>`.
- This is valid Rust and a useful borrowed version, but the ownership difference needs one focused comparison.
- `Option::Some(...)` and `Option::None` are correct; the shorter forms `Some(...)` and `None` are more common when the type is clear.

### Review result
Approved as a valid borrowed `Option<T>` solution. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
`Option<String>` by value versus `&Option<String>`.

## 2026-07-27 - `if let` Enum Variant Approved

### Demonstrated successfully
- Created `print_write_message(message: &Message)`.
- Used `if let Message::Write(text) = message` to handle only one enum variant.
- Called the function with `Write` and non-`Write` variants, proving the non-matching variants are ignored.
- Kept all enum variants and `handle_message` used, removing the previous dead-code warnings.

### Still practicing
- Choosing between `match` and `if let`: use `match` when every variant needs meaningful handling, and `if let` when only one pattern matters.
- Reinforce `if let` with `Option<T>`, because `Some(...)` / `None` is the most common beginner-friendly use case.

### Review result
Approved. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
`if let` with `Option<T>`.

## 2026-07-27 - Struct-Like Resize Variant Reinforcement Approved

### Demonstrated successfully
- Added the struct-like enum variant `Resize { width: u32, height: u32 }`.
- Matched it with `Message::Resize { width, height }`.
- Constructed a `Resize` value using named fields in an order different from the declaration, showing that field names matter more than order.
- Borrowed the `Resize` value into `handle_message(&dimension)`.

### Still practicing
- This was useful reinforcement, but not a brand-new concept: earlier notes already include struct-like variants and borrowed enum matching.
- Move forward to a new pattern-matching tool rather than repeating enum variant shapes again.

### Review result
Approved. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
`if let` for matching one enum variant.

## 2026-07-27 - Borrowed Enum With Data Approved

### Demonstrated successfully
- Changed `handle_message` from taking `Message` by value to taking `&Message`.
- Called `handle_message(&message)` twice with the same `Message::Write(String)` value.
- Matched a borrowed enum using `match message` and handled all variants.
- Confirmed that borrowing lets the caller keep ownership of the enum after the function call.

### Still practicing
- Understanding match ergonomics: when matching `&Message`, patterns like `Message::Write(message)` still work because Rust borrows inner data instead of moving the `String`.
- Noticing the difference between borrowing named enum values and borrowing temporary enum values such as `&Message::Quit`.

### Review result
Approved. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
Struct-like enum variants.

## 2026-07-27 - Enum Variants With Data Approved

### Demonstrated successfully
- Created `Message` with unit-like, tuple-like, and owned-data variants: `Quit`, `Move(i32, i32)`, and `Write(String)`.
- Used exhaustive `match` over all enum variants.
- Destructured `Move(x, y)` to read both stored coordinates.
- Destructured `Write(message)` to read the stored `String`.
- Called `handle_message` once for each variant.

### Still practicing
- Remembering that `handle_message(message: Message)` takes ownership of the whole enum value.
- If the active variant owns data, such as `Write(String)`, matching by value can move that owned data into the match arm.
- Borrowing enum values next will show how to inspect the same enum more than once.

### Review result
Approved. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
Borrowing enum values with attached data.

## 2026-07-27 - Simple Unit Enum Approved

### Demonstrated successfully
- Created a simple enum with unit variants: `Red`, `Yellow`, and `Green`.
- Wrote `describe_light(light: TraficLight)` and matched all variants exhaustively.
- Called the function once for each enum variant.
- Verified the program output for all three cases.

### Still practicing
- Naming precision: `TraficLight` compiles, but `TrafficLight` is the intended spelling and better for readability.
- Reinforcing that enum variants are accessed with `EnumName::Variant`.

### Review result
Approved. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
Enums with data attached to variants.

## 2026-07-27 - Pattern Binding With `@` Approved

### Demonstrated successfully
- Used `small @ 1..=3` to both match a range and bind the matched value.
- Kept an exact `0` arm before broader arms.
- Used a match guard `n if n % 2 == 0` after the `@` range.
- Used a named fallback arm for remaining odd values.
- Proved through `0..7_u32` that `1`, `2`, and `3` hit the `@` arm, `4` and `6` hit the even guard, and `5` hits the fallback.

### Still practicing
- Understanding the meaning of `@`: it binds a name to a value while also testing that value against a pattern.
- Reinforcing match arm order when exact patterns, range bindings, guards, and fallback arms appear together.

### Review result
Approved. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
Enum basics with simple unit variants.

## 2026-07-27 - Match Guard Approved

### Demonstrated successfully
- Added a separate exact `0` arm before the broader guarded pattern.
- Used `n if n % 2 == 0` to match non-zero even numbers after `0` was handled.
- Printed the captured value in both non-zero arms.
- Proved through `0..7_u32` that `0` hits its exact arm, `2`, `4`, and `6` hit the guard, and odd values hit the fallback.

### Still practicing
- Reinforcing arm order: earlier match arms get the first chance.
- Combining range patterns, guards, and captured values cleanly.

### Review result
Approved. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
Binding inside a pattern with `@`.

## 2026-07-27 - Match Guard Attempt Review

### Demonstrated successfully
- Used a match guard with `n if n % 2 == 0`.
- Captured the guarded value and printed it.
- Used a named fallback arm for odd numbers.

### Still practicing
- Handling exact special cases before broader guarded patterns.
- Remembering that `0 % 2 == 0`, so a general even-number guard also matches `0`.

### Review result
`cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed. The Rust syntax is clean, but the exercise is not exact yet because the requested separate `0` arm is missing.

### Next concept
Finish match guards with a separate exact `0` arm.

## 2026-07-27 - Match Alternative Pattern Approved

### Demonstrated successfully
- Used `1 | 2` as an alternative pattern inside `match`.
- Kept a range arm with `3..=5`.
- Kept a named fallback arm `other` and printed its captured value.
- Expanded the loop to `0..7_u32`, proving `1` and `2` share an arm, `3`, `4`, and `5` share the range arm, and `6` reaches the fallback.

### Still practicing
- Reinforcing how exact values, alternatives, ranges, and fallbacks work together.
- Learning the difference between pattern matching by shape/value and checking extra conditions.

### Review result
Approved. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
Match guards with `if`.

## 2026-07-26 - Match Range Pattern Approved

### Demonstrated successfully
- Used `2..=4` as an inclusive range pattern inside `match`.
- Kept exact arms for `0` and `1`.
- Kept a named fallback arm `other` and printed its captured value.
- Expanded the loop to `0..6_u32`, proving that `2`, `3`, and `4` hit the range arm while `5` hit the fallback.

### Still practicing
- Reinforcing pattern choice: exact value, range, named fallback, and later alternatives.
- Keeping match arms ordered from more specific to broader patterns.

### Review result
Approved. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
Match multiple alternatives with `|`.

## 2026-07-26 - Named Fallback Binding Approved

### Demonstrated successfully
- Revised `describe_number(n: u32)` to match the requested shape: `0`, `1`, and `other`.
- Used the named fallback arm `other` to capture and print every non-special value.
- Proved through `for i in 0..5_u32` that `2`, `3`, and `4` all go through the fallback arm.

### Still practicing
- Reinforcing when to use `_` versus a named fallback binding.
- Building confidence with broader match patterns.

### Review result
Approved. `cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed.

### Next concept
Match range patterns with `..=`.

## 2026-07-25 - Named Fallback Binding Attempt Review

### Demonstrated successfully
- Used a final `match` arm named `other` instead of `_`.
- Printed the actual unmatched value from the fallback arm.
- Called `describe_number(i)` from a `for i in 0..5_u32` loop.

### Still practicing
- Following the exact match-shape requested by the exercise.
- Letting a named fallback arm handle all non-special cases, not only the final remaining value after extra exact arms.

### Review result
`cargo check`, `cargo clippy`, `cargo fmt --check`, and `cargo run` all passed. The Rust concept is demonstrated, but the exercise is not exact yet because the code still handles `2` and `3` explicitly. The requested shape was `0`, `1`, and `other`.

### Next concept
Finish the named fallback binding exercise exactly as requested.

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
