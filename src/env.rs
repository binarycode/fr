use std::env::args_os;
use std::env::var;
use std::ffi::OsString;
use std::fmt::Debug;

use shaku::Component;
use shaku::Interface;
use tracing::debug;
use tracing::instrument;

pub trait IEnv: Debug + Interface {
    fn args(&self) -> Vec<OsString>;
    fn var(&self, key: &str) -> Option<String>;
}

#[derive(Debug)]
#[derive(Component)]
#[shaku(interface = IEnv)]
pub struct Env;

impl IEnv for Env {
    #[instrument(skip_all, level = "debug")]
    fn args(&self) -> Vec<OsString> {
        let args = args_os().collect();
        debug!(?args);

        args
    }

    #[instrument(skip_all, level = "debug")]
    fn var(&self, key: &str) -> Option<String> {
        var(key).ok()
    }
}
