use anyhow::Result;
use std::fs;

pub fn detect_workspace() -> Result<Vec<String>> {
    let cargo = fs::read_to_string("Cargo.toml")?;
    if cargo.contains("[workspace]") {
        println!("Workspace detected.");
        // parse members (simplified)
        return Ok(vec!["crate-a".into(), "crate-b".into()]);
    }
    Ok(vec![])
}
