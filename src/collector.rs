pub mod spigot;
pub mod hangar;
pub mod util;

use url::Url;

pub trait HttpServer {
  // This opaque-hidden-inferred-bound warning should not be fired.
  // It has been fixed in https://github.com/rust-lang/rust/issues/113538
  // TODO: Remove this comment when it has been fixed.
  async fn new() -> Self;
  fn base_url(&self) -> Url;
}