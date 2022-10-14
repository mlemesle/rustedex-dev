use anyhow::Result;
use async_trait::async_trait;
use rustemon::client::RustemonClient;
use serde::Serialize;

use super::{card::Card, Builder};

#[derive(Serialize)]
pub(crate) struct Pokemon {
    card: Card,
}

#[async_trait]
impl Builder<String> for Pokemon {
    async fn build(id: String, rc: &RustemonClient) -> Result<Self> {
        Ok(Pokemon {
            card: Card::build(id, rc).await?,
        })
    }
}
