"""Grade the multi-file feature evals in evals/projects/.

Usage:
    python evals/grade_projects.py evals/runs/<run-folder>

The run folder holds one subfolder per project, with one git patch per attempt
(for example `game_tick/skill-1.patch`). Each patch is applied to a fresh copy of
the project, which is then built and run. The grader reports:

- whether the feature works (output matches the reference),
- how many ownership workarounds the change added, counted by
  scripts/ownership_guard.py: all of them, and those without an
  `// ownership:` justification.
"""
import collections
import importlib.util
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

EVALS = Path(__file__).resolve().parent
ROOT = EVALS.parent

spec = importlib.util.spec_from_file_location("ownership_guard", ROOT / "scripts" / "ownership_guard.py")
guard = importlib.util.module_from_spec(spec)
spec.loader.exec_module(guard)

EXPECTED = {
    "game_tick": """Ada hp=100/100 heals=0
Bo hp=37/80 heals=1
Cy hp=0/90 heals=0
Di hp=45/120 heals=1
[turn 1] Bo is low on health
[turn 1] Cy is down
[turn 1] Di is low on health
[turn 2] Cy is down
[turn 3] Cy is down""",
    "doc_summaries": """0: Ownership: Each value has one owner.
1: Borrowing: References borrow a value without taking ownership.
0: Ownership: Each value has one owner.
2: Lifetimes: Lifetimes describe how long references are valid
1: Borrowing: References borrow a value without taking ownership.
0: Ownership: Each value has one owner.
computed 3 summaries""",
    "parallel_orders": "2000 orders, first Some(119), last Some(36666), sum 21736240",
}

# Extra requirement beyond the output: the feature must actually be implemented.
REQUIRED = {
    "parallel_orders": ("src/pipeline.rs", "thread"),
}


def run(cmd, cwd):
    return subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, encoding="utf-8",
                          errors="replace", timeout=600)


def git(cwd, *args):
    res = run(["git", "-c", "user.name=eval", "-c", "user.email=eval@example.com", *args], cwd)
    if res.returncode != 0:
        raise RuntimeError(res.stderr)
    return res.stdout


def count(root, justify):
    saved = guard.JUSTIFY
    if not justify:
        guard.JUSTIFY = __import__("re").compile(r"(?!x)x")  # never matches
    try:
        issues = guard.find_issues(root, guard.added_lines(root, "worktree"))
    finally:
        guard.JUSTIFY = saved
    return issues


def grade(project, patch, tmp):
    work = tmp / f"{project}-{patch.stem}"
    shutil.copytree(EVALS / "projects" / project, work)
    git(work, "init", "-q", "-b", "main")
    git(work, "add", ".")
    git(work, "commit", "-q", "-m", "start")
    if patch.read_text(encoding="utf-8").strip():
        git(work, "apply", "--whitespace=nowarn", str(patch.resolve()))
    out = run(["cargo", "run", "--quiet", "--release"], work)
    works = out.returncode == 0 and out.stdout.strip() == EXPECTED[project]
    if works and project in REQUIRED:
        rel, needle = REQUIRED[project]
        works = needle in (work / rel).read_text(encoding="utf-8")
    raw = count(work, justify=False)
    open_issues = count(work, justify=True)
    return works, raw, open_issues


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    runs = Path(sys.argv[1])
    rows = []
    with tempfile.TemporaryDirectory() as tmp:
        for patch in sorted(runs.glob("*/*.patch")):
            project = patch.parent.name
            works, raw, open_issues = grade(project, patch, Path(tmp))
            rows.append((project, patch.stem, works, raw, open_issues))

    print(f"{'project':16} {'run':10} works  added  unjustified  findings")
    for project, name, works, raw, open_issues in rows:
        kinds = ",".join(sorted({i[2] for i in raw}))
        print(f"{project:16} {name:10} {str(works):6} {len(raw):5}  {len(open_issues):11}  {kinds}")

    totals = collections.defaultdict(lambda: [0, 0, 0, 0])
    for project, name, works, raw, open_issues in rows:
        t = totals[name.rsplit("-", 1)[0]]
        t[0] += works
        t[1] += 1
        t[2] += len(raw)
        t[3] += len(open_issues)
    print()
    for cond, (ok, n, raw, open_issues) in sorted(totals.items()):
        print(f"{cond}: feature works {ok}/{n}, workarounds added {raw}, unjustified {open_issues}")


if __name__ == "__main__":
    main()
