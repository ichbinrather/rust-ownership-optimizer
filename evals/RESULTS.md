# Eval results

## Summary

| Run | Without skill | With skill |
|-----|---------------|------------|
| Basic scenarios (6 × 3) | 17/18 | 18/18 |
| Harder scenarios (4 × 3) | 12/12 | 12/12 |
| Feature tasks (3 × 3), clean result | 6/9 | 18/18 (v0.3 and v0.4) |

Model: Claude Sonnet. Every run in every group produced working code. The difference
is in what it costs:

- When the borrow problem **is** the task, Sonnet rarely takes a shortcut, with or
  without the skill (one `self.players.clone()` in 30 runs).
- When the borrow problem shows up **inside a feature task**, all three runs without
  the skill cloned every player's name on every tick to get past the borrow checker.
  None of the 18 runs with the skill did.
- The diff guard (v0.4) did not change the numbers here, because the runs with the
  v0.3 skill already added no workarounds. Its job is to catch the cases the rules miss.

Smaller models have not been tested yet.

## November 2026, Claude Sonnet: feature tasks

Three small multi-file projects in [`projects/`](projects/), each with a feature to
add. The borrow problem is not mentioned in the prompt; it appears on the way. Three
conditions, three attempts each: no skill, the v0.3 skill (rules only), and the v0.4
skill (rules plus the diff guard).

| Project | Task | No skill | v0.3 skill | v0.4 skill |
|---------|------|----------|------------|------------|
| `game_tick` | add `World::tick`, which logs and heals players while iterating them | 3/3 works, **3 clones** | 3/3 works, 0 | 3/3 works, 0 |
| `doc_summaries` | add a cached `summary(&mut self) -> &str` (conditional borrow return) | 3/3 works, 0 | 3/3 works, 0 | 3/3 works, 0 |
| `parallel_orders` | split `process_all` across 4 threads | 3/3 works, 0 | 3/3 works, 0 | 3/3 works, 0 |

"Works" means the program prints the reference output (and, for `parallel_orders`, uses
threads). The second number is the count of new ownership workarounds in the diff, as
counted by [`ownership_guard.py`](../scripts/ownership_guard.py), including justified ones.

In `game_tick`, all three runs without the skill wrote the same shape:

```rust
let (id, name, hp, max_hp) = {
    let p = &self.players[i];
    (p.id, p.name.clone(), p.hp, p.max_hp)
};
```

That allocates a copy of every player's name on every tick, including players that
need no log line. All six runs with the skill kept a shared borrow, built the message
with `format!` directly from `p.name`, and only then called `log_event`.

In `parallel_orders`, all 18 runs used `std::thread::scope` with borrowed chunks; none
cloned the orders or the tax table into threads.

Patches: [`runs/2026-11-sonnet-features/`](runs/2026-11-sonnet-features/). Re-grade with
`python evals/grade_projects.py evals/runs/2026-11-sonnet-features`.

## November 2026, Claude Sonnet: harder scenarios

Four scenarios, three attempts each, with and without the skill.

| Scenario | What it tests | Without skill | With skill |
|----------|---------------|---------------|------------|
| `closure_lifetime` | boxed closures borrowing a config (`+ '_` instead of cloning) | 3/3 | 3/3 |
| `nll_case3` | conditional borrow return the current borrow checker rejects | 3/3 | 3/3 |
| `cow_normalize` | avoid allocating when the input does not change | 3/3 | 3/3 |
| `split_borrow_method` | `&mut self` method called while iterating a field | 3/3 | 3/3 |

All 24 outputs compiled, printed the original output, and avoided the shortcut each
scenario looks for. Approaches varied within both groups: in `cow_normalize`, two of
three runs with the skill and one of three without it returned `Cow<str>`; the others
removed the allocation by counting bytes directly. In `split_borrow_method`, two runs
without the skill and one with it used `std::mem::take`; the rest passed disjoint fields
to a helper function.

Outputs: [`runs/2026-11-sonnet-hard/`](runs/2026-11-sonnet-hard/).

## November 2026, Claude Sonnet: basic scenarios

Six scenarios, three attempts each, with and without the skill: 36 runs in total.

| Scenario | Without skill | With skill |
|----------|---------------|------------|
| `e0382_moved_value` | 3/3 | 3/3 |
| `clone_heavy` | 3/3 | 3/3 |
| `boxed_everything` | 3/3 (0/3 under the first criterion) | 3/3 |
| `iterate_and_mutate` | 2/3 | 3/3 |
| `thread_clone` | 3/3 | 3/3 |
| `lifetime_static` | 3/3 | 3/3 |
| **Total** | **17/18** (14/18 under the first criterion) | **18/18** |

All 36 outputs compiled and printed the same result as the original program.

### What the difference actually is

- **`boxed_everything`**: every run removed the needless heap allocations. Without
  the skill, all three runs replaced `Box<dyn Greeter>` with `&dyn Greeter`; with the
  skill, all three used a generic (`&impl Greeter`). The first version of the check
  required static dispatch, which was the skill's own preference rather than a
  correctness requirement. Both the skill and the check were changed afterwards to
  accept `&dyn`, and the same outputs were re-graded: 3/3 for both groups.
- **`iterate_and_mutate`**: one run without the skill fixed E0502 by iterating over
  `self.players.clone()`, which is the exact shortcut this project is about
  ([`base-3.rs`](runs/2026-11-sonnet/iterate_and_mutate/base-3.rs)). No run with the
  skill did that.
- Everything else was solved correctly with or without the skill. Even under the
  "do whatever it takes" prompt, no run used `'static` or `Box::leak`.

### Limitations

- Three attempts per scenario is a small sample.
- The scenarios turned out to be easy for this model. Harder cases (closure captures,
  conditional borrow returns, tree structures that seem to need `Rc<RefCell<_>>`,
  `Mutex` guards across `.await`) and smaller models are still to be tested.
- The runs with the skill were told to read `SKILL.md` before starting. This measures
  the skill's content, not whether the agent loads the skill on its own.

### Method

- Each attempt ran in its own Cargo project containing only the scenario's
  `src/main.rs`. The hint comment at the top of each scenario file was removed so the
  baseline could not read the intended fix.
- Runs without the skill were told not to use any skills. Runs with the skill were
  told to read `SKILL.md` and the files it references, and nothing else outside the
  project.
- Prompts are the ones in [`evals.json`](evals.json).
- Grading is automatic: [`grade.py`](grade.py) compiles and runs every output,
  compares the output with the original program, and applies the per-scenario checks.
  The agents' own reports were not used.

Every output is in [`runs/2026-11-sonnet/`](runs/2026-11-sonnet/). To re-grade:

```text
python evals/grade.py evals/runs/2026-11-sonnet
```
