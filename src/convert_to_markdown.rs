use std::fmt::Debug;

use anyhow::Context;
use anyhow::Result;
use htmd::HtmlToMarkdown;
use indoc::formatdoc;
use readabilityrs::Readability;
use shaku::Component;
use shaku::Interface;
use tracing::instrument;

pub trait IConvertToMarkdown: Debug + Interface {
    fn run(&self, text: &str) -> Result<String>;
}

#[derive(Debug)]
#[derive(Component)]
#[shaku(interface = IConvertToMarkdown)]
pub struct ConvertToMarkdown;

impl IConvertToMarkdown for ConvertToMarkdown {
    #[instrument(skip_all, level = "debug")]
    fn run(&self, text: &str) -> Result<String> {
        let readability = Readability::new(text, None, None)?;
        let article = readability.parse().context("Failed to parse readable content")?;

        let mut text = article.content.context("Failed to find readable content")?;
        if let Some(title) = article.title {
            text = formatdoc! {r#"
                # {title}

                {text}
            "#};
        }

        let text = HtmlToMarkdown::builder()
            .skip_tags(vec!["img", "video", "audio", "embed"])
            .build()
            .convert(&text)?;

        Ok(text)
    }
}
