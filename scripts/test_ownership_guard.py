"""Tests for ownership_guard.py. Run: python scripts/test_ownership_guard.py"""
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

GUARD = Path(__file__).resolve().parent / "ownership_guard.py"

BASE = """fn main() {
    let names = vec![String::from("a")];
    let first = names[0].clone();
    println!("{first}");
}
"""


def git(cwd, *args):
    subprocess.run(["git", "-c", "user.name=test", "-c", "user.email=test@example.com", *args],
                   cwd=cwd, check=True, capture_output=True)


def guard(cwd, *args, stdin=None):
    return subprocess.run([sys.executable, str(GUARD), *args], cwd=cwd, input=stdin,
                          capture_output=True, text=True)


class GuardTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        git(self.root, "init", "-q", "-b", "main")
        (self.root / "src").mkdir()
        (self.root / "src" / "main.rs").write_text(BASE, encoding="utf-8")
        git(self.root, "add", ".")
        git(self.root, "commit", "-q", "-m", "init")

    def tearDown(self):
        self.tmp.cleanup()

    def edit(self, text, name="src/main.rs"):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
        return path

    def test_existing_code_is_ignored(self):
        result = guard(self.root)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_new_clone_is_flagged(self):
        self.edit(BASE.replace('println!("{first}");', 'let copy = names.clone();\n    println!("{first} {copy:?}");'))
        result = guard(self.root)
        self.assertEqual(result.returncode, 1)
        self.assertIn("src/main.rs:4: [clone]", result.stdout)

    def test_justified_same_line_and_line_above(self):
        self.edit(BASE.replace(
            'println!("{first}");',
            'let a = names.clone(); // ownership: a must outlive names\n'
            '    // ownership: b is sent to another thread\n'
            '    let b = names.clone();\n'
            '    println!("{first} {a:?} {b:?}");'))
        result = guard(self.root)
        self.assertEqual(result.returncode, 0, result.stdout)

    def test_comments_strings_and_rc_clone_are_ignored(self):
        self.edit(BASE.replace(
            'println!("{first}");',
            '// we used to call .clone() here\n'
            '    let s = "x.clone() and unsafe and \'static";\n'
            '    let rc = std::rc::Rc::new(1);\n'
            '    let rc2 = std::rc::Rc::clone(&rc);\n'
            '    println!("{first} {s} {rc2}");'))
        result = guard(self.root)
        self.assertEqual(result.returncode, 0, result.stdout)

    def test_each_category(self):
        self.edit(BASE + """
fn more() {
    let v: Box<Vec<u8>> = Box::new(vec![]);
    let leaked: &'static str = Box::leak(String::new().into_boxed_str());
    let shared = std::rc::Rc::new(std::cell::RefCell::new(0));
    let t: std::rc::Rc<std::cell::RefCell<u8>> = shared;
    unsafe { std::hint::unreachable_unchecked() }
}
""")
        out = guard(self.root).stdout
        for category in ("[box]", "[lifetime]", "[smart-ptr]", "[unsafe]"):
            self.assertIn(category, out)

    def test_untracked_file(self):
        self.edit("fn f(v: &Vec<u8>) -> Vec<u8> { v.to_vec() }\n", name="src/new.rs")
        result = guard(self.root)
        self.assertEqual(result.returncode, 1)
        self.assertIn("src/new.rs:1: [clone]", result.stdout)

    def test_staged_mode(self):
        self.edit(BASE.replace("let first", "let all = names.clone();\n    let first"))
        self.assertEqual(guard(self.root, "--staged").returncode, 0)
        git(self.root, "add", ".")
        self.assertEqual(guard(self.root, "--staged").returncode, 1)

    def test_base_mode(self):
        git(self.root, "checkout", "-q", "-b", "feature")
        self.edit(BASE.replace("let first", "let all = names.clone();\n    let first"))
        git(self.root, "commit", "-q", "-am", "feature")
        result = guard(self.root, "--base", "main")
        self.assertEqual(result.returncode, 1)
        self.assertIn("[clone]", result.stdout)

    def test_claude_hook(self):
        path = self.edit(BASE.replace("let first", "let all = names.clone();\n    let first"))
        payload = json.dumps({"cwd": str(self.root), "tool_name": "Edit",
                              "tool_input": {"file_path": str(path)}})
        result = guard(self.root, "--hook", "claude", stdin=payload)
        self.assertEqual(result.returncode, 2)
        self.assertIn("[clone]", result.stderr)

    def test_claude_hook_ignores_other_files(self):
        payload = json.dumps({"cwd": str(self.root), "tool_input": {"file_path": str(self.root / "README.md")}})
        self.assertEqual(guard(self.root, "--hook", "claude", stdin=payload).returncode, 0)

    def test_not_a_repository(self):
        with tempfile.TemporaryDirectory() as other:
            self.assertEqual(guard(other).returncode, 3)


if __name__ == "__main__":
    unittest.main(verbosity=2)
