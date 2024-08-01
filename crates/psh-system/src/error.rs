use std::{io, str::Utf8Error};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to get system information: {0}.")]
    System(#[from] io::Error),
    #[error("Failed to accquire mutex.")]
    Sync,
    #[error("Failed to retrive information using procfs: {0}.")]
    Procfs(#[from] procfs::ProcError),
    #[error("Failed to decode utf-8 string: {0}.")]
    Utf8(#[from] Utf8Error),
    #[error("Failed to find executable binary: {0}.")]
    Which(#[from] which::Error),
    #[error("Invalid Cpu Mask: {0}")]
    InvalidCpuMask(String),
    #[error("Value is empty")]
    EmptyValue,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
