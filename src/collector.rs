pub mod spigot;
pub mod hangar;
pub mod util;

use url::Url;

pub trait HttpServer {
  async fn new() -> Self;
  fn base_url(&self) -> Url;
}