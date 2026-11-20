"""Grade agent outputs for the eval scenarios.

Usage:
    python evals/grade.py evals/runs/<run-folder>

The run folder holds one subfolder per scenario, with one .rs file per attempt
(for example `iterate_and_mutate/skill-1.rs`). File names starting with `base`
and `skill` are counted as runs without and with the skill.

Each file is compiled with rustc, run, and compared against the expected output.
Then scenario-specific checks look for the workaround the scenario is about.
Works on Windows, macOS, and Linux.
"""
import collections
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path

EVALS = Path(__file__).resolve().parent

EXPECTED = {
    "e0382_moved_value": "Q3: 2 lines\narchived 2",
    "iterate_and_mutate": '["x joined", "y joined"]',
    "lifetime_static": "ownership",
}


def norm(out: str) -> str:
    # HashMap Debug output has random order: sort entries inside {...}.
    def sort_map(m):
        items = [s.strip() for s in m.group(1).split(",") if s.strip()]
        return "{" + ", ".join(sorted(items)) + "}"
    return re.sub(r"\{([^{}]*)\}", sort_map, out.strip().replace("\r\n", "\n"))


def strip_comments(src: str) -> str:
    src = re.sub(r"/\*.*?\*/", "", src, flags=re.S)
    return re.sub(r"//[^\n]*", "", src)


def compile_and_run(source: Path, workdir: Path):
    exe = workdir / (source.parent.name + "_" + source.stem + (".exe" if sys.platform == "win32" else ""))
    build = subprocess.run(["rustc", "--edition", "2021", "-O", str(source), "-o", str(exe)],
                           capture_output=True, text=True)
    if build.returncode != 0:
        return False, ""
    run = subprocess.run([str(exe)], capture_output=True, text=True, timeout=120)
    return run.returncode == 0, run.stdout


def check(name, code):
    c = strip_comments(code)
    checks = {}
    if name == "e0382_moved_value":
        checks["borrows_param"] = bool(re.search(r"fn\s+print_report\s*\(\s*\w+\s*:\s*&", c))
        checks["no_clone"] = ".clone()" not in c
    elif name == "clone_heavy":
        sig = re.search(r"fn\s+find_user\s*\(([^)]*)\)", c)
        checks["find_user_borrows"] = bool(sig) and "Vec<String>" not in sig.group(1).replace("&Vec", "") and not re.search(r":\s*String\b", sig.group(1))
        tl = re.search(r"fn\s+total_len\s*\(([^)]*)\)", c)
        checks["total_len_slice"] = bool(tl) and "Vec<" not in tl.group(1)
        checks["uses_entry"] = ".entry(" in c
        checks["clones_le_1"] = c.count(".clone()") <= 1
    elif name == "boxed_everything":
        checks["no_box_vec"] = not re.search(r"Box\s*<\s*Vec", c)
        checks["no_boxed_local"] = not re.search(r"Box::new\(\s*\d", c)
        run_sig = re.search(r"fn\s+run\s*(<[^>]*>)?\s*\(([^)]*)\)", c)
        checks["static_dispatch"] = bool(run_sig) and "dyn" not in run_sig.group(2)
    elif name == "iterate_and_mutate":
        checks["no_vec_clone"] = not re.search(r"players\s*\.\s*clone\(\)", c) and "players.to_vec()" not in c
        checks["no_refcell_unsafe"] = "RefCell" not in c and "unsafe" not in c
    elif name == "thread_clone":
        checks["no_data_clone"] = not re.search(r"data\s*\.\s*(clone|to_vec)\(\)", c)
        checks["scope_or_arc"] = "thread::scope" in c or "scope(" in c or "Arc" in c
    elif name == "lifetime_static":
        checks["no_static"] = "'static" not in c
        checks["no_leak"] = "leak" not in c
        checks["returns_str_ref"] = bool(re.search(r"fn\s+longest_word[^{]*->\s*&(\s*'\w+)?\s*str", c))
        checks["no_string_vec"] = "Vec<String>" not in c
    return checks


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    runs = Path(sys.argv[1])
    rows = []
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        expected = dict(EXPECTED)
        for f in sorted((EVALS / "files").glob("*.rs")):
            if f.stem not in expected:
                ok, out = compile_and_run(f, tmp)
                if not ok:
                    sys.exit(f"original scenario {f.name} failed to run")
                expected[f.stem] = out

        for f in sorted(runs.glob("*/*.rs")):
            name = f.parent.name
            code = f.read_text(encoding="utf-8")
            compiles, out = compile_and_run(f, tmp)
            checks = check(name, code)
            rows.append({
                "scenario": name,
                "run": f.stem,
                "condition": f.stem.split("-")[0],
                "compiles": compiles,
                "same_output": compiles and norm(out) == norm(expected[name]),
                "clones": strip_comments(code).count(".clone()"),
                "checks": checks,
            })
            rows[-1]["pass"] = rows[-1]["same_output"] and all(checks.values())

    print(f"{'scenario':22} {'run':8} pass   failed checks")
    for r in rows:
        failed = ",".join(k for k, v in r["checks"].items() if not v)
        print(f"{r['scenario']:22} {r['run']:8} {str(r['pass']):6} {failed}")

    totals = collections.defaultdict(lambda: [0, 0])
    for r in rows:
        totals[r["condition"]][0] += r["pass"]
        totals[r["condition"]][1] += 1
    print()
    for cond, (passed, n) in sorted(totals.items()):
        print(f"{cond}: {passed}/{n} passed")


if __name__ == "__main__":
    main()
