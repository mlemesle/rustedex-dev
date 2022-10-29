use anyhow::Result;
use async_trait::async_trait;
use rustemon::client::RustemonClient;
use serde::Serialize;

use super::{card::Card, descriptions::Descriptions, moves::Moves, Builder};

#[derive(Serialize)]
pub(crate) struct Pokemon {
    card: Card,
    descriptions: Descriptions,
    moves: Moves,
}

#[async_trait]
impl Builder<String> for Pokemon {
    async fn build(id: &String, rc: &RustemonClient, lang: &str) -> Result<Self> {
        Ok(Pokemon {
            card: Card::build(id, rc, lang).await?,
            descriptions: Descriptions::build(id, rc, lang).await?,
            moves: Moves::build(id, rc, lang).await?,
        })
    }
}
