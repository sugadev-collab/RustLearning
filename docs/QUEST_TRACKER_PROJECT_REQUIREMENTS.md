# Quest Tracker Project Requirements

## Purpose

Build a small but real command-line Quest Tracker in stable Rust 2024. The program must preserve quests in a text file so they remain available across separate program runs.

This is a learning project, not a test of memorizing APIs. The learner may use `docs/RUST_CHEAT_SHEET.md`, Rust compiler messages, and targeted standard-library documentation. The learner should still write and understand the project code rather than asking another AI to generate the whole implementation.

## Project Readiness

The project begins at the 55% learning-roadmap gate. The learner has already demonstrated the following at least once:

- Ownership, borrowing, mutable borrowing, and slices
- Structs, enums, methods, and method receivers
- `Option`, `Result`, matching, and the `?` operator
- Vectors and introductory iterator chains
- Reading and writing text files
- Command-line arguments
- A separate Rust module with `pub` visibility
- One focused unit test

These concepts are not all mastered. The project must reinforce them through repeated use.

## Scope Rules

- Use stable Rust and the Rust 2024 edition.
- Use only the Rust standard library.
- Do not use `unsafe`.
- Do not use external crates, databases, JSON, a GUI, a web server, or async Rust in Version 1.
- Prefer clear beginner-readable code over compressed or clever code.
- Do not panic for expected user mistakes, missing arguments, invalid quest numbers, malformed XP, or ordinary file errors.
- `unwrap` and `expect` must not be used in user-facing execution paths.
- Tests may use `unwrap` only when the reason is deliberate and the test would otherwise be obscured; matching or assertions are still preferred while learning.

## Version 1 User Experience

The program performs one command per run. It loads existing quests, executes the requested command, prints a clear result, and saves only after a successful mutation.

The fixed Version 1 data path is `data/quests.db`.

Supported commands:

| Command | Required behavior |
|---|---|
| `help` | Print the available commands and expected arguments. |
| `list` | Print every quest with a one-based quest number, status, title, XP, and note fallback. |
| `add <title> <xp> [note]` | Add a new planned quest after validating all supplied data. |
| `start <quest-number>` | Change a planned quest to active. |
| `complete <quest-number>` | Change an active quest to completed. |
| `summary` | Print quest counts by status and the total XP from completed quests. |

Examples of intended shell usage:

- `cargo run -- help`
- `cargo run -- list`
- `cargo run -- add "Learn Rust modules" 100 "Keep responsibilities clear"`
- `cargo run -- add "Practice tests" 80`
- `cargo run -- start 1`
- `cargo run -- complete 1`
- `cargo run -- summary`

Quoted shell arguments allow titles and notes to contain spaces. Supporting an interactive menu is outside Version 1.

## Quest Model Requirements

Create a quest model containing:

- An owned title
- Reward XP as `u32`
- A quest status
- An optional owned note

The status must be an enum with exactly these Version 1 states:

- Planned
- Active
- Completed

Required model behavior:

- Creating a quest always begins with Planned status.
- Starting a quest succeeds only when it is Planned.
- Completing a quest succeeds only when it is Active.
- Invalid transitions return an explanatory error instead of panicking or silently doing nothing.
- Read-only access needed by listing, saving, and reporting must use borrowing.
- Mutation must require a mutable borrow.
- The model should not expose every field publicly merely to avoid writing a clear API.

## Input Validation Requirements

Validate before mutating or saving data:

- A title must not be empty after trimming.
- XP must parse as `u32` and must be greater than zero.
- A quest number must parse as `usize` and must be at least 1.
- A quest number outside the current list must produce a useful error.
- An omitted note becomes `None`.
- An empty or whitespace-only note also becomes `None`.
- Titles and notes must not contain a newline or the `|` character in Version 1.
- An unknown command or incorrect argument count must print a useful message and suggest `help`.

No invalid command may overwrite the data file.

## Storage Format

Store one quest per line using four pipe-separated fields in this order:

1. Status
2. Reward XP
3. Title
4. Note

Use these exact lowercase status values in the file:

- `planned`
- `active`
- `completed`

An absent note is represented by an empty fourth field. Because Version 1 rejects `|` and newlines inside titles and notes, escaping is not required yet.

Illustrative persisted record:

`planned|100|Learn Rust modules|Keep responsibilities clear`

Storage behavior:

- A missing `data/quests.db` file represents an empty tracker rather than a fatal error.
- An existing empty file represents an empty tracker.
- Other file-read failures must be reported.
- Every non-empty stored line must contain exactly four fields.
- Invalid XP, status text, or field count must report the line number and reason.
- Malformed stored records must not be silently skipped.
- Saving rewrites the complete file only after a successful mutating command.
- `list`, `summary`, `help`, and failed commands must not rewrite the file.

Atomic replacement, file locking, escaping arbitrary text, and recovery from partial writes are future improvements, not Version 1 requirements.

## Required Module Responsibilities

Use at least these three source responsibilities:

| Area | Responsibility |
|---|---|
| Crate root / command-line coordination | Read arguments, choose a command, call model or storage operations, and print user-facing results. |
| Quest model module | Own the quest types, status rules, validation closely related to a quest, and state transitions. |
| Storage module | Convert quests to and from the Version 1 file representation and perform file reads/writes. |

The exact filenames may be proposed in the learner's plan, but `main.rs`, a quest/model module, and a storage module are expected.

Do not create many tiny modules merely to look architectural. A separate command parser module is optional and should be introduced only if `main.rs` becomes difficult to understand.

## Command Behavior Details

### `help`

- Must work even when the data file is missing or malformed because help does not need stored data.
- Must show every supported command and its arguments.

### `list`

- Print a friendly message if no quests exist.
- Display one-based quest numbers.
- Display a clear fallback such as `No note` for an absent note.
- Do not consume the quest vector merely to print it.

### `add`

- Validate the argument count, title, XP, note, and unsupported delimiters.
- Add one Planned quest.
- Save only after the quest was added successfully.

### `start`

- Interpret the user's number as one-based.
- Safely obtain a mutable quest without indexing that could panic.
- Apply the model's transition rule.
- Save only after a successful transition.

### `complete`

- Interpret the user's number as one-based.
- Safely obtain a mutable quest.
- Apply the model's transition rule.
- Save only after a successful transition.

### `summary`

- Count Planned, Active, and Completed quests.
- Sum XP only from Completed quests.
- Use borrowed iteration so quests remain available afterward.
- This command is intended to reinforce iterator adapters or a clear borrowed loop; either is acceptable if the learner can explain the ownership behavior.

## Error-Handling Requirements

- Expected failures must be represented and handled with `Option`, `Result`, matching, or `?` as appropriate.
- Error messages must state what failed and, when useful, how to correct it.
- File errors, malformed stored data, invalid numeric input, missing arguments, unknown commands, invalid quest numbers, and invalid status transitions must not cause a panic.
- It is acceptable for early project functions to use `Result<T, String>` for application-level explanations.
- Introducing a custom application error enum is optional and should not be required before the learner understands why it would help.

### New or challenging API research allowed

The following details are expected to require targeted cheat-sheet or standard-library lookup:

- Parsing `String` or `&str` into `u32` and `usize`
- Detecting `std::io::ErrorKind::NotFound`
- Converting a standard-library error into a useful application message
- Choosing safe access with `get` or `get_mut` after converting a one-based number
- Referring to sibling or crate-root modules with module paths such as `crate::...`

Ask for a small hint if blocked. Do not request a complete generated implementation.

## Testing Requirements

Tests should focus first on pure behavior rather than real filesystem side effects.

Minimum required tests:

1. A new quest begins as Planned.
2. Starting a Planned quest succeeds and makes it Active.
3. An invalid start transition returns an error.
4. Completing an Active quest succeeds and makes it Completed.
5. Completing a non-Active quest returns an error.
6. Formatting and then parsing a quest with a note preserves its values.
7. Formatting and then parsing a quest without a note preserves `None`.
8. Parsing a malformed stored line returns an error.

Additional tests are useful only when they verify meaningful rules discovered during implementation.

Every final review must run:

- `cargo test`
- `cargo check`
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- Representative commands for `help`, `list`, `add`, `start`, `complete`, and `summary`

## Implementation Milestones

The complete requirements are provided now. The learner may implement the whole attempt independently and request one final review. These milestones are an implementation order, not mandatory approval gates.

### Milestone 1: Structural plan

- Keep a brief evolving structure in source-file comments or a separate note, whichever feels more natural.
- Identify the responsibility of the crate root, model module, and storage module briefly.
- Adjust filenames or add a module later when implementation reveals a clear need.
- A polished up-front planning document is not required.

### Milestone 2: Quest model

- Implement quest data and status.
- Implement constructor, read access, and valid/invalid transitions.
- Add focused model tests.

### Milestone 3: Storage conversion

- Convert one quest to the required line format.
- Parse one stored line with validation and line-aware errors.
- Add storage conversion tests.

### Milestone 4: File persistence

- Load all quests.
- Treat a missing file as an empty tracker.
- Save the complete collection after successful mutations.

### Milestone 5: Commands

- Implement `help`, `list`, `add`, `start`, `complete`, and `summary`.
- Preserve data on failed commands.

### Milestone 6: Final verification and cleanup

- Run every required check and representative command.
- Remove obsolete experiment code only after the working project is preserved in Git.
- Keep useful learning explanations in the learning records rather than leaving large commented-out programs in `main.rs`.

## Definition of Done

Version 1 is complete only when:

- Every required command behaves as specified.
- Quests persist across separate program runs.
- Expected bad input and ordinary I/O errors do not panic.
- Invalid commands do not damage or overwrite saved data.
- All minimum tests pass.
- Check, formatting, and strict Clippy pass without warnings.
- The learner can explain the main ownership flow, why borrowed iteration does not consume the quest collection, how `Result` errors travel, why the modules are separated, and what `super` means in the test modules.

## Explicitly Deferred Features

Do not add these to Version 1:

- Deleting or editing quest text
- Due dates, priorities, tags, users, or authentication
- Interactive menus
- JSON or third-party serialization
- A database
- GUI or web interfaces
- Async I/O
- Concurrency
- File locking or atomic-save recovery
- External crates

These may become later project iterations when they serve a specific learning goal.

## What The Learner Must Do Next

Use this file as the authoritative Version 1 checklist, but it may be read progressively. Before starting each milestone, read that milestone and the relevant requirement sections, then implement it in the learner's own style.

Begin with the Quest model milestone. Continue through storage, persistence, commands, and final verification independently. Brief source comments and the evolving file layout are sufficient planning; no separate formal plan approval is required.

Return for a complete review after the independent attempt, or ask for a small hint when genuinely blocked. Do not add deferred Version 1 features without first checking whether they complicate the required behavior.

## Tutor And AI Handoff Rules

Any tutor or AI continuing this project must:

- Read `CURRENT_RUST_LEARNING_CONTEXT.md` first and use this requirements file as the authoritative Version 1 scope.
- Inspect learner-written code before suggesting changes.
- Run Rust checks when reviewing.
- Identify at most three important problems at one time.
- Separate compiler errors, logical errors, style issues, and optional improvements.
- Give hints before complete solutions.
- Never replace the learner's full implementation unless explicitly asked.
- Reinforce modules, `super`, tests, `Option`, `Result`, `?`, iterator item/reference types, and borrowing through actual project code.
- Keep the project standard-library-only, stable, safe, and beginner-readable.
- Do not expand Version 1 with deferred features without the learner's explicit request.
- Preserve approved checkpoints in Git before substantial cleanup or restructuring.
