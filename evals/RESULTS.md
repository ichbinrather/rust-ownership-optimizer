# Eval results

## November 2026, Claude Sonnet

Six scenarios, three attempts each, with and without the skill: 36 runs in total.

| Scenario | Without skill | With skill |
|----------|---------------|------------|
| `e0382_moved_value` | 3/3 | 3/3 |
| `clone_heavy` | 3/3 | 3/3 |
| `boxed_everything` | 0/3 | 3/3 |
| `iterate_and_mutate` | 2/3 | 3/3 |
| `thread_clone` | 3/3 | 3/3 |
| `lifetime_static` | 3/3 | 3/3 |
| **Total** | **14/18** | **18/18** |

All 36 outputs compiled and printed the same result as the original program.

### What the difference actually is

- **`boxed_everything`**: every run removed the needless heap allocations. Without
  the skill, all three runs replaced `Box<dyn Greeter>` with `&dyn Greeter`; with the
  skill, all three used a generic (`&impl Greeter`). The pass criterion asks for static
  dispatch, which is the skill's own rule, not a correctness requirement. Read this row
  as "the agent followed the skill", not as "the baseline was wrong".
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
