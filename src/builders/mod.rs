use anyhow::Result;
use async_trait::async_trait;
use rustemon::client::RustemonClient;
use serde::Serialize;

pub(crate) mod all_pokemon;
pub(crate) mod card;
pub(crate) mod descriptions;
pub(crate) mod pokemon;
pub(crate) mod search;

#[async_trait]
pub(crate) trait Builder<T>
where
    Self: Sized + Serialize,
{
    async fn build(data: &T, rc: &RustemonClient, lang: &str) -> Result<Self>;
}
