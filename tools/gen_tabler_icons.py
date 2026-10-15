#!/usr/bin/env python3
"""Regenerate `src/icons/table.rs` from the Tabler icons webfont stylesheet.

Usage:
    python3 tools/gen_tabler_icons.py [version]

Downloads the given `@tabler/icons-webfont` version (default pinned below) from
jsDelivr, parses the `.ti-*:before { content: "\\xxxx" }` rules and writes the
name -> codepoint table consumed by `src/icons.rs`.

The matching font file lives at `assets/fonts/tabler-icons.ttf`; download it with:
    curl -L -o assets/fonts/tabler-icons.ttf \\
      https://cdn.jsdelivr.net/npm/@tabler/icons-webfont@<version>/dist/fonts/tabler-icons.ttf
"""

import re
import sys
import urllib.request
from pathlib import Path

VERSION = "3.49.0"
ROOT = Path(__file__).resolve().parent.parent

RULE = re.compile(r'\.ti-([a-z0-9\-]+):before\s*\{\s*content:\s*"\\([0-9a-fA-F]+)";')


def main() -> int:
    version = sys.argv[1] if len(sys.argv) > 1 else VERSION
    url = f"https://cdn.jsdelivr.net/npm/@tabler/icons-webfont@{version}/dist/tabler-icons.css"
    print(f"downloading {url}")
    css = urllib.request.urlopen(url).read().decode("utf-8")

    seen: dict[str, int] = {}
    for name, code in RULE.findall(css):
        seen.setdefault(name, int(code, 16))

    items = sorted(seen.items())
    out = [
        f"//! Generated from `@tabler/icons-webfont` {version}. Do not edit by hand.",
        "//!",
        "//! Maps every Tabler icon name to the Private Use Area codepoint used by",
        "//! `assets/fonts/tabler-icons.ttf`. Regenerate with `tools/gen_tabler_icons.py`.",
        "",
        "/// Sorted list of `(icon name, codepoint)`, ready for binary search.",
        "pub static TABLER_GLYPHS: &[(&str, char)] = &[",
    ]
    out += [f'    ("{name}", \'\\u{{{code:x}}}\'),' for name, code in items]
    out.append("];")
    out.append("")

    dest = ROOT / "src" / "icons" / "table.rs"
    dest.write_text("\n".join(out))
    print(f"wrote {len(items)} icons to {dest.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
