use std::{io, str::Utf8Error};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to get system information: {0}.")]
    SystemError(#[from] io::Error),
    #[error("Failed to accquire mutex.")]
    SyncError,
    #[error("Failed to retrive information using procfs: {0}.")]
    ProcfsError(#[from] procfs::ProcError),
    #[error("Failed to decode utf-8 string: {0}.")]
    Utf8Error(#[from] Utf8Error),
    #[error("Failed to find executable binary: {0}.")]
    WhichError(#[from] which::Error),
    #[error("Value is empty")]
    EmptyValue,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
