//! Extension-based organize (B-11/B-12) — plan then apply.

mod apply;
mod map;
mod plan;

pub use apply::apply_moves;
pub use map::{build_extension_map, capitalize_category};
pub use plan::{plan_moves, PlannedMove};

#[cfg(test)]
mod tests;
