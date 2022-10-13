use anyhow::Result;
use handlebars::Handlebars;

pub fn init_handlebars() -> Result<Handlebars<'static>> {
    let mut hb = Handlebars::new();
    hb.set_strict_mode(true);

    hb.register_templates_directory(".hbs", "templates/")?;

    Ok(hb)
}
