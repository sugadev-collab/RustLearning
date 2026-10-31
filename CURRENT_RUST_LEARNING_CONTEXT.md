# Current Rust Learning Context

Read this file first at the start of a Rust review or lesson. It is intentionally compact.

## Reading Order

- Use this file for the current state.
- When transferring to another AI tutor, use `docs/preferences/AI_TUTOR_HANDOFF.md`; do not paste the full chat or large history files by default.
- Use `docs/reference/RUST_CHEAT_SHEET.md` only for targeted meanings and examples; do not read the whole file by default.
- Use targeted searches in `docs/learning/LEARNING_PROGRESS.md` or `docs/learning/RUST_DOUBTS_AND_QUESTIONS.md` only when more detail is needed.
- Do not read `docs/learning/archive/RUST_CONTEXT_ARCHIVE_2026_08_07.md` by default; it contains older checkpoint and weak-spot detail.

## Workflow Preferences

- Review the learner's code without editing it unless they explicitly request edits.
- Run `cargo check`, `cargo fmt --check`, `cargo clippy -- -D warnings`, and the program when appropriate.
- Explain at most three important issues, give hints first, and let the learner fix them.
- After an approved exercise, update progress, this compact context, and the cheat sheet when useful new syntax or APIs were introduced.
- Keep this file small; archive older detail under `docs/` when it begins growing again.
- For big exercises, provide the complete requirements file upfront and let the learner make a full independent attempt.
- For the Quest Tracker, treat brief source-file comments and the evolving file structure as the learner's plan; do not require a polished separate planning document.
- The learner prefers reading the complete requirements progressively by milestone, implementing important foundations first, and revising the structure when the code reveals a need.
- Prefer durable understanding over rushing toward a fixed deadline.
- Small Git commits are allowed when useful; never stage generated build artifacts.

## Latest Approved Exercises - 2026-10-24 to 2026-10-31

- 2026-10-24: learner independently practiced `Quest::new`, status getter, module paths, and fixed the `list() -> Option<Vec<Quest>>` implicit-return bug (source commit `7e2a6da`). This approves the specific return fix, not the entire tracker.
- 2026-10-27: learner wrote a standalone practice function collecting `Vec<Result<i32, &str>>` into `Result<Vec<i32>, &str>`, comparing an error case with all-success case, and used `match` to display the results. Exercise approved from pasted code; Cargo execution not independently verified.
- 2026-10-28: learner accurately explained static typing, literal integer defaults and string literals in type inference. Clarified that types, not runtime values, must be known at compile time; `10_i32` explicitly fixes the integer type; string literals have `&'static str`.
- 2026-10-28: learner independently implemented `parse_xp(text: &str) -> Result<u32, std::num::ParseIntError>` using `let number = text.parse::<u32>()?; Ok(number)`. Implementation reviewed and approved from pasted snippet; no local execution verified.
- 2026-10-31: learner predicted correctly that successive `nth(1)`, `nth(1)`, `next()` calls on a five-item iterator yield 20, 40, 50, and explored why repeated `nth` calls advance the same iterator. Conceptual exercise approved; still needs to implement the actual CLI parser fix.
- 2026-10-31: approved the actual Start command parser fix in source commit `a52f1d`: changed `arg.nth(2)` to `arg.next()`, re-enabled normal command dispatch, and removed the temporary iterator exercise. Verified via GitHub diff; Cargo not run in reviewer environment.
- 2026-10-31: approved the `Complete` command's `arg.nth(2)` → `arg.next()` change in source commit `f93ad50`; verified the exact GitHub diff, not the runtime. Both `Start` and `Complete` now retrieve the immediate next arg; actual quest operations still need file/data fixes.
- 2026-10-31: learner supplied a corrected local Add parser snippet, collecting remaining `Args` directly and using zero-based `.get(0)`, `.get(1)`, `.get(2)`. Parsing logic reviewed as correct assuming existing final `Commands::Add` construction; not pushed or compiler-tested at time of review.
- No source-code edits were made by the tutor.

## Active Weak Spots

- Current `list()` prints validation/parse errors then drops malformed records through `filter_map`; if `start`/`complete` later writes the reduced vector, it can erase stored records. User prefers reporting bad entries and continuing when safe; agree on a safe write policy before changing project behavior. Version 1 requirements currently mandate line-aware errors and no destructive invalid writes.
- `Result` vs `Option`, converting via `.ok()`, and collecting `Result<Vec<_>, E>` need deliberate practice.
- Other project bugs remain (absolute read vs relative write paths; `Add` argument parsing was corrected locally but is not yet compiler-verified or committed; `Add` execution still overwrites earlier quests).

- Iterator chains and their item/reference types need reinforcement, especially `map`, `filter`, and `collect`.
- `Option`, `Result`, and `?` are understood at an introductory level but need more independent repetition.
- Borrowed versus consuming iteration needs reinforcement: `.iter()`/`&items` preserve the collection; `.into_iter()`/owned iteration consume it.
- `self`, `&self`, `&mut self`, reborrowing, and lifetime elision should recur in later projects.
- Matching borrowed enum data and ownership during destructuring still need practice.
- Unicode-safe string handling is deferred; earlier byte slicing was only used with ASCII.
- The learner can use the cheat sheet successfully but wants reinforcement of explicit dereferencing, auto-dereferencing, and when not to use `unwrap`.
- Exact file-I/O API recall is still developing; emphasize conceptual choice first, then documentation lookup for API details.
- Modules and `pub` have been demonstrated once and need repetition inside the real project.
- `#[cfg(test)]`, `super`, `#[test]`, and `assert_eq!` were adapted from an external example successfully but are not yet independently understood.

## Current Next Concept

Separate command parsing from command execution, especially when an executor can overwrite persisted data. Validate corrected Add argument parsing without invoking the unsafe Add storage write.

## Next Small Exercise

Run `cargo check` with the locally corrected Add parser and share any errors. To verify argument order safely, construct or print the parsed `Commands::Add` fields without calling `execute_command`'s Add branch or altering existing quests. After that, address the data-preservation defect in Add execution.

## Roadmap Notes

- 2026-10-24 reinforcement checkpoint: keep the historical August estimate (~55% curriculum coverage, ~35-40% durable mastery) as historical, not a new verified assessment. No percentage advancement has been assigned.
- Axum goal: after stronger modules/borrowing/error flow, begin small Axum handlers and Tokio async basics in parallel with fundamentals; traits/generics, tests, shared state, and data persistence follow progressively.

- As of 2026-08-08, estimated curriculum coverage toward intermediate Rust is about 55%; independently reusable mastery remains about 35-40% because newer concepts still need repetition.
- The 55% prerequisite gate is reached: file I/O, error propagation, command-line input, a first module, and a first unit test are approved.
- The learner is ready to plan and begin a deliberately small Quest Tracker as a learning project. Modules and tests should be reinforced inside it rather than delayed until they are mastered separately.
- The 2026-08-09 planning checkpoint is complete. The authoritative Version 1 scope and tutor/AI handoff are in `docs/projects/QUEST_TRACKER_V1_REQUIREMENTS.md`.
- Stable roadmap: `docs/reference/RUST_LEARNING_ROADMAP.md`.
- Current completed big-exercise specification: `docs/projects/QUEST_REVIEW_REQUIREMENTS.md`.
