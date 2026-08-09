# Current Rust Learning Context

Read this file first at the start of a Rust review or lesson. It is intentionally compact.

## Reading Order

- Use this file for the current state.
- Use `docs/RUST_CHEAT_SHEET.md` only for targeted meanings and examples; do not read the whole file by default.
- Use targeted searches in `docs/LEARNING_PROGRESS.md` or `docs/RUST_DOUBTS_AND_QUESTIONS.md` only when more detail is needed.
- Do not read `docs/RUST_CONTEXT_ARCHIVE_2026_08_07.md` by default; it contains older checkpoint and weak-spot detail.

## Workflow Preferences

- Review the learner's code without editing it unless they explicitly request edits.
- Run `cargo check`, `cargo fmt --check`, `cargo clippy -- -D warnings`, and the program when appropriate.
- Explain at most three important issues, give hints first, and let the learner fix them.
- After an approved exercise, update progress, this compact context, and the cheat sheet when useful new syntax or APIs were introduced.
- Keep this file small; archive older detail under `docs/` when it begins growing again.
- For big exercises, provide the complete requirements file upfront and let the learner make a full independent attempt.
- Prefer durable understanding over rushing toward a fixed deadline.
- Small Git commits are allowed when useful; never stage generated build artifacts.

## Latest Approved Exercise - 2026-08-08

- Extracted a private pure formatter used by the public save helper.
- Added a `#[cfg(test)]` child module, imported the parent helper with `super::...`, and verified two-title formatting with `#[test]` plus `assert_eq!`.
- One test, check, formatting, strict Clippy, and the normal program run passed.

## Active Weak Spots

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

Project decomposition: assigning model, storage, and command-line responsibilities to clear modules.

## Next Small Exercise

Read `docs/QUEST_TRACKER_PROJECT_REQUIREMENTS.md` and submit the seven-item structural planning response listed under “What The Learner Must Do Next.” Do not invent additional product requirements or begin implementation before that compact plan is reviewed.

## Roadmap Notes

- As of 2026-08-08, estimated curriculum coverage toward intermediate Rust is about 55%; independently reusable mastery remains about 35-40% because newer concepts still need repetition.
- The 55% prerequisite gate is reached: file I/O, error propagation, command-line input, a first module, and a first unit test are approved.
- The learner is ready to plan and begin a deliberately small Quest Tracker as a learning project. Modules and tests should be reinforced inside it rather than delayed until they are mastered separately.
- The 2026-08-09 planning checkpoint is complete. The authoritative Version 1 scope and tutor/AI handoff are in `docs/QUEST_TRACKER_PROJECT_REQUIREMENTS.md`.
- Stable roadmap: `docs/RUST_LEARNING_ROADMAP.md`.
- Current completed big-exercise specification: `docs/QUEST_REVIEW_PROJECT.md`.
