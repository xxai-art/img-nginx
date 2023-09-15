use std::{
  collections::{HashMap, HashSet},
  env,
  fs::read_to_string,
  path::Path,
};

use anyhow::Result;
use blake3;
use futures::future::join_all;
use serde::Deserialize;
use thiserror::Error;
use tokio::time::{sleep, Duration};
use tracing::warn;

mod ip_bin;
use ip_bin::{bin_ip, ip_bin};

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

const WARIN_TIMEOUT: Duration = Duration::from_secs(12 * 60 * 60);

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

async fn gen(写: impl AsRef<Path>, working: &HashMap<Vec<u8>, u64>, conf: &配置) -> Result<()> {
  // for host in working {}
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

  let mut working = HashMap::with_capacity(conf.动.len());
  for host_str in conf.动.keys() {
    working.insert(ip_bin(host_str).unwrap(), 0);
  }

  let mut change = true;
  loop {
    let mut await_li = Vec::with_capacity(conf.动.len());
    for host in conf.动.keys() {
      await_li.push(ping(&host, conf.端口));
    }

    for (host_str, result) in conf.动.keys().zip(join_all(await_li).await) {
      let host = ip_bin(host_str).unwrap();
      let exist = working.contains_key(&host);
      if let Err(err) = result {
        if exist {
          warn!("{} {:?}", host_str, err);
          working.remove(&host);
          change = true;
        }
      } else {
        if !exist {
          change = true;
          working.insert(host, sts::sec());
        }
      }
      if change {
        xerr::log!(gen(&写, &working, &conf).await);
        change = false;
      }
    }
    sleep(Duration::from_secs(60)).await;
  }
}
