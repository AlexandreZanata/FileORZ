//! Typed config load/save and legacy migration.

mod io;
mod migrate;
mod schema;

pub use io::{
    atomic_write_json, legacy_backup_path, load_config_file, parse_config_json, ConfigError,
    LoadOutcome, Result,
};
pub use migrate::{looks_legacy, migrate_value};
pub use schema::{defaults, AppConfig, AutoDeleteConfig, CategoryMap, ExtMap, FolderDeleteConfig};

#[cfg(test)]
mod tests;
