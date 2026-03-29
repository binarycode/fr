use std::fmt::Debug;
use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use shaku::Component;
use shaku::Interface;
use tracing::debug;
use tracing::instrument;

use crate::IEnv;

const GEMINI_API_KEY: &str = "GEMINI_API_KEY";

#[async_trait]
pub trait IGemini: Debug + Interface {
    async fn run(&self, system_prompt: &str, prompt: &str) -> Result<String>;
}

#[derive(Debug)]
#[derive(Component)]
#[shaku(interface = IGemini)]
pub struct Gemini {
    #[shaku(inject)]
    env: Arc<dyn IEnv>,
}

#[async_trait]
impl IGemini for Gemini {
    #[instrument(skip_all, level = "debug")]
    async fn run(&self, system_prompt: &str, prompt: &str) -> Result<String> {
        let api_key = self
            .env
            .var(GEMINI_API_KEY)
            .with_context(|| format!("{GEMINI_API_KEY} environment variable is not set"))?;

        let client = gemini_rust::client::Gemini::new(api_key)?;

        let response = client
            .generate_content()
            .with_system_prompt(system_prompt)
            .with_user_message(prompt)
            .execute()
            .await?;
        debug!(?response);

        let text = response.text();
        debug!(?text);

        Ok(text)
    }
}
