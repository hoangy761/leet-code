---
name: leetcode-solution-notes
description: >-
  Write or update README.md for a LeetCode solution in this repo from the
  code in that solution folder, so a later reread shows how this attempt
  actually works. Use when the user asks to write a markdown file for this
  code, viết file md, viết md cho code này, viết md cho bài này, or a
  README for a solution under leet-code.
---

# LeetCode solution notes

Write `README.md` from the solution that is already in the folder. The note is for rereading later: how this attempt works, in the order the code runs.

## Which solution

```
leet-code/
  Cargo.toml                 # workspace; each member is one solution
  <problem-slug>/
    Cargo.toml
    src/main.rs              # Solution impl and a local main
    README.md                # this skill writes this file
```

- One README per solution folder, next to that folder's `Cargo.toml`.
- The focused file or a named folder picks the solution. Read `src/main.rs` and any other source in that folder before writing.
- If more than one solution is open and none is clearly selected, ask which folder.
- Do not change the Rust code unless the user asks.

## README.md

English. Title is the folder slug in title case (`remove-element` → Remove Element). State the LeetCode number only when the function name and signature make it unambiguous.

```markdown
# Title

One or two sentences: inputs and what the function returns, taken from the signature and body.

## What this solution does

The idea behind this file: structure, pointers or stack, and why the main loop is shaped this way. Use the real identifiers.

## Walkthrough

1. Steps in source order, including early returns and special cases.
2. A reader should match each step to a line in `src/main.rs`.

## Example

Trace one small input. Use the call in `main` when it hits the interesting path; otherwise pick a short input that does. Show the index, stack, or swap at the points that matter, then the return value and the kept prefix when the function mutates the input.

## Complexity

- Time: bound for this implementation, tied to the loops that are written.
- Space: extra memory for this implementation.

## Edge cases in this code

- Only cases this code special-cases, or that fall out of the same path.

## Cleaner approach

A few sentences when a shorter approach exists: the idea, and why it is simpler than this file. No second full program.
```

Omit **Cleaner approach** when this file is already the straightforward version. A few lines of sketch are fine when prose alone is vague. Do not present that sketch as the code in the folder.

## Rules

- Describe the code that is there, including awkward branches. Do not narrate a textbook solution as if it were this file.
- Keep identifiers as written (`left_index`, `open_scores`). Do not rename them in the note.
- Stay short enough to reread before retrying the problem. Skip generic LeetCode advice.
- If `README.md` already exists, replace it so it matches the current code.
