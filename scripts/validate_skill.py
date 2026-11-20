"""Validate SKILL.md frontmatter against the Agent Skills format. Exit code 1 on failure."""
import re
import sys
from pathlib import Path

root = Path(__file__).resolve().parent.parent
text = (root / "SKILL.md").read_text(encoding="utf-8")
errors = []

match = re.match(r"^---\r?\n(.*?)\r?\n---\r?\n", text, re.S)
if not match:
    sys.exit("SKILL.md: missing YAML frontmatter")
fields = dict(
    line.split(":", 1) for line in match.group(1).splitlines() if ":" in line and not line.startswith(" ")
)
fields = {k.strip(): v.strip() for k, v in fields.items()}

name = fields.get("name", "")
desc = fields.get("description", "")
if name != root.name and name != "rust-ownership-optimizer":
    errors.append(f"name '{name}' does not match the skill folder")
if not re.fullmatch(r"[a-z0-9]+(-[a-z0-9]+)*", name) or len(name) > 64:
    errors.append("name must be lowercase letters, digits, and hyphens, max 64 chars")
if not desc:
    errors.append("description is missing")
if len(desc) > 1024:
    errors.append(f"description is {len(desc)} chars, max 1024")
if len(text.splitlines()) > 500:
    errors.append("SKILL.md is over 500 lines; move detail into references/")
if re.search(r"(?i)garbage.collect(ed|s|ing)?\b(?! *or)", text) and "no garbage collector" not in text:
    errors.append("SKILL.md describes Rust with garbage-collection terms")

for ref in re.findall(r"`((?:references|examples|scripts)/[^`]+)`", text):
    if not (root / ref).exists():
        errors.append(f"referenced file does not exist: {ref}")

if errors:
    print("\n".join(f"error: {e}" for e in errors))
    sys.exit(1)
print(f"SKILL.md ok: name={name}, description={len(desc)} chars")
