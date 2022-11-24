use anyhow::Result;
use async_trait::async_trait;
use rustemon::client::RustemonClient;

pub(crate) mod all_pokemon;
pub(crate) mod card;
pub(crate) mod descriptions;
pub(crate) mod locations;
pub(crate) mod moves;
pub(crate) mod pokemon;
pub(crate) mod search;
pub(crate) mod weaknesses;

#[async_trait]
pub(crate) trait Builder<T>
where
    Self: Sized,
{
    async fn build(data: &T, rc: &RustemonClient, lang: &str) -> Result<Self>;
}
