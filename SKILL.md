---
name: rust-ownership-optimizer
description: Guides AI coding agents on Rust ownership, borrowing, lifetimes, and memory optimization; reduces unnecessary clones and allocations by leveraging compile-time ownership analysis. Use when writing, reviewing, or refactoring Rust code, when fixing borrow checker errors (E0382, E0499, E0502, E0505, E0506, E0507, E0515, E0597, E0716, E0106), or when code contains `.clone()`, `Box`, `Rc`, `RefCell`, `Arc`, `Mutex`, `to_string()`, or `to_owned()`.
license: MIT
metadata:
  version: "0.3.0"
---

# Rust Ownership Optimizer

## Core Principle

Rust has **no garbage collector and no runtime memory manager**. Memory is managed by
**compile-time ownership analysis**: the compiler (borrow checker) proves at compile time
who owns each value, how long each reference lives, and exactly where each value is
dropped. Deallocation is deterministic and inserted by the compiler at scope end (RAII).

Never describe Rust memory management with terms borrowed from runtime-managed
languages. Never write "the value is collected", "the collector frees it", or similar.
Say "the value is dropped at the end of its owner's scope".

Your goal: produce code that satisfies the borrow checker **by design**, not by
copying data until the errors disappear.

## The Ownership Rules (always apply)

1. Each value has exactly one owner.
2. When the owner goes out of scope, the value is dropped.
3. At any time, a value has **either** any number of shared references (`&T`) **or**
   exactly one mutable reference (`&mut T`), never both.
4. A reference must never outlive the value it points to.

## Workflow

Follow these steps for every Rust task in scope.

1. **Ask the ownership question first.** Before touching code, answer:
   *Who should own this data, and for how long?* Classify the value:
   - **Unique entity** (has identity, one place of truth): one owner, others borrow.
   - **Value object** (small, interchangeable): `Copy` or a cheap `clone()` is fine.
   - **Shared immutable data** (config, audit records, lookup tables): `Arc<T>` / `Rc<T>`
     or plain `&T` with a lifetime, never deep copies.
   - **Temporary** (intermediate result): restructure so it is never stored.
2. **Map the compiler error to a design question**, not to a quick fix.
   See `references/error-codes.md`.
3. **Apply the review checklist** below.
4. **Verify** with `cargo check` and Clippy (see Verification).
5. **Escalate after three failed attempts.** If the same borrow error survives three
   local fixes, stop patching. Re-examine the data model: ownership roles, struct
   boundaries, or whether the data should be split, indexed, or shared.

## Decision Order for Parameters and Fields

Pick the **first** option that compiles and expresses the intent:

| Need | Use | Runtime cost |
|------|-----|--------------|
| Read only | `&T`, `&str`, `&[T]`, `&Path` | none |
| Modify in place | `&mut T` | none |
| Store or consume the value | `T` (caller moves in) | none (move) |
| Usually borrowed, sometimes modified | `Cow<'_, T>` | allocation only on write |
| Accept many input types | `impl AsRef<T>` / `impl Into<T>` | none / conversion |
| Shared ownership, single thread | `Rc<T>` | non-atomic count |
| Shared ownership, multi thread | `Arc<T>` | atomic count |
| Mutation behind shared ownership | `Cell` / `RefCell` / `Mutex` / `RwLock` | runtime check (last resort) |

Prefer borrowed slice types in signatures: `&str` over `&String`, `&[T]` over `&Vec<T>`,
`&Path` over `&PathBuf`.

## Review Checklist

### 1. Unnecessary `clone()`

Flag a `.clone()` (or `.to_string()`, `.to_owned()`, `.to_vec()`) when:

- The clone is passed to a function that only reads it. Fix: change the parameter to `&T`.
- The original is never used after the clone. Fix: move the original instead.
- The clone exists only to silence E0382 (use after move). Fix: reorder code, borrow, or
  restructure so the last use moves.
- The clone is inside a loop over a collection that could be iterated by reference
  (`for x in &items`) or consumed (`for x in items`).
- A `String` is cloned only to be compared or hashed. Fix: compare `&str`.
- An `Option<T>` / `Result<T, E>` is cloned to inspect it. Fix: `.as_ref()`, `.as_deref()`,
  `if let Some(x) = &opt`.
- A struct field is cloned out to call a method while `self` is borrowed. Fix: borrow
  disjoint fields separately, or use `std::mem::take` / `std::mem::replace`.

A clone is **justified** when: the value is a cheap value object, two owners genuinely
need independent copies, the data crosses a thread boundary without `Arc`, or the clone
is of an `Rc`/`Arc` (a reference-count increment, not a deep copy). Write `Rc::clone(&x)`
to make that explicit.

### 2. Unnecessary `Box`

Flag `Box<T>` when:

- `T` is a sized value used only locally (`let x = Box::new(5);`). Use the value directly.
- `Box<Vec<T>>`, `Box<String>`, `Box<HashMap<..>>`: these already own heap storage.
- A parameter is `Box<dyn Trait>` but the function only uses the value during the call.
  Take a reference instead: `&dyn Trait` or `&impl Trait`. Both avoid the allocation.
- A function returns `Box<dyn Iterator>` where `impl Iterator` works.

`&dyn Trait` and generics are both fine; the problem is the `Box`, not the `dyn`. Do not
rewrite working `&dyn` code into generics just for static dispatch. Prefer generics only
when the call is in a measured hot path, and keep `&dyn` when it avoids duplicated code
from monomorphization or keeps compile times and binary size down.

`Box` is **justified** for: recursive types (`enum List { Cons(i32, Box<List>), Nil }`),
heterogeneous collections of trait objects, very large values moved often, and
`Box<dyn Error>` at application boundaries.

### 3. Borrow Conflicts

Fix these patterns without cloning:

- **Shared + mutable overlap (E0502, E0499):** finish the read, keep the small value you
  need (an index, a `Copy` value, `len()`), then mutate. A borrow ends at its last use
  (non-lexical lifetimes), so shorten the borrow instead of copying the data.
- **Mutating a collection while iterating it:** use `retain`, `retain_mut`, `drain(..)`,
  `iter_mut()`, `split_at_mut`, or collect indices first and apply after.
- **Two fields of `self`:** access fields directly (`&mut self.a`, `&self.b`) instead of
  through getters that borrow all of `self`.
- **Map lookup then insert:** use the `entry()` API.
- **Moving out of a reference (E0507):** use `std::mem::take`, `mem::replace`,
  `Option::take`, or borrow the inner value.
- **Value does not live long enough (E0597, E0716, E0515):** bind temporaries to a name,
  move the owner to a wider scope, or return owned data.

### 4. Allocations

Measure before micro-optimizing: algorithm and data structure choices outweigh
allocation tweaks. Then:

- Pre-size: `Vec::with_capacity(n)`, `String::with_capacity(n)`.
- Avoid `collect::<Vec<_>>()` when the result is iterated once: chain iterators.
- Build strings with `push_str` / `write!` in loops, not repeated `format!` or `+`.
- Return `&str` / `&[T]` views into existing data instead of new `String` / `Vec`.
- Use `Cow<'_, str>` when a function usually returns its input unchanged.
- Benchmark only in `--release`.

### 5. Lifetimes

- Rely on lifetime elision first. Add explicit lifetimes only when the compiler needs them.
- Never use `'static` to silence an error unless the data truly lives for the whole program.
- A struct holding references (`struct Parser<'a> { input: &'a str }`) avoids copying.
  Use it for short-lived views.
- If annotations grow complex, reconsider ownership: the struct may need to own its data.

### 6. Smart Pointers and Interior Mutability

- `Rc<RefCell<T>>` everywhere signals an ownership design problem. Try a single owner
  with borrowed access, an arena with indices, or message passing first.
- `RefCell` moves borrow checks to runtime and can panic. State why compile-time checks
  are not possible before using it. Prefer `Cell<T>` for `Copy` values.
- Read-heavy `Arc<Mutex<T>>`: consider `RwLock`, or share an immutable `Arc<T>`.
- Break reference cycles with `Weak<T>`, or the values are never dropped.

## Output Format

When reviewing code, report each finding as:

```
[category] location: problem -> fix (why it is cheaper or safer)
```

Categories: `clone`, `box`, `borrow`, `alloc`, `lifetime`, `smart-ptr`.

Then show the optimized code. Keep behavior identical. If a change alters the public
API (for example a `String` parameter becomes `&str`), say so.

## Verification

Run when a Cargo project is available. The commands work the same in PowerShell, cmd,
and POSIX shells:

```
cargo check
cargo clippy --all-targets -- -W clippy::redundant_clone -W clippy::implicit_clone -W clippy::unnecessary_to_owned -W clippy::needless_pass_by_value -W clippy::box_collection -W clippy::boxed_local
cargo test
```

For the full ownership lint set, run the bundled script from the project root:
`scripts/audit.ps1` on Windows, `scripts/audit.sh` on macOS and Linux.

`clippy::redundant_clone` and `clippy::needless_collect` are in Clippy's nursery group
and can report false positives. Confirm each finding against the rules above before
changing code.

Do not claim an optimization compiles until `cargo check` passes. If no toolchain is
available, say that the code was not compiled.

## Anti-Patterns to Refuse

- Adding `.clone()` at every error site until the code compiles.
- Wrapping values in `Rc<RefCell<_>>` to escape the borrow checker.
- Using `unsafe` to bypass borrow rules.
- Changing a lifetime to `'static` or leaking memory (`Box::leak`) to fix a lifetime error.

## References

- `references/error-codes.md`: borrow checker error codes, the design question behind
  each, and fixes that avoid copying. Read when a compiler error is present.
- `references/patterns.md`: closures, threads and async tasks, iterators, conditional
  borrow returns, `Cow`, graphs and arenas. Read when the code uses any of these.
- `examples/`: compilable before/after programs for clones, `Box`, borrow conflicts,
  and the advanced patterns.
- `scripts/audit.ps1`, `scripts/audit.sh`: Clippy with the full ownership lint set.
