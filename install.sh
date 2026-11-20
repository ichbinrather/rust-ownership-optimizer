#!/usr/bin/env sh
# Install the rust-ownership-optimizer skill for every supported agent found on this machine.
# Usage: ./install.sh [--link] [--all] [--force]
#   --link  create symlinks instead of copies (updates follow `git pull`)
#   --all   install for every agent, even if its config directory does not exist yet
#   --force replace an existing installation
set -eu

NAME="rust-ownership-optimizer"
SRC="$(cd "$(dirname "$0")" && pwd)"
MODE="copy"
ALL="no"
FORCE="no"

for arg in "$@"; do
  case "$arg" in
    --link) MODE="link" ;;
    --all) ALL="yes" ;;
    --force) FORCE="yes" ;;
    *) echo "Unknown option: $arg" >&2; exit 1 ;;
  esac
done

# "<agent config dir>:<skills dir>"
TARGETS="
$HOME/.claude:$HOME/.claude/skills
$HOME/.codex:$HOME/.codex/skills
$HOME/.agents:$HOME/.agents/skills
$HOME/.config/opencode:$HOME/.config/opencode/skills
$HOME/.cursor:$HOME/.cursor/skills
$HOME/.gemini:$HOME/.gemini/skills
$HOME/.copilot:$HOME/.copilot/skills
"

installed=0
for entry in $TARGETS; do
  base="${entry%%:*}"
  dir="${entry#*:}"
  if [ "$ALL" = "no" ] && [ ! -d "$base" ]; then
    continue
  fi
  mkdir -p "$dir"
  dest="$dir/$NAME"
  if [ -e "$dest" ] || [ -L "$dest" ]; then
    if [ "$FORCE" = "no" ]; then
      echo "skip   $dest (already exists; use --force to replace)"
      continue
    fi
    if [ -L "$dest" ]; then rm "$dest"; else rm -rf "$dest"; fi
  fi
  if [ "$MODE" = "link" ]; then
    ln -s "$SRC" "$dest"
  else
    mkdir -p "$dest"
    cp -R "$SRC/SKILL.md" "$SRC/AGENTS.md" "$SRC/references" "$SRC/examples" "$SRC/scripts" "$SRC/agents" "$dest/"
  fi
  echo "ok     $dest"
  installed=$((installed + 1))
done

echo "Installed for $installed agent(s). Restart your agent to load the skill."
