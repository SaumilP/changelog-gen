use handlebars::Handlebars;
use anyhow::Result;
use std::fs;

pub fn render(template_path: Option<&str>, data: &serde_json::Value) -> Result<String> {
    let mut hb = Handlebars::new();

    let template = if let Some(path) = template_path {
        fs::read_to_string(path)?
    } else {
        include_str!("../../templates/default.hbs").to_string()
    };

    hb.register_template_string("tpl", template)?;
    Ok(hb.render("tpl", data)?)
}
