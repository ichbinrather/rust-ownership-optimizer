# Advanced Ownership Patterns

Read this file when the code uses closures, threads or async tasks, iterator chains,
"get or insert" logic, or graph-like data. Runnable versions are in
`examples/advanced_patterns.rs`.

## Closures

- A closure borrows what it uses by default. Add `move` only when the closure must
  outlive the current scope (threads, tasks, returned closures).
- Before `move`, bind only the needed field: `let prefix = &config.prefix;` instead of
  cloning `config`.
- Choose the weakest trait bound that works: `Fn` (shared borrow) over `FnMut`
  (mutable borrow) over `FnOnce` (consumes captures).
- Since edition 2021 closures capture disjoint fields, so `|| self.a.len()` does not
  borrow all of `self`.

## Threads and Async Tasks

- `thread::spawn` and `tokio::spawn` require `'static` data. That is a reason to **share**
  (`Arc<T>`), not to deep-copy.
- Clone the `Arc`, not the data: `let data = Arc::clone(&data);` before `move`.
- When threads finish inside the function, use `std::thread::scope`: scoped threads can
  borrow local data with no `Arc` and no copy.
- Hold a `Mutex` guard for the shortest time possible, and never across an `.await`.
  Copy the small value you need out of the guard, drop the guard, then continue.
- Use `Arc<str>` / `Arc<[T]>` for shared immutable strings and slices.

## Iterators

| Need | Use |
|------|-----|
| Read elements | `iter()` / `for x in &v` |
| Modify elements | `iter_mut()` / `for x in &mut v` |
| Consume the collection | `into_iter()` / `for x in v` |
| Copy small `Copy` values | `copied()` (not `cloned()`) |
| Clone heavy values | `cloned()`, only if ownership is truly needed |

- Do not `collect()` into a `Vec` just to iterate again. Chain adapters lazily.
- Use `extend`, `retain`, `drain`, and `dedup` instead of building a second collection.

## Conditional Borrow Return

Returning a borrow from one branch and mutating in another branch is rejected by the
current borrow checker, even though it is sound:

```rust
// Rejected today:
fn get_or_insert<'m>(map: &'m mut HashMap<String, String>, k: &str) -> &'m String {
    if let Some(v) = map.get(k) { return v; }
    map.insert(k.to_string(), String::new());
    map.get(k).unwrap()
}
```

Do not clone the value to escape. Use one of:

1. The `entry` API: `map.entry(k.to_string()).or_default()`.
2. Check first, then borrow once: `if !map.contains_key(k) { map.insert(..); } map.get(k).unwrap()`.
3. For non-map cases, return an index or key instead of a reference.

## Cow (Clone on Write)

Use `Cow<'_, str>` or `Cow<'_, [T]>` when a function returns its input unchanged most of
the time. It borrows in the common case and allocates only when it must change the data.

## Graphs, Trees, and Back-References

- Avoid `Rc<RefCell<Node>>` webs. Store nodes in a `Vec` and refer to them by index
  (an arena). Indices are `Copy`, cannot dangle, and need no runtime borrow checks.
- For parent links in trees, use `Weak<T>` so the cycle does not prevent drops.
- Never build self-referential structs (a field that borrows another field of the same
  struct). Store an index or range into the owned data instead.

## Struct Design

- Own data in long-lived structs (`String`, `Vec<T>`).
- Borrow data in short-lived views (`struct Parser<'a> { input: &'a str }`).
- Take `self` by value for builder methods and conversions (`into_*`), `&self` for
  queries, `&mut self` for updates.
