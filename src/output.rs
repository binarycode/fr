use std::fmt::Debug;
use std::io::Write;

use shaku::Component;
use shaku::Interface;

pub trait IOutput: Debug + Interface {
    fn stdout(&self) -> Box<dyn Write>;
    #[cfg(test)]
    fn read(&self) -> String;
}

#[derive(Debug)]
#[derive(Component)]
#[shaku(interface = IOutput)]
pub struct Output;

impl IOutput for Output {
    fn stdout(&self) -> Box<dyn Write> {
        Box::new(std::io::stdout())
    }

    #[cfg(test)]
    fn read(&self) -> String {
        unimplemented!();
    }
}
