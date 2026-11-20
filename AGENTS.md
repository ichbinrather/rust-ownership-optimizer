# Rust Ownership Rules for Coding Agents

This file is for agents that read `AGENTS.md` (Codex, Cursor, GitHub Copilot, Zed,
Jules, Aider, and others). The full skill is in `SKILL.md`; detailed error guidance is in
`references/error-codes.md`.

When writing or reviewing Rust in this repository:

- Rust has no garbage collector. Memory is freed when the owner goes out of scope, as
  proven by compile-time ownership analysis. Describe it that way.
- Ask "who should own this data, and for how long?" before fixing any borrow error.
- Prefer, in order: `&T` / `&str` / `&[T]`, then `&mut T`, then owned `T`, then `Cow`,
  then `Rc` / `Arc`, then interior mutability as a last resort.
- Do not add `.clone()`, `Box`, `Rc<RefCell<_>>`, `'static`, `Box::leak`, or `unsafe`
  only to make the compiler accept the code.
- Remove `Box<Vec<T>>`, `Box<String>`, boxed locals, and `Box<dyn Trait>` where generics
  or `impl Trait` work.
- Fix borrow conflicts by shortening borrows, borrowing disjoint fields, using `entry()`,
  `retain`, `std::mem::take`, or `split_at_mut`.
- After three failed fixes of the same error, redesign the data ownership.
- Verify with `cargo check` and
  `cargo clippy -- -W clippy::redundant_clone -W clippy::needless_pass_by_value`.
- Report findings as `[category] location: problem -> fix (reason)` with categories
  `clone`, `box`, `borrow`, `alloc`, `lifetime`, `smart-ptr`.
