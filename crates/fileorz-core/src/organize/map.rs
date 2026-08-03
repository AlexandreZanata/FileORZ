//! Extension → capitalized category map from [`AppConfig`].

use crate::config::{AppConfig, CategoryMap};
use std::collections::BTreeMap;

/// Python `str.capitalize()`: first char upper, remainder lower.
#[must_use]
pub fn capitalize_category(id: &str) -> String {
    let mut chars = id.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let mut out: String = first.to_uppercase().collect();
            out.push_str(&chars.as_str().to_lowercase());
            out
        }
    }
}

/// Map `.ext` (lowercase, with dot) → category folder name.
#[must_use]
pub fn build_extension_map(categories: &CategoryMap) -> BTreeMap<String, String> {
    let mut mapping = BTreeMap::new();
    for (category, exts) in categories {
        let cat_name = capitalize_category(category);
        for (ext, enabled) in exts {
            if !*enabled {
                continue;
            }
            let clean = normalize_ext(ext);
            mapping.insert(clean, cat_name.clone());
        }
    }
    mapping
}

#[must_use]
pub fn extension_map_from_config(config: &AppConfig) -> BTreeMap<String, String> {
    build_extension_map(&config.categories)
}

fn normalize_ext(ext: &str) -> String {
    let trimmed = ext.trim().to_lowercase();
    if trimmed.starts_with('.') {
        trimmed
    } else {
        format!(".{trimmed}")
    }
}

#[cfg(test)]
mod map_tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn capitalize_matches_python() {
        assert_eq!(capitalize_category("documentos"), "Documentos");
        assert_eq!(capitalize_category("Desenvolvimento"), "Desenvolvimento");
        assert_eq!(capitalize_category("IMAGENS"), "Imagens");
    }

    #[test]
    fn disabled_ext_not_in_map() {
        let mut categorias = CategoryMap::new();
        let mut imgs = BTreeMap::new();
        imgs.insert(".png".into(), true);
        imgs.insert(".jpg".into(), false);
        categorias.insert("imagens".into(), imgs);
        let map = build_extension_map(&categorias);
        assert_eq!(map.get(".png").map(String::as_str), Some("Imagens"));
        assert!(!map.contains_key(".jpg"));
    }
}
