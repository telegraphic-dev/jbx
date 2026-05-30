#!/usr/bin/env python3
"""Build bundled jbx skills from the website command Markdown.

The website command pages are the curated source of truth. `jbx skill get`
returns a skill-shaped projection of those pages so command docs and bundled
agent guidance cannot drift.
"""
from __future__ import annotations

from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
COMMAND_DIR = ROOT / "website/content/pages/docs/commands"
SKILL_DATA_DIR = ROOT / "skill-data"
INSTALLABLE_SKILLS_DIR = ROOT / "skills"

ORDER = [
    "top-level", "run", "build", "check", "test", "docs", "doctor", "rewrite",
    "search", "resolve", "fetch", "info", "cache", "trust", "app", "alias",
    "catalog", "template", "init", "export", "publish", "install", "fmt", "graph",
    "skill", "jdk",
]


def parse_page(path: Path) -> tuple[dict[str, str], str]:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        raise SystemExit(f"{path}: missing frontmatter")
    _, frontmatter, body = text.split("---", 2)
    meta: dict[str, str] = {}
    for line in frontmatter.strip().splitlines():
        if ":" in line:
            key, value = line.split(":", 1)
            meta[key.strip()] = value.strip()
    return meta, body.lstrip()


def strip_section(body: str, heading: str) -> str:
    pattern = re.compile(rf"(?ms)^## {re.escape(heading)}\n.*?(?=^## |\Z)")
    return pattern.sub("", body)


def strip_skill_bootstrap(body: str) -> str:
    lines: list[str] = []
    skip_empty_after_removed = False
    for line in body.splitlines():
        if "jbx skill get" in line:
            skip_empty_after_removed = True
            continue
        if skip_empty_after_removed and not line.strip():
            skip_empty_after_removed = False
            continue
        skip_empty_after_removed = False
        # Renumber simple ordered-list gaps created by removing discovery steps.
        line = re.sub(r"^2\. Run the command", "1. Run the command", line)
        line = re.sub(r"^3\. Prefer JSON", "2. Prefer JSON", line)
        line = re.sub(r"^4\. Verify", "3. Verify", line)
        lines.append(line)
    text = "\n".join(lines)
    text = re.sub(r"\n{3,}", "\n\n", text)
    return text.strip() + "\n"


def skill_name(page_stem: str) -> str:
    return "jbx" if page_stem == "top-level" else f"jbx-{page_stem}"


def skill_markdown(page_stem: str, installable: bool) -> str:
    page = COMMAND_DIR / f"{page_stem}.md"
    meta, body = parse_page(page)
    name = skill_name(page_stem)
    description = meta.get("description", "jbx command guidance")
    body = strip_section(body, "Skill")
    body = strip_skill_bootstrap(body)
    suffix = "\n> For exact behavior, prefer the skill bundled with the `jbx` binary on the machine running the task.\n" if installable else ""
    return f"---\nname: {name}\ndescription: {description}\n---\n\n{body}{suffix}".rstrip() + "\n"


def write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def generate() -> None:
    for page_stem in ORDER:
        page = COMMAND_DIR / f"{page_stem}.md"
        if not page.exists():
            raise SystemExit(f"Missing command page: {page}")
        name = skill_name(page_stem)
        write(SKILL_DATA_DIR / name / "SKILL.md", skill_markdown(page_stem, installable=False))
        write(INSTALLABLE_SKILLS_DIR / name / "SKILL.md", skill_markdown(page_stem, installable=True))


if __name__ == "__main__":
    generate()
