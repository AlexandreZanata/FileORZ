#!/usr/bin/env python3
"""Ensure utils/*.py config key literals appear in docs/CONFIG-KEY-MAP.md."""

from __future__ import annotations

import re
from pathlib import Path

# Known config key literals used in utils (static inventory + regex harvest).
KEY_RE = re.compile(
    r"""(?:CONFIG|config|data|cfg|self\.CONFIG)\s*(?:\[['\"]([^'\"]+)['\"]"""
    r"""|\.get\(\s*['\"]([^'\"]+)['\"])"""
)

KNOWN = {
    "Folder",
    "timeverification",
    "Startup",
    "AutoDelete",
    "Enviar Para Lixeira",
    "Excluir permanentemente",
    "AdvancedOrganize",
    "AutoDeleteConfig",
    "folder_delete",
    "Por Data de Criação",
    "Por Data de Modificação",
    "Dias para Auto Deletar",
    "ativado",
    "lixeira",
    "excluir_permanentemente",
    "pastas_ORZ",
    "tudo",
}


def harvest(utils: Path) -> set[str]:
    found: set[str] = set()
    for path in utils.glob("*.py"):
        text = path.read_text(encoding="utf-8")
        for match in KEY_RE.finditer(text):
            key = match.group(1) or match.group(2)
            if key:
                found.add(key)
    return found


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    doc = (root / "docs" / "CONFIG-KEY-MAP.md").read_text(encoding="utf-8")
    found = harvest(root / "utils") | KNOWN
    # Dynamic category / ext keys are not required in the static map.
    skip_prefixes = ()
    missing = sorted(
        k
        for k in found
        if k not in doc and not any(k.startswith(p) for p in skip_prefixes)
    )
    # Ignore obvious non-config attribute noise if any slipped in.
    noise = {k for k in missing if k in {"name", "value", "category"}}
    missing = [k for k in missing if k not in noise]
    if missing:
        print("FAIL missing from CONFIG-KEY-MAP.md:", ", ".join(missing))
        return 1
    print(f"OK  CONFIG-KEY-MAP covers {len(found)} utils key literal(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
