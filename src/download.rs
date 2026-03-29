use std::fmt::Debug;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::get;
use shaku::Component;
use shaku::Interface;
use tracing::debug;
use tracing::instrument;
use url::Url;

use crate::ISpinner;

#[async_trait]
pub trait IDownload: Debug + Interface {
    async fn run(&self, url: Url) -> Result<String>;
}

#[derive(Debug)]
#[derive(Component)]
#[shaku(interface = IDownload)]
pub struct Download {
    #[shaku(inject)]
    spinner: Arc<dyn ISpinner>,
}

#[async_trait]
impl IDownload for Download {
    #[instrument(skip_all, level = "debug")]
    async fn run(&self, url: Url) -> Result<String> {
        let url = url.as_str();

        self.spinner.start(&format!("Downloading {url:?}..."));
        let result = get(url).await;
        self.spinner.stop();
        let response = result?;
        debug!(?response);

        let response = response.error_for_status()?;
        debug!(?response);

        let text = response.text().await?;

        Ok(text)
    }
}
