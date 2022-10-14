use anyhow::Result;
use handlebars::{handlebars_helper, Handlebars};
use num_format::{Locale, ToFormattedString};

handlebars_helper!(ff32: |number: f32| format!("{:09.3}", number.to_string()));

handlebars_helper!(pretty_i64: |number: i64| number.to_formatted_string(&Locale::fr));

pub fn init_handlebars() -> Result<Handlebars<'static>> {
    let mut hb = Handlebars::new();
    hb.set_strict_mode(true);

    hb.register_helper("ff32", Box::new(ff32));
    hb.register_helper("pretty_i64", Box::new(pretty_i64));

    hb.register_templates_directory(".hbs", "templates/")?;

    Ok(hb)
}
