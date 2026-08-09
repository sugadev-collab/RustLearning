# Rust Learning Roadmap

This is the stable roadmap for the current Rust learning path. Percentages are rough navigation markers, not exam scores. A stage is not considered mastered after one successful exercise; important ideas will return inside larger programs.

## Current Position

- Date recorded: 2026-08-08
- Overall curriculum coverage toward intermediate Rust: about 55%
- Ownership and borrowing foundation: about 60-65% of the desired intermediate depth
- Independently reusable mastery: about 35-40%
- Current stage: entering Stage 4, Reusable and Reliable Rust through the Quest Tracker
- Current checkpoint: the 55% prerequisite gate is reached. `Result`, `?`, file I/O, iterator-based parsing, command-line input, a first module, and a first focused unit test are approved. The Quest Tracker may now begin as a learning project.

## How To Read The Percentages

The main percentage measures how far we have travelled through the planned curriculum, not how many Rust questions the learner could answer on an exam. Independently reusable mastery is intentionally lower because a concept needs to work in several different programs before it counts as dependable knowledge.

| Percentage | Curriculum region | Practical meaning |
|---:|---|---|
| 0-25% | Language and memory foundations | Write small programs and reason about basic ownership and borrowing. |
| 25-45% | Rust-style data modeling and control flow | Use structs, enums, patterns, collections, methods, and introductory iterators together. |
| 45-55% | Errors and program boundaries | Use `Result`, `?`, files, command-line input, modules, and initial tests. Prerequisite stage completed once. |
| 55-70% | Reusable and reliable Rust | Build a modular real project and introduce tests, traits, generics, and deliberate error design. **Current: about 55%.** |
| 70-85% | Intermediate ownership and API design | Make stronger lifetime, smart-pointer, iterator, and abstraction decisions. |
| 85-100% | Intermediate consolidation | Use the ecosystem carefully and complete a substantial, tested program independently. |

## Major Progress Markers

| Marker | Evidence expected |
|---:|---|
| 48% | Read and parse a text file; propagate I/O errors; begin file persistence. |
| 50% | Save structured text reliably and report read/write failures clearly. |
| 51% | Accept and validate a basic command-line argument without panicking. |
| 53% | Separate file persistence into a module with deliberate visibility. |
| 55% (current) | Use command-line input, basic modules, and focused tests; decide the first Quest Tracker scope. |
| 70% | Maintain a working modular Quest Tracker while using traits, generics, tests, and better errors where motivated. |
| 85% | Explain and apply intermediate ownership and API-design choices in project code. |
| 100% | Plan, implement, debug, test, and explain a moderately sized Rust program with sensible tradeoffs. |

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

Status: prerequisite stage completed once. These concepts remain active practice areas and will recur inside the Quest Tracker.

Milestone: after these boundary concepts are understood through exercises, decide the first real-project scope.

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

1. **About 48-50%:** finish file output, truncation/overwrite behavior, and explicit I/O error reporting.
2. **About 50-51%:** accept and validate basic command-line arguments.
3. **About 51-53%:** separate the program into small modules with clear visibility.
4. **About 53-55%:** add focused unit tests and reassess weak spots.
5. **At the 55% gate:** choose a realistic first Quest Tracker version and prepare its detailed implementation plan.

## 2026-08-09 Planning Checkpoint

Completed on 2026-08-09. The learner reached the 55% readiness gate, and the authoritative Quest Tracker Version 1 scope plus tutor/AI handoff were created in `docs/projects/QUEST_TRACKER_V1_REQUIREMENTS.md`.

At that checkpoint:

- Review the actual completed exercises and remaining weak spots.
- Re-estimate progress rather than blindly reusing the current percentages.
- Decide whether to start the real Quest Tracker or do another focused prerequisite exercise.
- Create or update a comprehensive project plan and AI handoff document that a less capable assistant can follow without changing the learning pace.
- Make the handoff explicit about completed knowledge, weak spots, teaching rules, project scope, milestones, file layout, tests, and prohibited shortcuts.

The learner will use brief source comments and an evolving file structure instead of a formal planning document. The complete requirements remain authoritative but may be read milestone by milestone. The learner may implement Version 1 independently and request small hints only when blocked.
