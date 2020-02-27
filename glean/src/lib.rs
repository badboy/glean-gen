// We need to export `once_cell`, so we can use it in generated code and the user of Glean doesn't
// have to have a direct dependency on it (plus: no version mismatches).
pub use once_cell;

pub mod build;
pub mod metrics;
