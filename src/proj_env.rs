use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load() -> Result<()> {
    let file = File::open(".env")?;
    let reader = BufReader::new(file);
    for line in reader.lines().flatten() {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() == 2 {
            env::set_var(parts[0], parts[1]);
        }
    }
    Ok(())
}
