pub mod add_hints;
pub mod application;
pub mod convert_to_markdown;
pub mod download;
pub mod env;
pub mod gemini;
pub mod log;
pub mod output;
pub mod render_markdown;
pub mod reorder_rtl;
pub mod spinner;
pub mod translate_to_hebrew;

use anyhow::Result;
use shaku::HasComponent;

pub use self::add_hints::AddHints;
pub use self::add_hints::IAddHints;
pub use self::application::Application;
pub use self::application::IApplication;
pub use self::application::Level;
pub use self::convert_to_markdown::ConvertToMarkdown;
pub use self::convert_to_markdown::IConvertToMarkdown;
pub use self::download::Download;
pub use self::download::IDownload;
pub use self::env::Env;
pub use self::env::IEnv;
pub use self::gemini::Gemini;
pub use self::gemini::IGemini;
pub use self::log::ILog;
pub use self::log::Log;
pub use self::output::IOutput;
pub use self::output::Output;
pub use self::render_markdown::IRenderMarkdown;
pub use self::render_markdown::RenderMarkdown;
pub use self::reorder_rtl::IReorderRtl;
pub use self::reorder_rtl::ReorderRtl;
pub use self::spinner::ISpinner;
pub use self::spinner::Spinner;
pub use self::translate_to_hebrew::ITranslateToHebrew;
pub use self::translate_to_hebrew::TranslateToHebrew;

shaku::module! {
    pub Module {
        components = [
            AddHints,
            Application,
            ConvertToMarkdown,
            Download,
            Env,
            Gemini,
            Log,
            Output,
            RenderMarkdown,
            ReorderRtl,
            Spinner,
            TranslateToHebrew,
        ],
        providers = []
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let module = Module::builder().build();

    HasComponent::<dyn IApplication>::resolve(&module).run().await?;

    Ok(())
}
