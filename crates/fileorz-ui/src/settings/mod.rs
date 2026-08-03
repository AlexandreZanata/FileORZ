//! Settings hub + editors (phase 15).

pub mod ad_view;
pub mod adv_view;
pub mod chrome;
pub mod ext_logic;
pub mod ext_view;
pub mod hub;
pub mod keywords_logic;
pub mod msg;
pub mod mutex;
pub mod roundtrip;
pub mod screen;
pub mod strings;
pub mod update;

pub use msg::SettingsMsg;
pub use screen::SettingsScreen;
pub use strings::SettingsStrings;
