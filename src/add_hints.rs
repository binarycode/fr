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
pub trait IAddHints: Debug + Interface {
    async fn run(&self, text: &str, level: Level) -> Result<String>;
}

#[derive(Debug)]
#[derive(Component)]
#[shaku(interface = IAddHints)]
pub struct AddHints {
    #[shaku(inject)]
    gemini: Arc<dyn IGemini>,

    #[shaku(inject)]
    spinner: Arc<dyn ISpinner>,
}

#[async_trait]
impl IAddHints for AddHints {
    #[instrument(skip_all, level = "debug")]
    async fn run(&self, text: &str, level: Level) -> Result<String> {
        let system_prompt = formatdoc! {r#"
            Role: You are an expert Hebrew language instructor and an editor specializing in the Ilya Frank Reading Method.
            Objective: Process the provided Hebrew Markdown text to create an interlinear learning experience for a {level:?} student.
            Instructions:
                Sentence-by-Sentence Processing: For every Hebrew sentence, evaluate the vocabulary and idioms against the {level:?} curriculum.
                Vocabulary Hints: Immediately following a sentence, provide English translations for words or phrases that are likely new or difficult for a student at this level.
                Format: Use the format: (Hebrew Word - English, Hebrew Phrase - English Translation).
                Memory Rule (CRITICAL): Keep track of the words you have already translated. If a word or idiom has appeared and been translated in a previous sentence, do not translate it again. Only translate "new" words as they appear for the first time in the text.
                Visual Hierarchy: Place the translations on the same line as the Hebrew sentence.
                Constraint: Do not change the Hebrew text itself. Maintain all original Markdown headers and structure. Output only the processed Markdown.
        "#};

        self.spinner.start("Adding hints...");
        let result = self.gemini.run(&system_prompt, text).await;
        self.spinner.stop();
        let text = result?;

        Ok(text)
    }
}
