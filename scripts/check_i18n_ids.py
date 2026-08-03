#!/usr/bin/env python3
"""Assert en and pt-BR Fluent files share the same message ID set."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ID_RE = re.compile(r"^([a-zA-Z][a-zA-Z0-9_-]*)\s*=", re.MULTILINE)
LOCALES = ("en", "pt-BR")
FILES = ("main.ftl", "settings.ftl", "errors.ftl", "tray.ftl")


def ids_in(path: Path) -> set[str]:
    return set(ID_RE.findall(path.read_text(encoding="utf-8")))


def main() -> int:
    root = (
        Path(__file__).resolve().parents[1]
        / "crates"
        / "fileorz-i18n"
        / "locales"
    )
    failed = 0
    for name in FILES:
        sets = {loc: ids_in(root / loc / name) for loc in LOCALES}
        if not sets["en"]:
            print(f"FAIL {name}: no IDs in en", file=sys.stderr)
            failed += 1
            continue
        only_en = sets["en"] - sets["pt-BR"]
        only_pt = sets["pt-BR"] - sets["en"]
        if only_en or only_pt:
            failed += 1
            print(f"FAIL {name}: ID mismatch", file=sys.stderr)
            if only_en:
                print(f"  only en: {sorted(only_en)}", file=sys.stderr)
            if only_pt:
                print(f"  only pt-BR: {sorted(only_pt)}", file=sys.stderr)
        else:
            print(f"OK  {name} ({len(sets['en'])} IDs)")
    if failed:
        return 1
    print("All Fluent locale ID sets match")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
