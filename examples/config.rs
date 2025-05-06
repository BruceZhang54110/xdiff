use anyhow::{Ok, Result};
use xdiff::DiffConfig;
fn main() -> Result<()> {
    let content = include_str!("../fixtures/test.yml");

    let config = DiffConfig::from_yml(content)?;
    println!("{:#?}", config);

    Ok(())

}