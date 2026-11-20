# Contributing

## Good first contributions

- **Report a bad suggestion.** If an agent added a needless clone, `Box`, `Rc<RefCell<_>>`,
  or `'static` while using this skill, open a "Bad suggestion" issue with the code.
- **Add an eval scenario.** Put a small Rust file in `evals/files/` and its prompt and
  assertions in `evals/evals.json`.
- **Add a pattern.** Add the rule to `SKILL.md` or a file in `references/`, plus a
  compilable before/after example.

## Development setup

Works on Windows, macOS, and Linux with a stable Rust toolchain.

```
cargo build --examples
cargo clippy --examples -- -D warnings
python scripts/validate_skill.py
```

Install your working copy as a live link while you edit:

- Windows (PowerShell): `.\install.ps1 -Link -Force`
- macOS / Linux: `./install.sh --link --force`

## Style

- All skill content, code comments, and commit messages are in English.
- Use [Conventional Commits](https://www.conventionalcommits.org/): `feat:`, `fix:`, `docs:`.
- Describe Rust memory as compile-time ownership: values are dropped at the end of
  their owner's scope. Rust has no garbage collector.
- Keep `SKILL.md` under 500 lines. Move detail into `references/`.
