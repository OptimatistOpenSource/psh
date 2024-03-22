#![allow(unused_imports)]

mod config;
mod cpu;
mod event;
mod event_scope;
mod extra_config;
mod process;
mod stat;

pub use config::*;
pub use cpu::*;
pub use event::*;
pub use event_scope::*;
pub use extra_config::*;
pub use process::*;
pub use stat::*;
use thiserror::Error;

#[repr(transparent)]
pub struct Wrap<T>(T);

impl<T> Wrap<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Option is unsupported: {0}")]
    UnsupportedOption(String),
}
