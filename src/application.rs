use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use clap::ValueEnum;
use shaku::Component;
use shaku::Interface;
use tracing::debug;
use tracing::instrument;
use url::Url;

use crate::IAddHints;
use crate::IConvertToMarkdown;
use crate::IDownload;
use crate::IEnv;
use crate::ILog;
use crate::IOutput;
use crate::IRenderMarkdown;
use crate::IReorderRtl;
use crate::ITranslateToHebrew;

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(ValueEnum)]
pub enum Level {
    Aleph,
    Bet,
    Gimel,
    Dalet,
    Heh,
}

#[derive(Debug)]
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = Level::Gimel)]
    level: Level,

    #[arg(short, long, default_value_t = 80)]
    width: u16,

    #[arg(required = true)]
    url: Url,
}

#[async_trait]
pub trait IApplication: Interface {
    async fn run(&self) -> Result<()>;
}

#[derive(Debug)]
#[derive(Component)]
#[shaku(interface = IApplication)]
pub struct Application {
    #[shaku(inject)]
    add_hints: Arc<dyn IAddHints>,

    #[shaku(inject)]
    convert_to_markdown: Arc<dyn IConvertToMarkdown>,

    #[shaku(inject)]
    download: Arc<dyn IDownload>,

    #[shaku(inject)]
    env: Arc<dyn IEnv>,

    #[shaku(inject)]
    log: Arc<dyn ILog>,

    #[shaku(inject)]
    output: Arc<dyn IOutput>,

    #[shaku(inject)]
    reorder_rtl: Arc<dyn IReorderRtl>,

    #[shaku(inject)]
    render_markdown: Arc<dyn IRenderMarkdown>,

    #[shaku(inject)]
    translate_to_hebrew: Arc<dyn ITranslateToHebrew>,
}

#[async_trait]
impl IApplication for Application {
    #[instrument(skip_all, level = "debug")]
    async fn run(&self) -> Result<()> {
        self.log.init();

        let args = self.env.args();
        let args = Args::parse_from(args);
        debug!(?args);

        let text = self.download.run(args.url).await?;
        let text = self.convert_to_markdown.run(&text)?;
        let text = self.translate_to_hebrew.run(&text, args.level).await?;
        let text = self.add_hints.run(&text, args.level).await?;
        let text = self.reorder_rtl.run(&text)?;
        let text = self.render_markdown.run(&text, args.width)?;

        let mut stdout = self.output.stdout();
        writeln!(stdout, "{text}")?;

        Ok(())
    }
}
