"""Auto-delete replay — utils/AutoDelete.py age rules (mtime path)."""

from __future__ import annotations

import os
from datetime import datetime, timezone
from pathlib import Path

from . import IGNORED_CONFIG_KEYS


def category_keys(config: dict) -> list[str]:
    return [k for k in config if k not in IGNORED_CONFIG_KEYS and isinstance(config[k], dict)]


def age_days(path: Path) -> int:
    mtime = datetime.fromtimestamp(path.stat().st_mtime, tz=timezone.utc)
    now = datetime.now(tz=timezone.utc)
    return (now - mtime).days


def scan_subfolder(root: Path, category: str, sub: str, days: int) -> list[dict]:
    folder = root / category / sub
    if not folder.is_dir():
        return []
    actions: list[dict] = []
    with os.scandir(folder) as entries:
        for entry in entries:
            if not entry.is_file():
                continue
            if age_days(Path(entry.path)) <= days:
                continue
            rel = Path(category, sub, entry.name).as_posix()
            os.remove(entry.path)
            actions.append(
                {"action": "delete", "from": rel, "mode": "permanent"}
            )
    return actions


def auto_delete_permanent_mtime(root: Path, config: dict) -> list[dict]:
    if not config.get("AutoDelete"):
        return []
    if not config.get("Excluir permanentemente"):
        raise ValueError("characterization aged-files expects permanent delete")
    cfg = config.get("AutoDeleteConfig", {})
    if not cfg.get("Por Data de Modificação"):
        return []
    days = int(cfg.get("Dias para Auto Deletar", "0"))
    actions: list[dict] = []
    for key in category_keys(config):
        # Linux-normalized: match organize Capitalize folders (Win case-fold).
        cat_dir = root / key.capitalize()
        if not cat_dir.is_dir():
            continue
        for sub in os.listdir(cat_dir):
            sub_norm = sub.upper().replace(".", "")
            actions.extend(scan_subfolder(root, key.capitalize(), sub_norm, days))
    return actions
