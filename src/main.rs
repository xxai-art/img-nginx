use std::{
  collections::HashMap,
  env,
  fs::{read_to_string, File},
  io::Write,
  path::Path,
};

use anyhow::Result;
use futures::future::join_all;
use serde::Deserialize;
use thiserror::Error;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};

mod ip_bin;
use ip_bin::ip_bin;

mod sh;
use sh::sh;

#[derive(Error, Debug)]
pub enum Error {
  #[error("img hash not match")]
  ImgHashNotMatch,
}

#[derive(Debug, PartialEq, Deserialize)]
struct 配置 {
  端口: u16,
  写: String,
  参: String,
  静: HashMap<String, u32>,
  动: HashMap<String, u32>,
}

const WARIN_TIMEOUT: u64 = 18 * 60 * 60;

const IMG_HASH: [u8; 32] = [
  0x73, 0x6d, 0xc7, 0x09, 0xa1, 0x7d, 0xe0, 0x4b, 0xa8, 0x91, 0x43, 0xa4, 0x1b, 0x8c, 0xbc, 0x6a,
  0xd1, 0xfd, 0x06, 0xfe, 0xbc, 0x7b, 0xb3, 0xcb, 0x30, 0x6b, 0x1d, 0x17, 0x31, 0xa6, 0x4e, 0xff,
];

async fn ping(host: &str, port: u16) -> Result<()> {
  let url = format!("http://{host}:{port}/1");
  let client = reqwest::ClientBuilder::new()
    .timeout(Duration::from_secs(10))
    .build()?;

  let bin = client.get(url).send().await?.bytes().await?;
  let hash = blake3::hash(&bin);
  if hash.as_bytes() == &IMG_HASH[..] {
    return Ok(());
  }

  Err(Error::ImgHashNotMatch.into())
}

async fn gen(写: impl AsRef<Path>, down: &HashMap<Vec<u8>, u64>, conf: &配置) -> Result<()> {
  let mut out = Vec::with_capacity(conf.动.len() + conf.静.len() - down.len());
  macro_rules! push {
    ($host_str:ident, $weight:ident) => {
      out.push(format!(
        "server {}:{} weight={} {};",
        $host_str, conf.端口, $weight, conf.参
      ))
    };
  }
  for (host_str, weight) in &conf.动 {
    let host = ip_bin(host_str).unwrap();
    if !down.contains_key(&host) {
      push!(host_str, weight);
    } else {
      out.push(format!("# {host_str}"));
    }
  }
  conf.静.iter().for_each(|(h, w)| push!(h, w));
  File::create(写)?.write_all(out.join("\n").as_bytes())?;
  let cmd = "nginx -s reload";
  match sh(&cmd) {
    Ok(s) => {
      if s.is_empty() {
        info!("{cmd}")
      } else {
        info!("{cmd} → {s}")
      }
    }
    Err(err) => warn!("{}", err),
  }
  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  loginit::init();
  let pwd = env::current_dir()?;
  let conf: 配置 = serde_yaml::from_str(&read_to_string(pwd.join("conf.yml"))?)?;

  let 写 = if !conf.写.starts_with('/') {
    pwd.join(&conf.写)
  } else {
    conf.写.clone().into()
  };

  let mut down = HashMap::with_capacity(conf.动.len());
  for host_str in conf.动.keys() {
    down.insert(ip_bin(host_str).unwrap(), 0);
  }

  let mut change = true;
  loop {
    let mut await_li = Vec::with_capacity(conf.动.len());
    for host in conf.动.keys() {
      await_li.push(ping(host, conf.端口));
    }

    for (host_str, result) in conf.动.keys().zip(join_all(await_li).await) {
      let host = ip_bin(host_str).unwrap();
      let pre_sec = down.get(&host);
      if let Err(err) = result {
        let now = sts::sec();
        if let Some(pre_sec) = pre_sec {
          if (now - pre_sec) > WARIN_TIMEOUT {
            down.insert(host, now);
            warn!("{}", err);
            xerr::log!(xwarn::send(format!("img {err}")).await);
          }
        } else {
          warn!("{}", err);
          down.insert(host, now);
          change = true;
        }
      } else if pre_sec.is_some() {
        change = true;
        down.remove(&host);
      }
    }
    if change {
      xerr::log!(gen(&写, &down, &conf).await);
      change = false;
    }
    sleep(Duration::from_secs(60)).await;
  }
}
