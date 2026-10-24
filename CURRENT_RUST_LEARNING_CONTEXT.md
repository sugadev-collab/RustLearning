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

## Latest Approved Exercise - 2026-10-24

- The learner independently practiced a `Quest` constructor, `Status` enum, getter and sibling-module function from `main.rs`, and explained `mod`, `pub`, `use`, `crate::`, `super::`, method-call dot syntax and module/type paths.
- The learner corrected the original `list() -> Option<Vec<Quest>>` implicit-return bug. Latest learner source commit checked: `7e2a6da` ("list fn defect fixed", 2026-10-24).
- Explained final-expression/semicolon semantics and `filter_map`'s `Some` extraction. Demonstrated conceptual understanding; do not interpret this as full iterator mastery or full project approval.
- The code was reviewed from GitHub; no local `cargo check`, tests, or runtime verification was performed by the tutor for this checkpoint.

## Active Weak Spots

- Current `list()` prints validation/parse errors then drops malformed records through `filter_map`; if `start`/`complete` later writes the reduced vector, it can erase stored records. User prefers reporting bad entries and continuing when safe; agree on a safe write policy before changing project behavior. Version 1 requirements currently mandate line-aware errors and no destructive invalid writes.
- `Result` vs `Option`, converting via `.ok()`, and collecting `Result<Vec<_>, E>` need deliberate practice.
- Other project bugs remain outside the approved return-value fix (absolute read vs relative write paths; argument iteration; add command replacing the file).

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

Preserving and propagating `Result` errors while processing collections, especially `map` and `collect::<Result<Vec<_>, _>>()`; compare this with `filter_map` and deliberate log-and-skip read-only policies.

## Next Small Exercise

In a small isolated practice function (not a Quest Tracker rewrite), collect `Vec<Result<i32, &str>>` using `.into_iter().collect::<Result<Vec<i32>, &str>>()`; compare inputs with all `Ok` versus one `Err`. Explain why a mutating command needs a reliable loading result.

## Roadmap Notes

- 2026-10-24 reinforcement checkpoint: keep the historical August estimate (~55% curriculum coverage, ~35-40% durable mastery) as historical, not a new verified assessment. No percentage advancement has been assigned.
- Axum goal: after stronger modules/borrowing/error flow, begin small Axum handlers and Tokio async basics in parallel with fundamentals; traits/generics, tests, shared state, and data persistence follow progressively.

- As of 2026-08-08, estimated curriculum coverage toward intermediate Rust is about 55%; independently reusable mastery remains about 35-40% because newer concepts still need repetition.
- The 55% prerequisite gate is reached: file I/O, error propagation, command-line input, a first module, and a first unit test are approved.
- The learner is ready to plan and begin a deliberately small Quest Tracker as a learning project. Modules and tests should be reinforced inside it rather than delayed until they are mastered separately.
- The 2026-08-09 planning checkpoint is complete. The authoritative Version 1 scope and tutor/AI handoff are in `docs/projects/QUEST_TRACKER_V1_REQUIREMENTS.md`.
- Stable roadmap: `docs/reference/RUST_LEARNING_ROADMAP.md`.
- Current completed big-exercise specification: `docs/projects/QUEST_REVIEW_REQUIREMENTS.md`.
