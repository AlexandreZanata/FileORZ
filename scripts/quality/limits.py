"""Hard caps from agent-rules/00-core/size-and-complexity-limits.md."""

MAX_FILE_LINES = 200
MAX_FUNCTION_LINES = 80
MAX_CYCLOMATIC = 10

SOURCE_SUFFIXES = {
    ".kt",
    ".kts",
    ".java",
    ".ts",
    ".tsx",
    ".js",
    ".jsx",
    ".mjs",
    ".cjs",
    ".py",
    ".rs",
}

SKIP_DIR_NAMES = {
    ".git",
    ".local",
    ".gradle",
    "build",
    "node_modules",
    "dist",
    "target",
    "coverage",
    ".venv",
    "venv",
    "__pycache__",
    "agent-rules",
    "agent-harness",
    ".cursor",
}
