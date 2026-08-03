//! Auto-delete planning and apply (B-20..B-23, ADR-0005).

mod age;
mod apply;
mod mode;
mod plan;

pub use age::{age_days, exceeds_max_age, read_birthtime, read_mtime};
pub use apply::{apply_deletes, ApplyError, TrashSink};
pub use mode::{resolve_delete_mode, DeleteMode};
pub use plan::{plan_deletes, PlanError, PlannedDelete};

#[cfg(test)]
mod tests;
