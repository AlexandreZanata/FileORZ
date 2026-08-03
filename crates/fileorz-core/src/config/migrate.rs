//! Legacy Portuguese / mixed JSON → stable [`AppConfig`].

use crate::config::schema::{
    defaults, AppConfig, AutoDeleteConfig, CategoryMap, ExtMap, FolderDeleteConfig,
};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

const LEGACY_META: &[&str] = &[
    "Folder",
    "timeverification",
    "Startup",
    "AutoDelete",
    "Enviar Para Lixeira",
    "Excluir permanentemente",
    "AdvancedOrganize",
    "AutoDeleteConfig",
    "folder_delete",
    "_comment",
];

const PLACEHOLDER_FOLDER: &str = "pasta de organização";

/// True when the document still uses legacy top-level keys.
#[must_use]
pub fn looks_legacy(value: &Value) -> bool {
    let Some(obj) = value.as_object() else {
        return false;
    };
    obj.contains_key("Folder")
        || obj.contains_key("timeverification")
        || obj.contains_key("AutoDeleteConfig")
}

/// Convert a legacy or mixed JSON value into stable [`AppConfig`].
pub fn migrate_value(value: Value) -> Result<AppConfig, String> {
    if !looks_legacy(&value) {
        return serde_json::from_value(value).map_err(|e| e.to_string());
    }
    let obj = value
        .as_object()
        .ok_or_else(|| "config root must be an object".to_string())?;
    Ok(AppConfig {
        folder: migrate_folder(obj),
        interval_minutes: parse_u32(obj.get("timeverification"), 5),
        autostart: as_bool(obj.get("Startup"), false),
        locale: "en".into(),
        advanced_organize: as_bool(obj.get("AdvancedOrganize"), false),
        auto_delete: migrate_auto_delete(obj),
        folder_delete: migrate_folder_delete(obj),
        categories: migrate_categories(obj),
    })
}

fn migrate_folder(obj: &Map<String, Value>) -> Option<String> {
    match obj.get("Folder").and_then(Value::as_str) {
        None => None,
        Some(s) if s.is_empty() || s == PLACEHOLDER_FOLDER => None,
        Some(s) => Some(s.to_string()),
    }
}

fn migrate_auto_delete(obj: &Map<String, Value>) -> AutoDeleteConfig {
    let nested = obj
        .get("AutoDeleteConfig")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    AutoDeleteConfig {
        enabled: as_bool(obj.get("AutoDelete"), false),
        to_trash: as_bool(obj.get("Enviar Para Lixeira"), false),
        permanent: as_bool(obj.get("Excluir permanentemente"), false),
        by_created: as_bool(nested.get("Por Data de Criação"), false),
        by_modified: as_bool(nested.get("Por Data de Modificação"), false),
        max_age_days: parse_u32(nested.get("Dias para Auto Deletar"), 15),
    }
}

fn migrate_folder_delete(obj: &Map<String, Value>) -> FolderDeleteConfig {
    let nested = obj
        .get("folder_delete")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    if nested.is_empty() {
        return defaults().folder_delete;
    }
    FolderDeleteConfig {
        enabled: as_bool(nested.get("ativado"), false),
        to_trash: as_bool(nested.get("lixeira"), false),
        permanent: as_bool(nested.get("excluir_permanentemente"), false),
        orz_folders_only: as_bool(nested.get("pastas_ORZ"), false),
        everything: as_bool(nested.get("tudo"), false),
    }
}

fn migrate_categories(obj: &Map<String, Value>) -> CategoryMap {
    let mut out = BTreeMap::new();
    for (key, val) in obj {
        if LEGACY_META.contains(&key.as_str()) {
            continue;
        }
        let Some(map) = val.as_object() else {
            continue;
        };
        if !map_looks_like_exts(map) {
            continue;
        }
        out.insert(key.clone(), ext_map_from(map));
    }
    out
}

fn map_looks_like_exts(map: &Map<String, Value>) -> bool {
    !map.is_empty() && map.keys().all(|k| k.starts_with('.'))
}

fn ext_map_from(map: &Map<String, Value>) -> ExtMap {
    map.iter()
        .filter_map(|(k, v)| v.as_bool().map(|b| (k.clone(), b)))
        .collect()
}

fn as_bool(v: Option<&Value>, default: bool) -> bool {
    v.and_then(Value::as_bool).unwrap_or(default)
}

fn parse_u32(v: Option<&Value>, default: u32) -> u32 {
    match v {
        Some(Value::Number(n)) => n.as_u64().map_or(default, |x| x as u32),
        Some(Value::String(s)) => s.parse().unwrap_or(default),
        _ => default,
    }
}
