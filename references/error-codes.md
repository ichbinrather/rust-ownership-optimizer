# Borrow Checker Error Codes

Each error is a symptom. Answer the design question before choosing a fix. Fixes are
listed from cheapest (no copy, no runtime cost) to most expensive. Use `rustc --explain
EXXXX` for the official description.

| Code | Meaning | Design question | Preferred fixes (in order) |
|------|---------|-----------------|----------------------------|
| E0382 | Use of a moved value | Who should own this data after the move? | Borrow (`&T`) instead of moving; reorder so the last use moves; share with `Rc`/`Arc` if many owners are real; `clone()` only for a value object |
| E0499 | Two mutable borrows at once | Do both paths really need write access at the same time? | Sequence the borrows; split with `split_at_mut` / disjoint fields; iterate with `iter_mut()` |
| E0502 | Mutable borrow while a shared borrow is alive | Can the read finish before the write? | Copy out the small value you need, then mutate; use `entry()` for maps; use `retain` / `drain` for collections |
| E0505 | Move out while borrowed | Should the borrower or the new owner come first? | End the borrow before the move; move a smaller part |
| E0506 | Assign to a borrowed value | Should the mutation happen somewhere else? | End the borrow first; compute the new value, then assign |
| E0507 | Move out of a borrowed reference | Why move from data you do not own? | Borrow the inner value; `std::mem::take` / `mem::replace` / `Option::take`; take `self` by value if the method consumes it |
| E0515 | Return reference to a local | Should the caller own the result? | Return the owned value; return a reference derived from an input parameter |
| E0597 | Borrowed value does not live long enough | Is the owner declared in the right scope? | Move the owner to the outer scope; return owned data; restructure so the reference is shorter-lived |
| E0716 | Temporary dropped while borrowed | Why is this value temporary? | Bind it to a named variable; keep the owned value |
| E0106 | Missing lifetime specifier | Which input does the output borrow from? | Return owned data if it borrows nothing; tie the output to the right input with one named lifetime |
| E0596 | Borrow as mutable from an immutable binding | Should this binding be mutable at all? | Add `mut` to the binding or take `&mut self`; `Cell` / `RefCell` only for real shared mutation |

## Rules That Apply to Every Code

- Do not add `.clone()` as the first fix. Check the borrow options first.
- Do not answer a lifetime error with `'static` or `Box::leak`.
- Three failed local fixes for the same error mean the data model is wrong. Revisit who
  owns what: split the struct, store indices instead of references, or own the data.

## Ownership Roles

| Role | Typical type | Copy cost acceptable? |
|------|--------------|-----------------------|
| Unique entity (one source of truth) | owned `T`, borrowed elsewhere | No |
| Value object (small, interchangeable) | `Copy` struct, short `String` | Yes |
| Shared immutable data | `Arc<T>`, `Rc<T>`, `&'a T` | No, share it |
| Temporary result | local value, iterator chain | Avoid storing it |
