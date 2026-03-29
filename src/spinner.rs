mod tracing_layer_wrapper;

use std::fmt::Debug;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::time::Duration;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use shaku::Component;
use shaku::Interface;
use tracing::instrument;

pub use self::tracing_layer_wrapper::TracingLayerWrapper;

pub static PROGRESS_BAR: LazyLock<Mutex<ProgressBar>> = LazyLock::new(|| {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    progress_bar.finish_and_clear();

    Mutex::new(progress_bar)
});

pub trait ISpinner: Debug + Interface {
    fn start(&self, message: &str);
    fn stop(&self);
}

#[derive(Debug)]
#[derive(Component)]
#[shaku(interface = ISpinner)]
pub struct Spinner;

impl ISpinner for Spinner {
    #[instrument(skip_all, level = "debug")]
    fn start(&self, message: &str) {
        let progress_bar = PROGRESS_BAR.lock().unwrap();
        progress_bar.reset();
        progress_bar.enable_steady_tick(Duration::from_millis(100));
        progress_bar.set_message(message.to_string());
    }

    #[instrument(skip_all, level = "debug")]
    fn stop(&self) {
        let progress_bar = PROGRESS_BAR.lock().unwrap();
        progress_bar.finish_and_clear();
    }
}
