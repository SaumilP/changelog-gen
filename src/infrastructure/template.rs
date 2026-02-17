use handlebars::Handlebars;
use anyhow::Result;
use crate::traits::template::TemplateRenderer;
use crate::domain::commit::Commit;

pub struct HandlebarsRenderer;

#[async_trait::async_trait]
impl TemplateRenderer for HandlebarsRenderer {
    async fn render(&self, commits: Vec<Commit>) -> Result<String> {
        let mut hb = Handlebars::new();
        hb.register_template_string("tpl",
            include_str!("../../templates/default.hbs"))?;

        let data = serde_json::json!({
            "commits": commits
        });

        Ok(hb.render("tpl", &data)?)
    }
}
