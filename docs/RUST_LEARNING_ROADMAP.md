# Rust Learning Roadmap

This is the stable roadmap for the current Rust learning path. Percentages are rough navigation markers, not exam scores. A stage is not considered mastered after one successful exercise; important ideas will return inside larger programs.

## Current Position

- Date recorded: 2026-08-05
- Overall path toward intermediate Rust: about 40%
- Ownership and borrowing foundation: about 60% of the desired depth
- Independently reusable mastery: about 30-35%
- Current stage: Quest Review consolidation completed; beginning `Result<T, E>` and explicit error handling

## Learning Principles

- Prefer durable understanding over reaching a percentage quickly.
- Practice important concepts in code more than once and in different contexts.
- Explain lower-priority syntax briefly when it appears naturally.
- Use stable Rust 2024, the standard library, and safe Rust while learning the fundamentals.
- Review learner-written code before offering a complete solution.
- For big exercises, provide one complete requirements file, let the learner build independently, and review the completed attempt unless help is requested.

## Stage 1: Language and Memory Foundations (0-25%)

Main topics:

- Variables, mutability, expressions, functions, and control flow
- Ownership, moves, copying, borrowing, and mutable borrowing
- `String`, `&str`, slices, UTF-8 awareness, and basic heap/stack reasoning
- Basic structs and methods
- Introductory lifetime reasoning and non-lexical lifetimes

Status: substantially covered, but ownership rules will continue to be reinforced in every later stage.

## Stage 2: Data Modeling and Rust-Style Control Flow (25-45%)

Main topics:

- Enums with and without attached data
- `match`, pattern destructuring, guards, `if let`, and `while let`
- `Option<T>` instead of nullable values
- `Vec<T>`, slices, borrowed and consuming iteration
- Iterator adapters and consumers such as `map`, `filter`, `zip`, `collect`, and `sum`
- `impl` blocks, associated functions, and `self`, `&self`, and `&mut self` receivers

Status: the Quest Review consolidation exercise is complete. Many topics have now been combined successfully once and should keep recurring in later work.

## Stage 3: Errors and Program Boundaries (45-55%)

Main topics:

- `Result<T, E>` and matching success versus failure
- The `?` operator and error propagation
- Choosing between `Option` and `Result`
- Basic standard-library error types
- Reading and writing text files
- Command-line arguments and input validation
- Separating a program into initial modules

Milestone: after `Result`, `?`, and basic file handling are understood through exercises, begin the first real project.

## First Real Project Marker (about 50-55%)

The planned first real project is a command-line Quest Tracker. It should begin before the learner is fully intermediate because building and improving it is part of reaching that level.

Initial project abilities:

- Add and list quests
- Change quest status
- Attach optional notes and XP rewards
- Load quests from a file
- Save updated quests
- Report errors without panicking
- Organize code into modules
- Add focused automated tests

Do not start it only because a calendar date has arrived. Start when the prerequisite lessons are sufficiently understood, or deliberately reduce the first version if more reinforcement is needed.

## Stage 4: Reusable and Reliable Rust (55-70%)

Main topics:

- Modules, visibility, and crate organization
- Unit and integration tests
- Traits and their relationship to Java interfaces
- Generics, trait bounds, and reusable functions/types
- Derivable traits such as `Debug`, `Clone`, `PartialEq`, and `Default`
- More deliberate error design
- Refactoring the Quest Tracker without losing working behavior

Project work: grow the real project while each abstraction is motivated by an actual need.

## Stage 5: Intermediate Ownership and API Design (70-85%)

Main topics:

- Lifetime elision and explicit lifetime relationships in APIs
- Owned versus borrowed fields and return values
- Closures and deeper iterator reasoning
- `Box<T>`, `Rc<T>`, `RefCell<T>`, and when each is appropriate
- Dynamic dispatch with trait objects versus static dispatch with generics
- Designing APIs that make invalid states difficult to represent

Project work: improve internal design and add features that require stronger ownership decisions.

## Stage 6: Intermediate Consolidation and Ecosystem Use (85-100%)

Main topics:

- Using carefully selected external crates and reading their documentation
- More complete testing strategy and documentation
- Cargo features and practical project organization
- Basic threads, channels, and concurrency concepts
- Introductory async Rust only when there is a real reason to use it
- A capstone or a substantial expansion of the Quest Tracker

Outcome: independently plan, implement, debug, test, and explain a moderately sized Rust program while making sensible ownership and error-handling decisions.

## Flexible Near-Term Sequence

1. Learn `Result<T, E>` carefully.
2. Practice the `?` operator and error propagation.
3. Learn basic file input/output.
4. Introduce modules and focused testing.
5. Decide whether the Quest Tracker is ready to begin.

## 2026-10-09 Planning Checkpoint

In the first chat on 2026-10-09, remind the learner about the planned project checkpoint. If there is no chat that day, give the reminder in the first later chat. The learner may or may not be ready to start; readiness matters more than the date.

At that checkpoint:

- Review the actual completed exercises and remaining weak spots.
- Re-estimate progress rather than blindly reusing the current percentages.
- Decide whether to start the real Quest Tracker or do another focused prerequisite exercise.
- Create or update a comprehensive project plan and AI handoff document that a less capable assistant can follow without changing the learning pace.
- Make the handoff explicit about completed knowledge, weak spots, teaching rules, project scope, milestones, file layout, tests, and prohibited shortcuts.

An AI cannot send a timed notification by itself in an inactive chat. This dated instruction is recorded so any future tutor that reads the repository context can remind the learner during a session on or after the checkpoint date.
