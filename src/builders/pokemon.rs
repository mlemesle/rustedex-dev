use anyhow::Result;
use async_trait::async_trait;
use rustemon::client::RustemonClient;
use serde::Serialize;

use super::{card::Card, descriptions::Descriptions, Builder};

#[derive(Serialize)]
pub(crate) struct Pokemon {
    card: Card,
    descriptions: Descriptions,
}

#[async_trait]
impl Builder<String> for Pokemon {
    async fn build(id: String, rc: &RustemonClient, lang: &str) -> Result<Self> {
        Ok(Pokemon {
            card: Card::build(id.clone(), rc, lang).await?,
            descriptions: Descriptions::build(id, rc, lang).await?,
        })
    }
}
