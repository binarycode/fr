use std::fmt::Debug;

use shaku::Component;
use shaku::Interface;
use tracing::instrument;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry;

use crate::spinner::TracingLayerWrapper;

pub trait ILog: Debug + Interface {
    fn init(&self);
}

#[derive(Debug)]
#[derive(Component)]
#[shaku(interface = ILog)]
pub struct Log;

impl ILog for Log {
    #[instrument(skip_all, level = "debug")]
    fn init(&self) {
        let layer = layer();
        let layer = TracingLayerWrapper::new(layer);
        let layer = layer.with_filter(EnvFilter::from_default_env());

        registry().with(layer).init();
    }
}
