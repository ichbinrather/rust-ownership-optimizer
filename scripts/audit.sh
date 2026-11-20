#!/usr/bin/env sh
# Run Clippy with the ownership and allocation lints used by the rust-ownership-optimizer skill.
# Usage: run inside a Cargo project:  sh path/to/scripts/audit.sh
set -eu
cargo clippy --all-targets -- \
  -W clippy::redundant_clone \
  -W clippy::implicit_clone \
  -W clippy::unnecessary_to_owned \
  -W clippy::clone_on_ref_ptr \
  -W clippy::needless_pass_by_value \
  -W clippy::trivially_copy_pass_by_ref \
  -W clippy::large_types_passed_by_value \
  -W clippy::box_collection \
  -W clippy::boxed_local \
  -W clippy::needless_collect \
  -W clippy::ptr_arg \
  -W clippy::rc_buffer
