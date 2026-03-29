use std::fmt::Debug;

use anyhow::Result;
use shaku::Component;
use shaku::Interface;
use termimad::Alignment;
use termimad::Area;
use termimad::MadSkin;
use termimad::crossterm::style::Color::*;
use tracing::debug;
use tracing::instrument;

pub trait IRenderMarkdown: Debug + Interface {
    fn run(&self, text: &str, width: u16) -> Result<String>;
}

#[derive(Debug)]
#[derive(Component)]
#[shaku(interface = IRenderMarkdown)]
pub struct RenderMarkdown;

impl IRenderMarkdown for RenderMarkdown {
    #[instrument(skip_all, level = "debug")]
    fn run(&self, text: &str, width: u16) -> Result<String> {
        let mut skin = MadSkin::default();
        skin.set_headers_fg(Yellow);
        skin.headers[1].align = Alignment::Center;
        skin.headers[2].align = Alignment::Center;
        skin.headers[3].align = Alignment::Center;
        skin.headers[4].compound_style.set_fg(DarkGreen);
        skin.headers[5].compound_style.set_fg(Magenta);
        skin.headers[6].compound_style.set_fg(Cyan);
        skin.headers[7].compound_style.set_fg(Red);
        skin.bold.set_fg(DarkBlue);
        skin.italic.set_fg(Red);
        skin.quote_mark.set_fg(DarkBlue);
        skin.table.set_fg(DarkGrey);
        skin.table.align = Alignment::Center;
        skin.horizontal_rule.set_fg(DarkGreen);
        skin.inline_code.set_fgbg(Cyan, Reset);
        debug!(?skin);

        let mut area = Area::full_screen();
        area.pad_for_max_width(width);
        debug!(?area);

        let text = skin.area_text(text, &area).to_string();

        Ok(text)
    }
}
