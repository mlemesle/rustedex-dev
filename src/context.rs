use anyhow::Result;
use handlebars::{handlebars_helper, Handlebars};
use num_format::{Locale, ToFormattedString};
use rustemon::client::RustemonClient;

use crate::builders::weaknesses::DamageMultiplicator;

pub(crate) struct Context<'a> {
    hb: Handlebars<'a>,
    rc: RustemonClient,
    lang: String,
}

const SPLITTER_SRC: &str = include_str!("../scripts/splitter.rhai");

handlebars_helper!(ff32: |number: f32| format!("{:09.3}", number.to_string()));

handlebars_helper!(pretty_i64: |number: i64| number.to_formatted_string(&Locale::fr));

handlebars_helper!(damage_multiplicator_to_f32: |damage_multiplicator: DamageMultiplicator| match damage_multiplicator {
    DamageMultiplicator::Immune => 0.0,
    DamageMultiplicator::Quarter => 0.25,
    DamageMultiplicator::Half => 0.5,
    DamageMultiplicator::Simple => 1.0,
    DamageMultiplicator::Double => 2.0,
    DamageMultiplicator::Quadruple => 4.0,
});

fn init_handlebars() -> Result<Handlebars<'static>> {
    let mut hb = Handlebars::new();
    hb.set_strict_mode(true);

    hb.register_helper("ff32", Box::new(ff32));
    hb.register_helper("pretty_i64", Box::new(pretty_i64));
    hb.register_helper(
        "damage_multiplicator_to_f32",
        Box::new(damage_multiplicator_to_f32),
    );

    hb.register_script_helper("splitter", SPLITTER_SRC)?;

    hb.register_templates_directory(".hbs", "templates/")?;

    Ok(hb)
}

impl<'a> Context<'a> {
    pub fn try_new() -> Result<Self> {
        let hb = init_handlebars()?;
        let rc = RustemonClient::default();
        let lang = "en".to_string();

        Ok(Self { hb, rc, lang })
    }

    pub fn hb(&self) -> &Handlebars<'_> {
        &self.hb
    }

    pub fn rc(&self) -> &RustemonClient {
        &self.rc
    }

    pub fn lang(&self) -> &String {
        &self.lang
    }
}
