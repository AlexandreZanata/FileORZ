"""Advanced PDF keyword replay — AdvancedAlg/Alg.py semantics."""

from __future__ import annotations

import os
import shutil
from pathlib import Path

import pypdf


def last_page_text(pdf_path: Path) -> str:
    """Upstream overwrites per page — effective haystack is last page."""
    text = ""
    with pypdf.PdfReader(str(pdf_path)) as reader:
        for page in reader.pages:
            extracted = page.extract_text() or ""
            text = extracted.upper()
    return text


def list_root_pdfs(root: Path) -> list[str]:
    return [n for n in os.listdir(root) if n.endswith(".pdf") and (root / n).is_file()]


def try_move_pdf(
    root: Path, name: str, group: str, actions: list[dict]
) -> bool:
    dest_dir = root / group
    dest_dir.mkdir(parents=True, exist_ok=True)
    dest = dest_dir / name
    rel = dest.relative_to(root).as_posix()
    if dest.exists():
        actions.append(
            {
                "action": "skip",
                "from": name,
                "to": rel,
                "reason": "destination_exists",
            }
        )
        return False
    shutil.move(str(root / name), str(dest))
    actions.append({"action": "move", "from": name, "to": rel})
    return True


def process_pdfs(root: Path, keywords: dict) -> list[dict]:
    actions: list[dict] = []
    for name in sorted(list_root_pdfs(root)):
        haystack = last_page_text(root / name)
        moved = False
        for group, phrases in keywords.items():
            if moved:
                break
            for phrase in phrases:
                if phrase.upper() not in haystack:
                    continue
                moved = try_move_pdf(root, name, group, actions)
                break
    return actions
