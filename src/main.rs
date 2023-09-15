use std::{collections::HashMap, env};

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
struct Point {
  端口: u16,
  参: String,
  静: HashMap<String, u32>,
  动: HashMap<String, u32>,
}

fn main() -> Result<()> {
  let mut conf = env::current_dir()?;
  conf.push("conf.yml");
  println!("Hello, world! {:?}", conf);
  Ok(())
}
