//! Keyword draft rows + CRUD helpers.

use fileorz_core::advanced_pdf::KeywordGroups;

/// Editable keyword group row (name + comma-separated phrases).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordRow {
    pub name: String,
    pub phrases: String,
}

/// Load disk groups into editable rows.
#[must_use]
pub fn rows_from_groups(groups: &KeywordGroups) -> Vec<KeywordRow> {
    groups
        .iter()
        .map(|(name, phrases)| KeywordRow {
            name: name.clone(),
            phrases: phrases.join(", "),
        })
        .collect()
}

/// Build KeywordGroups from rows (skips empty names; trims phrases).
#[must_use]
pub fn groups_from_rows(rows: &[KeywordRow]) -> KeywordGroups {
    let mut out = KeywordGroups::new();
    for row in rows {
        let name = row.name.trim();
        if name.is_empty() {
            continue;
        }
        let phrases: Vec<String> = row
            .phrases
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();
        out.insert(name.to_string(), phrases);
    }
    out
}

/// Append an empty draft row.
pub fn add_row(rows: &mut Vec<KeywordRow>) {
    rows.push(KeywordRow {
        name: String::new(),
        phrases: String::new(),
    });
}

/// Remove a row by index if in range.
pub fn remove_row(rows: &mut Vec<KeywordRow>, index: usize) {
    if index < rows.len() {
        rows.remove(index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_rows_and_groups() {
        let mut groups = KeywordGroups::new();
        groups.insert("Invoices".into(), vec!["nota".into(), "boleto".into()]);
        let rows = rows_from_groups(&groups);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].phrases, "nota, boleto");
        let back = groups_from_rows(&rows);
        assert_eq!(back, groups);
    }

    #[test]
    fn add_and_remove_row() {
        let mut rows = Vec::new();
        add_row(&mut rows);
        assert_eq!(rows.len(), 1);
        remove_row(&mut rows, 0);
        assert!(rows.is_empty());
    }
}
