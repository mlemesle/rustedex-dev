use anyhow::Result;
use async_trait::async_trait;
use rustemon::client::RustemonClient;
use serde::Serialize;

pub(crate) mod card;
pub(crate) mod pokemon;

#[async_trait]
pub(crate) trait Builder
where
    Self: Sized + Serialize,
{
    async fn build(id: &str, rc: &RustemonClient) -> Result<Self>;
}
