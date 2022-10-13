use anyhow::Result;
use handlebars::{handlebars_helper, Handlebars};

handlebars_helper!(ff32: |number: f32| format!("{:09.3}", number.to_string()));

pub fn init_handlebars() -> Result<Handlebars<'static>> {
    let mut hb = Handlebars::new();
    hb.set_strict_mode(true);

    hb.register_helper("ff32", Box::new(ff32));
    hb.register_templates_directory(".hbs", "templates/")?;

    Ok(hb)
}
