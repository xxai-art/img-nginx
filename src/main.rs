use std::{collections::HashMap, env, fs::read_to_string};

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
struct 配置 {
  端口: u16,
  写: String,
  参: String,
  静: HashMap<String, u32>,
  动: HashMap<String, u32>,
}

fn main() -> Result<()> {
  let pwd = env::current_dir()?;
  let conf: 配置 = serde_yaml::from_str(&read_to_string(pwd.join("conf.yml"))?)?;

  let 写 = if !conf.写.starts_with('/') {
    pwd.join(conf.写)
  } else {
    conf.写.into()
  };

  dbg!(写);
  Ok(())
}
