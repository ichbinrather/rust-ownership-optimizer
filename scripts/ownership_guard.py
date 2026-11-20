#!/usr/bin/env python3
"""Flag ownership workarounds that were just added to Rust code.

Looks only at lines added in the current change, so existing code does not
create noise. Every finding needs either a real fix or a justification comment
on the same line or the line above:

    let snapshot = state.clone(); // ownership: snapshot must outlive the lock

Usage:
    ownership_guard.py                  # working tree vs HEAD, plus untracked .rs files
    ownership_guard.py --staged         # staged changes (for a git pre-commit hook)
    ownership_guard.py --base main      # changes since the merge base with main (for CI)
    ownership_guard.py --hook claude    # Claude Code PostToolUse hook (reads JSON on stdin)

Exit codes: 0 = nothing found, 1 = findings (2 in --hook mode, so the agent sees them),
3 = usage or git error. Needs Python 3.8+ and git. Works on Windows, macOS, and Linux.
"""
import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

JUSTIFY = re.compile(r"//\s*ownership\s*:\s*\S")

# (category, pattern, message). Patterns run on code with comments and string literals removed.
RULES = [
    ("clone", re.compile(r"\.clone\(\)"),
     "new .clone(): borrow, move on last use, or share with Arc/Rc instead"),
    ("clone", re.compile(r"\.to_owned\(\)|\.to_vec\(\)"),
     "new owned copy: check whether a borrow (&str, &[T]) is enough"),
    ("smart-ptr", re.compile(
        r"\bRc\s*(?:<|::new\s*\()\s*(?:[\w:]+::)?RefCell\b"
        r"|\bArc\s*(?:<|::new\s*\()\s*(?:[\w:]+::)?(?:Mutex|RwLock)\b"),
     "new shared mutable state: check for a single owner, an arena with indices, or message passing"),
    ("lifetime", re.compile(r"'static\b"),
     "new 'static: make sure the data really lives for the whole program"),
    ("lifetime", re.compile(r"Box::leak\b|mem::forget\b"),
     "leaks memory to satisfy the borrow checker"),
    ("unsafe", re.compile(r"\bunsafe\b"),
     "new unsafe: never use it to bypass borrow rules; add a SAFETY comment if it is required"),
    ("box", re.compile(r"Box\s*<\s*(Vec|String|HashMap|BTreeMap|VecDeque)\b"),
     "boxed collection: the collection already owns heap storage"),
]

# Explicit reference-count increments are cheap and intentional.
ALLOWED = re.compile(r"\b(Rc|Arc)::clone\(")


def git(*args, cwd=None):
    result = subprocess.run(["git", *args], cwd=cwd, capture_output=True, text=True,
                            encoding="utf-8", errors="replace")
    if result.returncode != 0:
        raise RuntimeError(result.stderr.strip() or "git " + " ".join(args) + " failed")
    return result.stdout


def repo_root(start):
    return Path(git("rev-parse", "--show-toplevel", cwd=start).strip())


def strip_code(line):
    """Remove string/char literals and // comments so they do not trigger rules."""
    line = re.sub(r'r#*"(?:[^"\\]|\\.)*"#*', '""', line)
    line = re.sub(r'"(?:[^"\\]|\\.)*"', '""', line)
    line = re.sub(r"'(?:[^'\\]|\\.)'", "' '", line)
    return line.split("//", 1)[0]


def parse_diff(diff_text):
    """Yield (path, line_number, text) for every added line in a unified diff."""
    path = None
    line_no = 0
    for raw in diff_text.splitlines():
        if raw.startswith("+++ "):
            target = raw[4:].strip()
            path = None if target == "/dev/null" else re.sub(r"^b/", "", target)
            continue
        if raw.startswith("@@"):
            m = re.match(r"@@ -\d+(?:,\d+)? \+(\d+)(?:,\d+)? @@", raw)
            line_no = int(m.group(1)) if m else 0
            continue
        if path is None or raw.startswith("---"):
            continue
        if raw.startswith("+"):
            yield path, line_no, raw[1:]
            line_no += 1
        elif not raw.startswith("-") and not raw.startswith("\\"):
            line_no += 1


def untracked_rust_files(root, only=None):
    out = git("ls-files", "--others", "--exclude-standard", "--", "*.rs", cwd=root)
    files = [f for f in out.splitlines() if f]
    if only is not None:
        files = [f for f in files if f == only]
    for rel in files:
        text = (root / rel).read_text(encoding="utf-8", errors="replace")
        for i, line in enumerate(text.splitlines(), start=1):
            yield rel, i, line


def added_lines(root, mode, base=None, only=None):
    args = ["diff", "--no-color", "--no-ext-diff", "-U0"]
    if mode == "staged":
        args.append("--cached")
    elif mode == "base":
        args.append(git("merge-base", base, "HEAD", cwd=root).strip())
    else:
        args.append("HEAD")
    args += ["--", only if only else "*.rs"]
    yield from parse_diff(git(*args, cwd=root))
    if mode == "worktree":
        yield from untracked_rust_files(root, only)


def file_lines(root, rel):
    try:
        return (root / rel).read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        return []


def find_issues(root, lines):
    cache = {}
    issues = []
    for rel, no, text in lines:
        if not rel.endswith(".rs"):
            continue
        if JUSTIFY.search(text):
            continue
        if rel not in cache:
            cache[rel] = file_lines(root, rel)
        prev = cache[rel][no - 2] if no >= 2 and len(cache[rel]) >= no - 1 else ""
        if JUSTIFY.search(prev):
            continue
        code = ALLOWED.sub("", strip_code(text))
        for category, pattern, message in RULES:
            if pattern.search(code):
                issues.append((rel, no, category, message, text.strip()))
    return issues


def report(issues):
    lines = []
    for rel, no, category, message, text in issues:
        lines.append(f"{rel}:{no}: [{category}] {message}\n    {text}")
    lines.append(
        f"\n{len(issues)} new ownership workaround(s). For each one, either fix it or add a "
        "specific justification on the same line or the line above, e.g. "
        "`// ownership: snapshot must outlive the lock`."
    )
    return "\n".join(lines)


def run_hook(agent):
    if agent != "claude":
        print(f"unsupported hook agent: {agent}", file=sys.stderr)
        return 3
    try:
        payload = json.load(sys.stdin)
    except ValueError:
        return 0
    tool_input = payload.get("tool_input") or {}
    file_path = tool_input.get("file_path") or tool_input.get("path")
    if not file_path or not str(file_path).endswith(".rs"):
        return 0
    path = Path(file_path)
    cwd = Path(payload.get("cwd") or path.parent)
    if not path.is_absolute():
        path = cwd / path
    try:
        root = repo_root(path.parent)
        rel = path.resolve().relative_to(root.resolve()).as_posix()
        issues = find_issues(root, added_lines(root, "worktree", only=rel))
    except (RuntimeError, ValueError, OSError):
        return 0  # not a git repository or file outside it: stay silent
    if not issues:
        return 0
    print(report(issues), file=sys.stderr)
    return 2


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    group = parser.add_mutually_exclusive_group()
    group.add_argument("--staged", action="store_true", help="check staged changes")
    group.add_argument("--base", metavar="REF", help="check changes since the merge base with REF")
    group.add_argument("--hook", metavar="AGENT", help="run as an agent hook (supported: claude)")
    parser.add_argument("-C", dest="directory", default=".", help="run as if started in this directory")
    args = parser.parse_args()

    if args.hook:
        return run_hook(args.hook)

    mode = "staged" if args.staged else "base" if args.base else "worktree"
    try:
        root = repo_root(Path(args.directory))
        issues = find_issues(root, added_lines(root, mode, base=args.base))
    except RuntimeError as err:
        print(f"ownership_guard: {err}", file=sys.stderr)
        return 3
    if not issues:
        print("ownership_guard: no new ownership workarounds")
        return 0
    print(report(issues))
    return 1


if __name__ == "__main__":
    sys.exit(main())
