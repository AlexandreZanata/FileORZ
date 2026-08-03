//! Settings Fluent strings.

use fileorz_i18n::Localization;
use std::collections::HashMap;

/// Localized settings copy.
#[derive(Debug, Clone)]
pub struct SettingsStrings {
    pub hub_window: String,
    pub hub_title: String,
    pub hub_subtitle: String,
    pub card_ext_title: String,
    pub card_ext_body: String,
    pub card_adv_title: String,
    pub card_adv_body: String,
    pub card_ad_title: String,
    pub card_ad_body: String,
    pub configure: String,
    pub back: String,
    pub ext_title: String,
    pub ext_subtitle: String,
    pub ext_all: String,
    pub ext_none: String,
    pub ext_save: String,
    pub ext_saved: String,
    pub adv_window: String,
    pub adv_title: String,
    pub adv_on: String,
    pub adv_off: String,
    pub adv_add: String,
    pub adv_help: String,
    pub adv_empty: String,
    pub adv_name_ph: String,
    pub adv_kw_ph: String,
    pub adv_save: String,
    pub adv_delete: String,
    pub ad_window: String,
    pub ad_title: String,
    pub ad_on: String,
    pub ad_off: String,
    pub ad_filters: String,
    pub ad_by_created: String,
    pub ad_by_modified: String,
    pub ad_deadline: String,
    pub ad_deadline_help: String,
    pub ad_type: String,
    pub ad_trash: String,
    pub ad_permanent: String,
    pub categories: HashMap<String, String>,
}

impl SettingsStrings {
    /// Resolve settings IDs from an embedded catalog.
    #[must_use]
    pub fn from_localization(loc: &Localization) -> Self {
        let mut categories = HashMap::new();
        for id in [
            "documentos",
            "imagens",
            "audios",
            "videos",
            "compactos",
            "fontes",
            "setups",
            "desenvolvimento",
            "outros",
        ] {
            categories.insert(id.to_string(), loc.message(&format!("category-{id}")));
        }
        Self {
            hub_window: loc.message("settings-hub-window-title"),
            hub_title: loc.message("settings-hub-title"),
            hub_subtitle: loc.message("settings-hub-subtitle"),
            card_ext_title: loc.message("settings-card-extensions-title"),
            card_ext_body: loc.message("settings-card-extensions-body"),
            card_adv_title: loc.message("settings-card-advanced-title"),
            card_adv_body: loc.message("settings-card-advanced-body"),
            card_ad_title: loc.message("settings-card-autodelete-title"),
            card_ad_body: loc.message("settings-card-autodelete-body"),
            configure: loc.message("settings-card-configure"),
            back: loc.message("settings-back"),
            ext_title: loc.message("settings-ext-title"),
            ext_subtitle: loc.message("settings-ext-subtitle"),
            ext_all: loc.message("settings-ext-select-all"),
            ext_none: loc.message("settings-ext-select-none"),
            ext_save: loc.message("settings-ext-save"),
            ext_saved: loc.message("settings-ext-saved"),
            adv_window: loc.message("settings-advanced-window-title"),
            adv_title: loc.message("settings-advanced-title"),
            adv_on: loc.message("settings-advanced-enabled-on"),
            adv_off: loc.message("settings-advanced-enabled-off"),
            adv_add: loc.message("settings-advanced-add-group"),
            adv_help: loc.message("settings-advanced-help"),
            adv_empty: loc.message("settings-advanced-empty"),
            adv_name_ph: loc.message("settings-advanced-group-name-placeholder"),
            adv_kw_ph: loc.message("settings-advanced-keywords-placeholder"),
            adv_save: loc.message("settings-advanced-save-group"),
            adv_delete: loc.message("settings-advanced-delete-group"),
            ad_window: loc.message("settings-autodelete-window-title"),
            ad_title: loc.message("settings-autodelete-title"),
            ad_on: loc.message("settings-autodelete-enabled-on"),
            ad_off: loc.message("settings-autodelete-enabled-off"),
            ad_filters: loc.message("settings-autodelete-filters-title"),
            ad_by_created: loc.message("settings-autodelete-filter-by-created"),
            ad_by_modified: loc.message("settings-autodelete-filter-by-modified"),
            ad_deadline: loc.message("settings-autodelete-deadline-title"),
            ad_deadline_help: loc.message("settings-autodelete-deadline-help"),
            ad_type: loc.message("settings-autodelete-type-title"),
            ad_trash: loc.message("settings-autodelete-to-trash"),
            ad_permanent: loc.message("settings-autodelete-permanent"),
            categories,
        }
    }

    /// Display label for a category folder id.
    #[must_use]
    pub fn category_label(&self, id: &str) -> String {
        self.categories
            .get(&id.to_lowercase())
            .cloned()
            .unwrap_or_else(|| id.to_string())
    }
}
