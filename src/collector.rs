pub mod hangar;
pub mod modrinth;
pub mod spigot;
pub mod util;

use url::Url;

pub trait HttpServer {
  async fn new() -> Self;
  fn base_url(&self) -> Url;
}