# Evals

These scenarios measure whether the skill changes agent behavior. Each one is a Rust
file with a typical ownership mistake and a prompt that real users write.

| Scenario | Starts as | Tests |
|----------|-----------|-------|
| `e0382_moved_value.rs` | compile error E0382 | borrow instead of clone |
| `clone_heavy.rs` | compiles, wasteful | needless clones, `&Vec<T>`, `entry()` |
| `boxed_everything.rs` | compiles, wasteful | needless `Box`, static dispatch |
| `iterate_and_mutate.rs` | compile error E0502 | disjoint field borrows |
| `thread_clone.rs` | compiles, wasteful | `thread::scope` / `Arc::clone` |
| `lifetime_static.rs` | compile error E0515 | no `'static` / `Box::leak` under pressure |

Prompts, expected results, and pass/fail assertions are in `evals.json`.

## Running an eval by hand

Run every scenario twice: once with the skill installed, once without it.

1. Copy the scenario into a scratch Cargo project as `src/main.rs`.
2. Give the agent the prompt from `evals.json`.
3. Check the result:
   - `cargo run` succeeds and the output is unchanged.
   - Every assertion in `evals.json` holds.
   - Count remaining `.clone()` calls:
     - Windows (PowerShell): `(Select-String -Path src\main.rs -Pattern '\.clone\(\)').Count`
     - macOS / Linux: `grep -c '\.clone()' src/main.rs`

## Grading

`grade.py` compiles and runs every output in a run folder and applies the checks from
`evals.json`. Results so far are in [RESULTS.md](RESULTS.md).

```text
python evals/grade.py evals/runs/2026-11-sonnet
```

## Recording results

Put each attempt in `runs/<date>-<model>/<scenario>/<base|skill>-<n>.rs`, grade it, and
add a section to RESULTS.md.
Pull requests with new scenarios are welcome: keep each one small, realistic, and tied
to one rule in `SKILL.md`.
