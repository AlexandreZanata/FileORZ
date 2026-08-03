"""Extension organize replay — FileORZ.organize_files extension stage."""

from __future__ import annotations

import os
from pathlib import Path

from . import IGNORED_CONFIG_KEYS


def extension_map(config: dict) -> dict[str, str]:
    mapping: dict[str, str] = {}
    for category, exts in config.items():
        if category in IGNORED_CONFIG_KEYS or not isinstance(exts, dict):
            continue
        cat_name = category.capitalize()
        for ext, enabled in exts.items():
            if not enabled:
                continue
            clean = ext.lower().strip()
            if not clean.startswith("."):
                clean = "." + clean
            mapping[clean] = cat_name
    return mapping


def unique_destination(folder: Path, filename: str, ext: str) -> Path:
    dest = folder / f"{filename}{ext}"
    counter = 1
    while dest.exists():
        dest = folder / f"{filename}_{counter}{ext}"
        counter += 1
    return dest


def organize_extensions(root: Path, config: dict) -> list[dict]:
    mapping = extension_map(config)
    actions: list[dict] = []
    with os.scandir(root) as entries:
        for entry in entries:
            if not entry.is_file() or entry.name.startswith("."):
                continue
            stem, ext = os.path.splitext(entry.name)
            ext_lower = ext.lower()
            category = mapping.get(ext_lower, "OUTROS")
            sub = ext.upper()[1:] if len(ext) > 1 else "OUTROS"
            target_dir = root / category / sub
            target_dir.mkdir(parents=True, exist_ok=True)
            dest = unique_destination(target_dir, stem, ext)
            rel_to = dest.relative_to(root).as_posix()
            os.rename(entry.path, dest)
            actions.append(
                {"action": "move", "from": entry.name, "to": rel_to}
            )
    return actions
