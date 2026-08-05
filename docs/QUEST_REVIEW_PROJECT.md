# Quest Review Consolidation Exercise

## Purpose

Quest Review is an in-memory Rust program for combining recently learned concepts in one coherent exercise. It is preparation for the later command-line Quest Tracker, but it is not yet the persistent real project.

Read the complete requirements first, then build the whole program independently. The numbered parts are a suggested build order, not separate lessons or approval gates. You do not need to stop after each part. Ask for help only if you become stuck, or submit the complete attempt for review when you are done.

## Allowed Concepts

- Structs and enums
- `String` and `&str`
- `Option<T>`
- `Vec<T>` and slices
- `match` and `if let`
- Iterator methods
- Ownership and borrowing
- Associated functions and methods using `self`, `&self`, or `&mut self`

## Not Required Yet

- `Result<T, E>`
- The `?` operator
- File input/output
- Modules
- External crates
- `unsafe`

## Required Data Model

```rust
enum QuestStatus {
    Planned,
    Active,
    Completed,
}

struct Quest {
    title: String,
    reward_xp: u32,
    status: QuestStatus,
    note: Option<String>,
}
```

## Full Requirements and Suggested Build Order

### Part 1: Type Skeleton

Requirements:

- Define `QuestStatus` with `Planned`, `Active`, and `Completed`.
- Define `Quest` with `title`, `reward_xp`, `status`, and `note`.
- Add an empty `impl Quest` block.
- Keep `main` empty.

Status: completed and approved on 2026-08-04. Compilation, Clippy, formatting, and execution passed. Dead-code warnings are expected because the types are deliberately unused at this point.

### Part 2: Construction

Requirements:

- Add this associated-function shape: `fn new(title: String, reward_xp: u32, note: Option<String>) -> Self`.
- Accept the title, XP reward, and optional note as inputs.
- Give every newly constructed quest the `Planned` status.
- Construct one quest in `main`.

Learning focus: an associated function has no `self` receiver, while `Self` refers to the type currently being implemented.

### Part 3: Reading and Mutation Methods

Requirements:

- Add `fn title(&self) -> &str`, returning a borrowed view of the title without moving it.
- Add `fn complete(&mut self)`, changing the quest to `Completed`.
- Add `fn into_reward_xp(self) -> u32`, consuming one quest and returning its reward XP.
- Demonstrate the consuming method with a separate quest so the main quest vector remains available for the report.

Learning focus: choosing a receiver from what the operation needs to do.

### Part 4: Collection and Optional Data

Requirements:

- Create a `Vec<Quest>` containing at least four quests.
- Ensure the collection uses every `QuestStatus` variant at least once.
- Include at least one `Some(String)` note and one `None` note.
- Use `Vec::get_mut(index)` to obtain one quest safely and complete it.
- Use `if let` to print a note only when one exists.
- Use `match` to print or describe each quest's status.

Learning focus: borrowing elements safely and handling optional data explicitly.

### Part 5: Slice and Iterator Report

Requirements:

- Write at least one function accepting `&[Quest]`, not `&Vec<Quest>`.
- Use an iterator chain containing `filter`, `map`, and `sum`.
- Calculate the total reward XP for completed quests.
- Keep the vector usable after generating the report.
- To recognize completed quests, you may use a borrowed `match`, the `matches!` macro, or derive and use `PartialEq`. Be prepared to explain your choice.

Learning focus: borrowed collection views, iterator item types, and non-consuming analysis.

### Part 6: Final Review

The program is complete when:

- Every required behavior is present.
- Ownership choices can be explained by the learner.
- `cargo check` passes without warnings.
- `cargo clippy -- -D warnings` passes.
- `cargo fmt --check` passes.
- `cargo run --quiet` produces understandable output.

## Tutor Guardrails

- Give the complete requirements file up front and let the learner attempt the whole exercise independently.
- Do not turn the parts into mandatory one-by-one lessons unless the learner asks for that help.
- Review the learner's own code before suggesting changes.
- Separate compiler errors, logical errors, style concerns, and optional improvements.
- Preserve the exercise's learning value; do not introduce advanced abstractions merely to make the program shorter.
- If the learner is stuck, give the smallest useful hint first.

## Relationship to the Real Project

Quest Review stores everything in memory and rehearses the core data model. The later Quest Tracker will add `Result`, `?`, file persistence, command-line interaction, modules, and tests. Its detailed plan should be created or finalized at the 2026-10-09 planning checkpoint after checking the learner's actual readiness.
