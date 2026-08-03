"""Run one golden case: copy tree → temp → pipeline → assert."""

from __future__ import annotations

import json
import os
import shutil
import tempfile
import time
from pathlib import Path

from .autodelete import auto_delete_permanent_mtime
from .extension import organize_extensions
from .fs_assert import (
    assert_absent,
    assert_actions,
    assert_present,
    assert_root_remaining,
)
from .pdf_adv import process_pdfs

FIX_ROOT = Path(__file__).resolve().parents[2] / "tests" / "fixtures"


def load_json(rel: str) -> dict:
    return json.loads((FIX_ROOT / rel).read_text(encoding="utf-8"))


def apply_mtimes(root: Path, mtime_spec: dict) -> None:
    now = time.time()
    for rel, spec in mtime_spec.items():
        stamp = now - float(spec["days_ago"]) * 86400
        os.utime(root / rel, (stamp, stamp))


def run_stage(root: Path, case: dict, stage: str) -> list[dict]:
    config = load_json(case["config"])
    config["Folder"] = str(root)
    if stage == "auto_delete":
        return auto_delete_permanent_mtime(root, config)
    if stage == "advanced_pdf":
        return process_pdfs(root, load_json(case["keywords"]))
    if stage == "extension":
        return organize_extensions(root, config)
    raise ValueError(f"unknown pipeline stage: {stage}")


def run_pipeline(root: Path, case: dict) -> list[dict]:
    actions: list[dict] = []
    for stage in case["pipeline"]:
        actions.extend(run_stage(root, case, stage))
    return actions


def assert_case(root: Path, case: dict, actions: list[dict]) -> None:
    assert_actions(actions, case["expected_actions"])
    optional = (
        ("expected_present", assert_present),
        ("expected_absent", assert_absent),
        ("expected_root_remaining", assert_root_remaining),
    )
    for key, checker in optional:
        if key in case:
            checker(root, case[key])


def run_case(golden_path: Path) -> None:
    case = json.loads(golden_path.read_text(encoding="utf-8"))
    src = FIX_ROOT / case["tree"]
    with tempfile.TemporaryDirectory(prefix="fileorz-char-") as tmp:
        dest = Path(tmp) / "tree"
        shutil.copytree(src, dest)
        if "mtime" in case:
            apply_mtimes(dest, case["mtime"])
        actions = run_pipeline(dest, case)
        assert_case(dest, case, actions)


def all_goldens() -> list[Path]:
    return sorted((FIX_ROOT / "golden").glob("*.json"))
