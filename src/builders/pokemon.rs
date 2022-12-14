use anyhow::Result;
use async_trait::async_trait;
use rustemon::client::RustemonClient;
use serde::Serialize;

use super::{
    card::Card, descriptions::Descriptions, locations::Locations, moves::Moves,
    weaknesses::Weaknesses, Builder,
};

#[derive(Serialize)]
pub(crate) struct Pokemon {
    card: Card,
    descriptions: Descriptions,
    moves: Moves,
    weaknesses: Weaknesses,
    locations: Locations,
}

#[async_trait]
impl Builder<String> for Pokemon {
    async fn build(id: &String, rc: &RustemonClient, lang: &str) -> Result<Self> {
        Ok(Pokemon {
            card: Card::build(id, rc, lang).await?,
            descriptions: Descriptions::build(id, rc, lang).await?,
            moves: Moves::build(id, rc, lang).await?,
            weaknesses: Weaknesses::build(id, rc, lang).await?,
            locations: Locations::default(),
            // TODO: Uncomment this when data has been merged
            // locations: Locations::build(id, rc, lang).await?,
        })
    }
}
