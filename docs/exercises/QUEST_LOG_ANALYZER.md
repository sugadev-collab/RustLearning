# Quest Log Analyzer - Big Rust Checkpoint

This is a mixed checkpoint exercise. It is meant to be a little challenging, so searching the Rust docs or examples is allowed.

Do not use external crates.

## Goal

Build a small program that stores quests/tasks, analyzes them, and prints a report.

This exercise should combine:

- `String` and `&str`
- string slices
- structs
- enums
- `Option`
- `match`
- `if let`
- vectors
- borrowing versus moving
- iterator methods: `.iter()`, `.filter()`, `.map()`, `.sum()`, `.collect()`

## Starter Types

Use these types, or something very close to them:

```rust
enum QuestStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
}

struct Quest {
    title: String,
    description: String,
    reward_xp: u32,
    status: QuestStatus,
    tag: Option<String>,
}
```

## Data Requirements

Create a `Vec<Quest>` with at least 5 quests.

Your quests must include:

- at least one `Completed` quest
- at least one `InProgress` quest
- at least one `Failed` quest
- at least one quest with `Some(tag)`
- at least one quest with `None`

## Program Requirements

### 1. Describe Every Quest

Write this function:

```rust
fn describe_quest(quest: &Quest)
```

Inside the function:

- print the quest title
- print the reward XP
- use `match` on the quest status
- print the tag using `match` or `if let`
- print a short description preview using a string slice

Example output idea:

```text
Title: Find the Lost Sword
Status: Completed
XP: 100
Tag: combat
Preview: Find ...
```

## 2. Use A String Slice

Somewhere inside `describe_quest`, print a short preview of the description.

For example:

```rust
let preview = &quest.description[0..5];
```

But be careful: slicing can panic if the string is shorter than the range.

Hint:

```rust
if quest.description.len() >= 5 {
    // safe for simple ASCII text
}
```

For this exercise, use ASCII-only descriptions so byte slicing does not become the main topic.

## 3. Collect Completed Quests

Use `.iter().filter(...)` and `.collect::<Vec<_>>()` to collect only completed quests.

Idea:

```rust
let completed = quests
    .iter()
    .filter(...)
    .collect::<Vec<_>>();
```

The result should borrow quests, not consume them.

## 4. Sum Completed XP

Use `.iter().map(...).sum::<u32>()` to calculate the total XP from completed quests only.

You may calculate this from either:

- the original `quests` vector with `filter + map + sum`
- the collected `completed` vector with `map + sum`

## 5. Prove The Original Vector Still Exists

After filtering and summing, print something from the original vector:

```rust
println!("Total quests: {}", quests.len());
```

This proves you borrowed the vector instead of consuming it.

## Search Challenges

Search these if you get stuck:

- Rust `matches!` macro
- Rust `Option as_ref`
- Rust match borrowed enum field
- Rust iterator filter collect Vec
- Rust string slice safety

## Important Rules

- Do not use external crates.
- Do not use `clone()` unless you can explain why.
- Prefer borrowing with `&Quest` and `.iter()`.
- It is okay if your first version has compiler errors.
- Do not worry about making the output beautiful first. Correct Rust comes first.

## Suggested Build Order

1. Define `QuestStatus`.
2. Define `Quest`.
3. Create one quest and print it.
4. Write `describe_quest(&Quest)`.
5. Create the full `Vec<Quest>`.
6. Loop through `quests.iter()` and describe every quest.
7. Add completed quest filtering.
8. Add completed XP sum.
9. Print `quests.len()` at the end.

## Expected Concepts To Practice

When I review your solution, I will look for:

- Did you avoid accidentally moving the vector?
- Did you borrow quests with `&Quest`?
- Did your `match` handle every status?
- Did your `Option<String>` handling avoid unnecessary ownership moves?
- Did your iterator chain produce the right result?
- Did your string slice avoid panic for short descriptions?

## Next Step

Implement the exercise in `src/main.rs`.

When done, say:

```text
done
```

If you get stuck, paste the compiler error or ask a focused question.
