"""Filesystem assertions against golden manifests."""

from __future__ import annotations

from pathlib import Path


def action_key(item: dict) -> tuple:
    return (
        item.get("action"),
        item.get("from"),
        item.get("to"),
        item.get("reason"),
        item.get("mode"),
    )


def assert_actions(actual: list[dict], expected: list[dict]) -> None:
    got = sorted(action_key(a) for a in actual)
    want = sorted(action_key(e) for e in expected)
    if got != want:
        raise AssertionError(f"actions mismatch\n got={got}\nwant={want}")


def assert_present(root: Path, paths: list[str]) -> None:
    for rel in paths:
        if not (root / rel).exists():
            raise AssertionError(f"missing expected path: {rel}")


def assert_absent(root: Path, paths: list[str]) -> None:
    for rel in paths:
        if (root / rel).exists():
            raise AssertionError(f"path should be absent: {rel}")


def assert_root_remaining(root: Path, names: list[str]) -> None:
    files = sorted(
        e.name for e in root.iterdir() if e.is_file()
    )
    if files != sorted(names):
        raise AssertionError(
            f"root files mismatch\n got={files}\nwant={sorted(names)}"
        )
