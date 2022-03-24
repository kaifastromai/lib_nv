///A number of re-exported crates that encapsulates dependencies that could be common
/// to multiple novella modules. This avoids making cargo build different versions of
/// the same crate for different modules.
pub extern crate anyhow;
pub extern crate petgraph;
pub extern crate serde;
pub extern crate serde_json;
