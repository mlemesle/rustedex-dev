use std::path::PathBuf;

use anyhow::Result;
use handlebars::JsonValue;

use super::render_to_write;
use crate::context::Context;

pub(super) async fn generate_home_page(mut path: PathBuf, context: &Context<'_>) -> Result<()> {
    path.push("home.html");
    render_to_write(context.hb(), "home", &JsonValue::Null, &path).await
}
