# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses
[Semantic Versioning](https://semver.org/).

## [0.3.0]

### Changed
- `&dyn Trait` is no longer flagged. The skill only removes the `Box` around a borrowed
  trait object and no longer pushes generics for static dispatch.

### Added
- Four harder eval scenarios: closure lifetimes, conditional borrow return, `Cow`,
  and `&mut self` methods inside field loops.
- `evals/grade.py` and the first results on Claude Sonnet in `evals/RESULTS.md`.

## [0.2.0]

### Added
- Ownership workflow: ownership roles, error-to-design-question mapping, escalation after
  three failed fixes.
- `references/error-codes.md` and `references/patterns.md` (closures, threads, async,
  iterators, conditional borrow returns, `Cow`, arenas).
- `examples/advanced_patterns.rs`, Cargo project for running examples on any OS.
- Windows installer (`install.ps1`) with junction support; `--force` for `install.sh`.
- `AGENTS.md` and Codex metadata for multi-agent support.
- Clippy audit scripts for PowerShell and POSIX shells.
- Eval scenarios in `evals/`.
- CI on Windows, macOS, and Linux.

## [0.1.0]

### Added
- Initial `SKILL.md` with clone, `Box`, borrow, allocation, lifetime, and smart pointer
  checklists.
- Before/after examples for clones, `Box`, and borrow conflicts.
