use crate::collector::spigot::{SpigotAuthor, SpigotClient};

use anyhow::Result;

pub mod collector;

#[tokio::main]
async fn main() -> Result<()> {
    let spigot_client = SpigotClient::new()?;
    let spigot_authors: Vec<SpigotAuthor> = spigot_client.get_spigot_authors().await?;
    println!("{:?}", spigot_authors);

    Ok(())
}
