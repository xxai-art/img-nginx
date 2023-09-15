use std::{collections::HashMap, env, fs::read_to_string};

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
struct 配置 {
  端口: u16,
  参: String,
  静: HashMap<String, u32>,
  动: HashMap<String, u32>,
}

fn main() -> Result<()> {
  let mut 配置文件 = env::current_dir()?;
  配置文件.push("conf.yml");
  let conf = read_to_string(配置文件);
  dbg!(conf);
  Ok(())
}
