//! Embedded Fluent catalogs (`locales/{en,pt-BR}/*.ftl`).

pub const LOCALES: &[&str] = &["en", "pt-BR"];
pub const FILES: &[&str] = &["main.ftl", "settings.ftl", "errors.ftl", "tray.ftl"];

/// Return concatenated FTL source for a supported locale, or `None`.
#[must_use]
pub fn embedded_ftl(locale: &str) -> Option<&'static str> {
    match locale {
        "en" => Some(EN),
        "pt-BR" => Some(PT_BR),
        _ => None,
    }
}

const EN: &str = concat!(
    include_str!("../locales/en/main.ftl"),
    "\n",
    include_str!("../locales/en/settings.ftl"),
    "\n",
    include_str!("../locales/en/errors.ftl"),
    "\n",
    include_str!("../locales/en/tray.ftl"),
);

const PT_BR: &str = concat!(
    include_str!("../locales/pt-BR/main.ftl"),
    "\n",
    include_str!("../locales/pt-BR/settings.ftl"),
    "\n",
    include_str!("../locales/pt-BR/errors.ftl"),
    "\n",
    include_str!("../locales/pt-BR/tray.ftl"),
);
