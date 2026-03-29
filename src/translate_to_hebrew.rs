use std::fmt::Debug;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use indoc::formatdoc;
use shaku::Component;
use shaku::Interface;
use tracing::instrument;

use crate::IGemini;
use crate::ISpinner;
use crate::Level;

#[async_trait]
pub trait ITranslateToHebrew: Debug + Interface {
    async fn run(&self, text: &str, level: Level) -> Result<String>;
}

#[derive(Debug)]
#[derive(Component)]
#[shaku(interface = ITranslateToHebrew)]
pub struct TranslateToHebrew {
    #[shaku(inject)]
    gemini: Arc<dyn IGemini>,

    #[shaku(inject)]
    spinner: Arc<dyn ISpinner>,
}

#[async_trait]
impl ITranslateToHebrew for TranslateToHebrew {
    #[instrument(skip_all, level = "debug")]
    async fn run(&self, text: &str, level: Level) -> Result<String> {
        let system_prompt = formatdoc! {r#"
            You are a professional Hebrew translator specializing in the Ulpan {level:?} curriculum.
            Your Goal: Translate the provided Markdown text into Hebrew that is perfectly calibrated for a {level:?} student.
            Vocabulary: Use words found in standard {level:?} textbooks.
            Grammar: Limit yourself to the tenses and structures taught at {level:?}.
            Output: Return ONLY the translated Markdown. No conversational filler.
        "#};

        self.spinner.start(&format!("Translating to hebrew ({level:?})..."));
        let result = self.gemini.run(&system_prompt, text).await;
        self.spinner.stop();
        let text = result?;

        Ok(text)
    }
}
