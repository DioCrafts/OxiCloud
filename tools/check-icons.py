#!/usr/bin/env python3
"""
check-icons.py — Audit FA icon usage against the inline SVG registry.

Usage:
    python3 tools/check-icons.py [--dry-run]

What it does:
  1. Scans every file under static/ for  fas fa-<name>  occurrences.
  2. Reads the _ICONS registry from static/js/core/icons.js.
  3. For each icon name that is missing from the registry, looks up
       tmp/Font-Awesome/svgs/solid/<name>.svg
     and adds the entry  '<name>': [<width>, '<path>']  to _ICONS.
  4. Rewrites icons.js in-place (unless --dry-run is given).

FontAwesome source:  tmp/Font-Awesome/svgs/solid/
Icons registry:      static/js/core/icons.js
"""

import re
import subprocess
import sys
import os
from pathlib import Path

# ── Paths ──────────────────────────────────────────────────────────────────────
REPO_ROOT   = Path(__file__).resolve().parent.parent
STATIC_DIR  = REPO_ROOT / "static"
ICONS_JS    = STATIC_DIR / "js" / "core" / "icons.js"
FA_SVG_DIR  = REPO_ROOT / "tmp" / "Font-Awesome" / "svgs" / "solid"

DRY_RUN = "--dry-run" in sys.argv

# ── 0. Ensure Font-Awesome source is available ────────────────────────────────
TMP_DIR = REPO_ROOT / "tmp"
if not TMP_DIR.exists():
    print(f"Creating {TMP_DIR.relative_to(REPO_ROOT)}/")
    TMP_DIR.mkdir(parents=True, exist_ok=True)

FA_REPO = TMP_DIR / "Font-Awesome"
if not FA_REPO.exists():
    print(f"Font-Awesome not found at {FA_REPO.relative_to(REPO_ROOT)} — cloning …")
    result = subprocess.run(
        ["git", "clone", "https://github.com/FortAwesome/Font-Awesome.git", str(FA_REPO)],
        check=False,
    )
    if result.returncode != 0:
        print("✗ git clone failed — cannot continue without Font-Awesome source.")
        sys.exit(1)
    print("✓ Font-Awesome cloned successfully.\n")

# ── 1. Scan static/ for all  fas fa-<name>  occurrences ───────────────────────
FA_RE = re.compile(r'\bfas fa-([\w-]+)')

used_icons: dict[str, list[str]] = {}   # name → [file, …]

SKIP_DIRS = {".git", "node_modules"}

for path in sorted(STATIC_DIR.rglob("*")):
    if any(part in SKIP_DIRS for part in path.parts):
        continue
    if not path.is_file():
        continue
    # Only scan text files we care about
    if path.suffix not in {".html", ".js", ".css"}:
        continue
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        continue
    for m in FA_RE.finditer(text):
        name = m.group(1)
        rel  = str(path.relative_to(REPO_ROOT))
        used_icons.setdefault(name, []).append(rel)

print(f"Found {len(used_icons)} distinct FA icon(s) referenced in static/")

# ── 2. Parse existing _ICONS keys from icons.js ───────────────────────────────
icons_src = ICONS_JS.read_text(encoding="utf-8")

# Extract only the _ICONS object body so we never accidentally match keys from
# other objects or functions elsewhere in the file.
_ICONS_BLOCK_RE = re.compile(r'const _ICONS\s*=\s*\{(.*?)^};', re.DOTALL | re.MULTILINE)
block_match = _ICONS_BLOCK_RE.search(icons_src)
if not block_match:
    print("✗ Could not locate 'const _ICONS = { … };' in icons.js — aborting.")
    sys.exit(1)
icons_block = block_match.group(1)

# Now parse keys only within that block.
# Keys may be quoted ('bars', "bars") or bare (bars) — make the quotes optional.
# Hyphenated names like 'arrow-left' must be quoted in JS; bare keys are word-only.
ICON_KEY_RE = re.compile(r"""['"]?([\w-]+)['"]?\s*:\s*\[""")
registered: set[str] = {m.group(1) for m in ICON_KEY_RE.finditer(icons_block)}

print(f"Registry has {len(registered)} icon(s) in _ICONS")

# ── 3. Find missing icons ─────────────────────────────────────────────────────
missing = {name: files for name, files in used_icons.items() if name not in registered}

if not missing:
    print("✓ All used icons are present in the registry — nothing to do.")
    sys.exit(0)

print(f"\n{len(missing)} missing icon(s):")

# ── 4. Resolve each missing icon from FA SVG files ────────────────────────────
VIEWBOX_RE = re.compile(r'viewBox="0 0 (\d+) (\d+)"')
PATH_D_RE  = re.compile(r'<path[^>]+\bd="([^"]+)"')

new_entries: list[tuple[str, int, str]] = []   # (name, width, d)
not_found:   list[str]                  = []

for name, files in sorted(missing.items()):
    svg_path = FA_SVG_DIR / f"{name}.svg"
    print(f"  • {name:30s}  used in: {', '.join(files)}", end="")

    if not svg_path.exists():
        print(f"  ✗  SVG not found: {svg_path.relative_to(REPO_ROOT)}")
        not_found.append(name)
        continue

    svg_text = svg_path.read_text(encoding="utf-8")

    vb = VIEWBOX_RE.search(svg_text)
    pd = PATH_D_RE.search(svg_text)

    if not vb or not pd:
        print(f"  ✗  Could not parse SVG (viewBox={bool(vb)}, path={bool(pd)})")
        not_found.append(name)
        continue

    width = int(vb.group(1))
    d     = pd.group(1)
    print(f"  ✓  viewBox=0 0 {width} 512")
    new_entries.append((name, width, d))

# ── 5. Patch icons.js ─────────────────────────────────────────────────────────
if not new_entries:
    if not_found:
        print(f"\n✗ {len(not_found)} icon(s) could not be resolved — no changes written.")
    sys.exit(1 if not_found else 0)

# Build the text block to insert, sorted alphabetically for readability
new_entries.sort(key=lambda x: x[0])

insert_lines = []
for name, width, d in new_entries:
    insert_lines.append(f"    '{name}': [\n        {width},\n        '{d}'\n    ],")

insert_block = "\n".join(insert_lines) + "\n"

# Insert just before the closing "};" of _ICONS (line 396 area)
# Anchor: the line that is exactly "};"
ICONS_END_RE = re.compile(r'^};$', re.MULTILINE)
m = ICONS_END_RE.search(icons_src)
if not m:
    print("\n✗ Could not locate the closing '}; ' of _ICONS in icons.js — aborting.")
    sys.exit(1)

# Ensure the last existing entry has a trailing comma before we append.
before = icons_src[: m.start()]
if before.rstrip()[-1:] != ',':
    # Insert comma right after the last non-whitespace character
    rstripped = before.rstrip()
    trailing_ws = before[len(rstripped):]
    before = rstripped + ',\n' + trailing_ws

new_src = before + insert_block + icons_src[m.start() :]

if DRY_RUN:
    print(f"\n-- DRY RUN: would insert into icons.js --\n{insert_block}")
else:
    ICONS_JS.write_text(new_src, encoding="utf-8")
    print(f"\n✓ Added {len(new_entries)} icon(s) to {ICONS_JS.relative_to(REPO_ROOT)}")

if not_found:
    print(f"\n⚠ {len(not_found)} icon(s) still missing (no SVG source found):")
    for n in not_found:
        print(f"    - {n}")
    sys.exit(1)
