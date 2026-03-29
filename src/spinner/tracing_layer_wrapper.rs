use tracing::Event;
use tracing::Subscriber;
use tracing_subscriber::Layer;

use super::PROGRESS_BAR;

/// A `tracing_subscriber::Layer` wrapper that suspends the global progress bar
/// before logging events through the wrapped layer.
///
/// This ensures that log output does not interfere with the spinner's display.
pub struct TracingLayerWrapper<L> {
    wrapped_layer: L,
}

impl<L> TracingLayerWrapper<L> {
    /// Creates a new `TracingLayerWrapper` that wraps the provided tracing
    /// layer.
    pub fn new(wrapped_layer: L) -> Self {
        Self { wrapped_layer }
    }
}

impl<S, L> Layer<S> for TracingLayerWrapper<L>
where
    S: Subscriber,
    L: Layer<S>,
{
    /// Handles a tracing event by first suspending the global progress bar and
    /// then delegating the event to the wrapped layer.
    ///
    /// This method is called by the `tracing` subscriber when an event occurs.
    /// It ensures that any output from the wrapped layer (e.g., log messages)
    /// does not overwrite or get mixed with the spinner animation.
    fn on_event(&self, event: &Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let progress_bar = PROGRESS_BAR.lock().unwrap();

        if progress_bar.is_hidden() {
            self.wrapped_layer.on_event(event, ctx);
        } else {
            progress_bar.suspend(|| {
                self.wrapped_layer.on_event(event, ctx);
            });
        }
    }
}
