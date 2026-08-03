"""Shared constants for characterization oracle (catalog B-10..B-23)."""

from __future__ import annotations

IGNORED_CONFIG_KEYS = frozenset(
    {
        "Folder",
        "timeverification",
        "Startup",
        "AutoDelete",
        "Enviar Para Lixeira",
        "Excluir permanentemente",
        "AutoDeleteConfig",
        "AdvancedOrganize",
        "folder_delete",
    }
)
