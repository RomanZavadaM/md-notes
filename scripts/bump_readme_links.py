#!/usr/bin/env python3
"""Sets the version in the download blocks of every README.

The block sits between `<!-- x-release-please-start-version -->` and
`<!-- x-release-please-end -->`. release-please's generic updater replaces only
the first version on a line, and our download links carry two (tag and file
name), so the release workflow runs this script after the assets exist.

Usage: bump_readme_links.py 0.3.5
"""
import re
import sys
from pathlib import Path

START = "<!-- x-release-please-start-version -->"
END = "<!-- x-release-please-end -->"
SEMVER = re.compile(r"\d+\.\d+\.\d+")


def bump(text: str, version: str) -> str:
    out = []
    inside = False
    for line in text.split("\n"):
        if START in line:
            inside = True
        elif END in line:
            inside = False
        elif inside:
            line = SEMVER.sub(version, line)
        out.append(line)
    return "\n".join(out)


def main() -> int:
    if len(sys.argv) != 2 or not SEMVER.fullmatch(sys.argv[1].lstrip("v")):
        print(__doc__)
        return 2
    version = sys.argv[1].lstrip("v")
    root = Path(__file__).resolve().parents[1]
    files = [root / "README.md", *sorted((root / "docs" / "readme").glob("README.*.md"))]
    for path in files:
        raw = path.read_bytes().decode("utf-8")
        newline = "\r\n" if "\r\n" in raw else "\n"
        text = raw.replace("\r\n", "\n")
        if START not in text or END not in text:
            print(f"{path}: no version block", file=sys.stderr)
            return 1
        updated = bump(text, version)
        if updated != text:
            path.write_bytes(updated.replace("\n", newline).encode("utf-8"))
            print(f"updated {path.relative_to(root)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
