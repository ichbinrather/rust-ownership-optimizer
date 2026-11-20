# Feature-task projects

Each folder is a small crate. Give the agent the prompt below inside a git repository
that contains only that folder, then save `git diff` as a patch and grade it with
`evals/grade_projects.py`.

## game_tick

> Add a `tick(&mut self)` method to `World` in src/world.rs. `main.rs` already calls it.
> Each tick: increase `turn` by 1 first. Then go through the players in order: a player
> with 0 hp gets the log event "<name> is down" (use `log_event`) and nothing else; a
> player below 30% of their max_hp gets the log event "<name> is low on health" and is
> healed by 25 with `heal`. Make sure `cargo run` works.

## doc_summaries

> Add `pub fn summary(&mut self, idx: usize) -> &str` to `Store` in src/store.rs;
> `main.rs` already calls it. A summary is the doc's title, then ": ", then the first
> sentence of the body (up to and including the first '.', or the whole body if there
> is none). Compute each summary at most once: cache it in `summaries` and increase
> `computed` only when a summary is actually computed. Make sure `cargo run` works.

## parallel_orders

> `Pipeline::process_all` in src/pipeline.rs is too slow on big inputs. Make it split
> the work across 4 threads using only the standard library. Keep the signature, and
> the totals must stay identical and in the same order.
