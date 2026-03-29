use std::fmt::Debug;

use anyhow::Result;
use shaku::Component;
use shaku::Interface;
use tracing::instrument;
use unicode_bidi::BidiInfo;
use unicode_bidi::level::Level;

pub trait IReorderRtl: Debug + Interface {
    fn run(&self, text: &str) -> Result<String>;
}

#[derive(Debug)]
#[derive(Component)]
#[shaku(interface = IReorderRtl)]
pub struct ReorderRtl;

impl IReorderRtl for ReorderRtl {
    #[instrument(skip_all, level = "debug")]
    fn run(&self, text: &str) -> Result<String> {
        let level = Level::ltr();
        let bidi_info = BidiInfo::new(text, Some(level));

        let mut text = String::new();

        for paragraph in &bidi_info.paragraphs {
            let line = paragraph.range.clone();
            let line = bidi_info.reorder_line(paragraph, line);

            text.push_str(&line);
        }

        Ok(text)
    }
}
