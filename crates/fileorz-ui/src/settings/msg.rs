//! Settings-specific iced messages.

/// Events from settings hub / editors.
#[derive(Debug, Clone)]
pub enum SettingsMsg {
    /// Open extensions editor.
    OpenExtensions,
    /// Open advanced keywords editor.
    OpenAdvanced,
    /// Open auto-delete editor.
    OpenAutoDelete,
    /// Toggle one extension.
    ExtToggle {
        category: String,
        ext: String,
        enabled: bool,
    },
    /// Bulk enable/disable for a category.
    ExtSetAll { category: String, enabled: bool },
    /// Apply extensions to config.json.
    ExtSave,
    /// Advanced organize master toggle (autosave).
    AdvEnabled(bool),
    /// Append empty keyword row.
    AdvAddGroup,
    /// Delete keyword row.
    AdvDeleteGroup(usize),
    /// Edit group name.
    AdvName(usize, String),
    /// Edit comma-separated phrases.
    AdvPhrases(usize, String),
    /// Persist all keyword rows to keywords.json.
    AdvSaveGroup(usize),
    /// Auto-delete master (autosave).
    AdEnabled(bool),
    /// Age filter: created (mutex).
    AdByCreated,
    /// Age filter: modified (mutex).
    AdByModified,
    /// Deadline days (autosave).
    AdDays(u32),
    /// Destination trash (mutex, autosave).
    AdTrash,
    /// Destination permanent (mutex, autosave).
    AdPermanent,
}
